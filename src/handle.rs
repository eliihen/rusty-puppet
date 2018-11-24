use crate::launcher::{Launcher, LaunchOptions};
use crate::browser::Browser;

pub struct Handle {
    project_root: Option<String>,
    pub launcher: Launcher,
}

impl Handle {
  pub fn new() -> Handle {
      Handle {
          project_root: None,
          launcher: Launcher::new()
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
    use crate::handle::Handle;
    use futures::executor::block_on;

    #[test]
    fn test_construct_no_throw() {
        Handle::new();
    }

    #[test]
    fn test_launch() {
        let handle = Handle::new();
        let browser = block_on(handle.launch());
        block_on(browser.close());
    }
}

