// TODO Use response from some crate
pub struct Response {}

pub struct PageOptions {}

pub struct Page {}

impl Page {
    pub fn new() -> Page {
        Page {}
    }

    pub fn goto(&self, url: String) -> Response {
        let options = PageOptions {};
        self.goto_with_options(url, options)
    }
    pub fn goto_with_options(&self, url: String, options: PageOptions) -> Response {
        unimplemented!();
    }
}
