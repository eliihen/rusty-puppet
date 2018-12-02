use crate::connection::Connection;
use crate::page::Page;
use std::process::Child;

pub struct Browser {
    pub connection: Connection,
    pub child_process: Child,
}

impl Browser {
    pub fn new(connection: Connection, child_process: Child) -> Browser {
        Browser {
            connection: connection,
            child_process: child_process,
        }
    }

    pub fn new_page(&self) -> Page {
        unimplemented!();
    }

    pub async fn close(&mut self) {
        self.child_process.kill();
    }
}
