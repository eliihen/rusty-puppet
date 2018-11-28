use crate::connection::Connection;
use crate::page::Page;
use std::process::Child;

pub struct Browser {
    pub child_process: Child,
    //pub connection: Connection
}

impl Browser {
    pub fn new(connection: Connection) -> Browser {
        unimplemented!();
    }

    pub fn new_page(&self) -> Page {
        unimplemented!();
    }

    pub async fn close(&mut self) {
        self.child_process.kill();
    }
}
