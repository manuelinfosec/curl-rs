use crate::constants;
use crate::types::URL;

/// Parses a URL string into a URL struct.
///
/// # Arguments
///
/// * `url` - The URL string to parse.
///
/// # Returns
///
/// A URL struct representing the parsed URL.
pub fn parse_url(url: &str) -> URL {
    // split the URL string into protocol and the rest of the URL after "://"
    let (protocol_str, rest) = url.split_once("://").unwrap();
    // split the remaining URL into hostname and pathname after "/"
    let (temp_hostname, pathname) = rest.split_once("/").unwrap();
    // check if the hostname contains a port number
    let (hostname, port) = if temp_hostname.contains(":") {
        // if it does, split it into hostname and port
        temp_hostname.split_once(":").expect("Invalid hostname")
    } else {
        // otherwise, use the default port specified in constants module
        (temp_hostname, constants::DEFAULT_PORT)
    };
    // format the socket address as "hostname:port"
    let socket_addr = format!("{}:{}", hostname, port);
    // retrieve the protocol string from the constants module
    let protocol = constants::PROTOCOL_STRING
        .get(protocol_str)
        .expect("Invalid protocol");

    // create a new URL struct with the parsed components
    URL {
        protocol,
        hostname,
        pathname,
        socket_addr,
    }
}
