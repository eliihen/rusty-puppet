use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use ws::{self, connect, CloseCode, Handler, Handshake, Message, Result};

pub struct Event {}

pub struct WebSocketTransport {
    sender: Sender<Event>,
    receiver: Receiver<Event>,
}

struct Client {
    out: ws::Sender,
    sender: Sender<Event>,
}

impl Handler for Client {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        println!("OPENED");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Echo the message back
        //self.out.send(msg)
        println!("Got a message: {}", &msg);
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        // The WebSocket protocol allows for a utf8 reason for the closing state after the
        // close code. WS-RS will attempt to interpret this data as a utf8 description of the
        // reason for closing the connection. I many cases, `reason` will be an empty string.
        // So, you may not normally want to display `reason` to the user,
        // but let's assume that we know that `reason` is human-readable.
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

impl WebSocketTransport {
    pub fn new(browser_WS_endpoint: String) -> WebSocketTransport {
        let (sender, receiver) = channel();

        let thread_sender = sender.clone();
        thread::spawn(move || {
            connect(browser_WS_endpoint, |out| Client {
                out: out,
                sender: thread_sender.clone(),
            });
        });

        WebSocketTransport {
            receiver: receiver,
            sender: sender,
        }
    }
}
