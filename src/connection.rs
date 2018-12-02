use crate::message::Message;
use crate::ws::{Response, WebSocketTransport};
use serde_json::value::Value;

pub struct Connection {
    browser_WS_endpoint: String,
    transport: WebSocketTransport,
    slow_mo: u32,
}

impl Connection {
    pub fn new(
        browser_WS_endpoint: String,
        transport: WebSocketTransport,
        slow_mo: u32,
    ) -> Connection {
        Connection {
            browser_WS_endpoint: browser_WS_endpoint,
            transport: transport,
            slow_mo: slow_mo,
        }
    }

    pub async fn send(&mut self, msg: Message) -> Response {
        await!(self.transport.send(msg))
    }
}
