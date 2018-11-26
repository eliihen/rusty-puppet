use std::process::Child;

pub struct Browser {
    pub child_process: Child
}

impl Browser {
    pub async fn close(&mut self) {
        self.child_process.kill();
    }
}
