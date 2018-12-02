use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum Method {
    Browser_getVersion,
    Page_navigate,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = format!("{:?}", self).replace("_", ".");
        write!(f, "{}", string)
    }
}

#[derive(Debug)]
pub struct Message {
    method: Method,
    params: Option<HashMap<String, String>>,
}

impl Message {
    pub fn new(method: Method) -> Message {
        Message {
            method: method,
            params: None,
        }
    }

    pub fn new_with_params(method: Method, params: HashMap<String, String>) -> Message {
        Message {
            method: method,
            params: Some(params),
        }
    }

    /// Adds the provided ID to the dataset and serializes the message as JSON
    pub fn serialize(&self, id: u64) -> String {
        let data = if let Some(params) = &self.params {
            json!({
                "id": id,
                "method": format!("{}", self.method),
                "params": self.params
            })
        } else {
            json!({
                "id": id,
                "method": format!("{}", self.method),
            })
        };

        format!("{}", data)
    }
}

#[cfg(test)]
mod test {
    extern crate env_logger;

    use crate::message::{Message, Method};

    #[test]
    fn test_mesage_display() {
        let _ = env_logger::try_init();

        let message = Message {
            method: Method::Browser_getVersion,
            params: None,
        };

        assert_eq!(
            message.serialize(123),
            "{\"id\":123,\"method\":\"Browser.getVersion\"}",
        );

        let message = Message {
            method: Method::Page_navigate,
            params: Some(
                [("url".to_string(), "https://example.com".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            ),
        };

        assert_eq!(
            message.serialize(456),
            "{\"id\":456,\"method\":\"Page.navigate\",\"params\":{\"url\":\"https://example.com\"}}",
        );
    }
}
