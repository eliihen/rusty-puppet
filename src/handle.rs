use crate::browser::Browser;
use crate::launcher::{LaunchOptions, Launcher};

pub struct Handle {
    project_root: Option<String>,
    pub launcher: Launcher,
}

impl Handle {
    pub fn new() -> Handle {
        Handle {
            project_root: None,
            launcher: Launcher::new(),
        }
    }

    pub fn from_root(project_root: String) -> Handle {
        Handle {
            project_root: Some(project_root.clone()),
            launcher: Launcher::from_root(project_root),
        }
    }

    pub async fn launch<'a>(&'a self) -> Browser {
        let options = LaunchOptions::new();
        await!(self.launcher.launch(&options))
    }

    pub async fn launch_with_opts<'a>(&'a self, options: &'a LaunchOptions) -> Browser {
        await!(self.launcher.launch(options))
    }

    //pub fn connect(options) {
    //  return this._launcher.connect(options);
    //}

    //pub fn executablePath() {
    //  return this._launcher.executablePath();
    //}

    //pub fn defaultArgs(options) {
    //  return this._launcher.defaultArgs(options);
    //}

    //pub fn createBrowserFetcher(options) {
    //  return new BrowserFetcher(this._projectRoot, options);
    //}
}

#[cfg(test)]
mod tests {
    extern crate env_logger;

    use crate::handle::Handle;
    use futures::executor::block_on;

    #[test]
    fn test_construct_no_throw() {
        let _ = env_logger::try_init();

        Handle::new();
    }

    #[test]
    fn test_launch() {
        let _ = env_logger::try_init();

        let handle = Handle::new();
        let mut browser = block_on(handle.launch());
        block_on(browser.close());
    }

    #[test]
    #[ignore]
    fn test_page_goto() {
        let _ = env_logger::try_init();

        let handle = Handle::new();
        let mut browser = block_on(handle.launch());
        let page = browser.new_page();
        page.goto("https://example.com".to_string());
        block_on(browser.close());
    }
}
