use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref PROTOCOL_STRING: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("http", "HTTP/1.1");
        map
    };
}

pub const DEFAULT_PORT: &str = "80";
