//! Contains the WebSocket implementation for communicating with Chrome

use crate::message::Message;
use std::collections::HashMap;
use std::thread;
use std::time;
use futures::channel::{mpsc, oneshot};
use serde_json::value::Value;
use websocket::{self, ClientBuilder, OwnedMessage};

pub enum Event {
    Connected,
}

/// A repesentation of a response from Chrome after a `send()`
pub enum Response {
    Frame(Value),
    Meta(Event),
}

/// The transport that communicates with Chrome
///
/// When instantiated, it will fire up two threads that will respectively read
/// and write frames to chrome over websocket. The `send()` function is the main
/// method for communicating with chrome. It returns a Future that resolves when
/// the response is received.
pub struct WebSocketTransport {
    counter: u64,
    sender: mpsc::UnboundedSender<OwnedMessage>,
    callback_registration: mpsc::UnboundedSender<(u64, oneshot::Sender<Response>)>,
}

impl WebSocketTransport {
    pub fn new(browser_WS_endpoint: String) -> WebSocketTransport {
        info!("Connecting to {}", browser_WS_endpoint);
        let client = ClientBuilder::new(&browser_WS_endpoint)
            .unwrap()
            .connect_insecure()
            .unwrap();
        info!("Connected!");

        let (mut ws_receiver, mut ws_sender) = client.split().unwrap();
        let (register_callback, receive_callback) =
            mpsc::unbounded::<(u64, oneshot::Sender<Response>)>();
        let (message_sender, message_receiver) = mpsc::unbounded::<OwnedMessage>();

        thread::Builder::new()
            .name("websocket_sender".to_string())
            .spawn(WebSocketTransport::build_sender(
                ws_sender,
                message_receiver,
            ))
            .expect("Failed to start websocket sender thread");
        thread::Builder::new()
            .name("websocket_receiver".to_string())
            .spawn(WebSocketTransport::build_receiver(
                ws_receiver,
                message_sender.clone(),
                receive_callback,
            ))
            .expect("Failed to start websocket receiver thread");

        WebSocketTransport {
            counter: 0,
            sender: message_sender,
            callback_registration: register_callback,
        }
    }

    pub async fn send(&mut self, message: Message) -> Response {
        let msg_id = self.generate_id();
        let message = message.serialize(msg_id);
        info!("Sending message {}", message);
        let (sender, receiver) = oneshot::channel::<Response>();
        self.callback_registration.unbounded_send((msg_id, sender));
        self.sender.unbounded_send(OwnedMessage::Text(message));

        await!(receiver).unwrap()
    }

    fn generate_id(&mut self) -> u64 {
        self.counter += 1;
        self.counter
    }

    fn build_sender(
        mut ws_sender: websocket::sender::Writer<std::net::TcpStream>,
        mut receiver: mpsc::UnboundedReceiver<websocket::OwnedMessage>,
    ) -> impl FnOnce() {
        move || {
            debug!("Spawned websocket sender thread");

            loop {
                let message = match receiver.try_next() {
                    Ok(m) => m.unwrap(),
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        return;
                    }
                };
                match message {
                    OwnedMessage::Close(_) => {
                        let _ = ws_sender.send_message(&message);
                        // If it's a close message, just send it and then return.
                        return;
                    }
                    _ => (),
                }
                // Send the message
                match ws_sender.send_message(&message) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        let _ = ws_sender.send_message(&websocket::Message::close());
                        return;
                    }
                }
            }

            debug!("Exiting websocket sender thread");
        }
    }

    fn build_receiver(
        mut ws_receiver: websocket::receiver::Reader<std::net::TcpStream>,
        sender: mpsc::UnboundedSender<websocket::OwnedMessage>,
        mut receive_callback: mpsc::UnboundedReceiver<(u64, oneshot::Sender<Response>)>,
    ) -> impl FnOnce() {
        move || {
            debug!("Spawned websocket reciever thread");
            let mut callbacks = HashMap::new();

            loop {
                if let Some((id, callback)) = receive_callback.try_next().unwrap() {
                    callbacks.insert(id, callback);
                }

                let message = ws_receiver.recv_message();
                let message = match message {
                    Ok(m) => m,
                    Err(e) => {
                        error!("Receive Loop: {:?}", e);
                        let _ = sender.unbounded_send(OwnedMessage::Close(None));
                        return;
                    }
                };
                let message = match message {
                    OwnedMessage::Close(_) => {
                        // Got a close message, so send a close message and return
                        let _ = sender.unbounded_send(OwnedMessage::Close(None));
                        return;
                    }
                    OwnedMessage::Ping(data) => {
                        match sender.unbounded_send(OwnedMessage::Pong(data)) {
                            // Send a pong in response
                            Ok(()) => (),
                            Err(e) => {
                                println!("Receive Loop: {:?}", e);
                                return;
                            }
                        };
                        continue;
                    }
                    OwnedMessage::Text(data) => data,
                    _ => continue,
                };

                let data: Value = match serde_json::from_str(&message) {
                    Ok(data) => data,
                    Err(err) => {
                        warn!("Failed to parse frame {}", err);
                        continue;
                    }
                };

                let id = data["id"].as_u64().unwrap();
                if let Some(callback) = callbacks.remove(&id) {
                    callback.send(Response::Frame(data));
                } else {
                    error!("No callback registered for id {}. Race condition?", id);
                }

                // Sleep to avoid eating the processor
                // thread::sleep(time::Duration::from_millis(10));
                thread::yield_now();
            }

            debug!("Exiting websocket reciever thread");
        }
    }
}
