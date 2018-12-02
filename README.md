# rusty-puppet

A rust library which provides an API to control Chrome over the DevTools
Protocol.

## Running tests

```bash
# Without logging
CHROME_DEVEL_SANDBOX=path_to_sandbox cargo test

# With logging
RUST_LOG=rusty_puppet=info CHROME_DEVEL_SANDBOX=path_to_sandbox cargo test -- --nocapture

# Watching files with cargo-watch
RUST_LOG=rusty_puppet=info CHROME_DEVEL_SANDBOX=path_to_sandbox cargo watch -x 'test -- --nocapture'
```
