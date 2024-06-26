/// Populates an HTTP request message based on provided parameters.
/// RFC document: https://datatracker.ietf.org/doc/html/rfc9110#name-connection
///
///
/// # Arguments
///
/// * `protocol` - The HTTP protocol version (e.g., "HTTP/1.1").
/// * `host` - The host to which the request will be sent (e.g., "www.example.com").
/// * `path` - The path of the resource being requested (e.g., "/example/path").
/// * `method` - Optional HTTP method (default is "GET").
/// * `data` - Optional data to include in the request body.
/// * `headers` - Additional headers to include in the request.
///
/// # Returns
///
/// A string representing the HTTP request message.
///
/// # Example
///
/// ```
/// let request = populate_request(
///     "HTTP/1.1",
///     "www.example.com",
///     "/example/path",
///     "GET",
///     None,
///     vec![
///         "Accept: */*",
///         "Accept-Language: en-US,en;q=0.5",
///         "Accept-Encoding: gzip, deflate",
///         "Connection: keep-alive",
///     ],
/// );
/// ```
pub fn populate_request(
    protocol: &str,
    host: &str,
    path: &str,
    method: &str,
    data: Option<&String>,
    headers: Vec<&str>,
) -> String {
    // placeholder for HTTP messages (RFC9110 Section 3.9)
    let mut message = String::new();

    // request line (RFC9110 Section 2.5)
    message.push_str(&format!("{method} /{path} {protocol}\r\n"));
    // host header (RFC9110 Section 7.2)
    message.push_str(&format!("Host: {host}\r\n"));
    // user-agent header (RFC9110 10.1.5)
    message.push_str(&format!("User-Agent: Curl-rs 1.0"));
    // Accept header (RFC9110 Section 12.5.1)
    message.push_str("Accept: */*\r\n");
    // connection header (RFC9110 Section 7.6.1)
    message.push_str("Connection: close\r\n");

    // check for other HTTP methods (RFC9110 Section 9.3)
    if method == "POST" || method == "PUT" {
        // if there are custom headers
        if headers.len() > 0 {
            // append custom headers (RFC9110 Section 6.3)
            for head in headers {
                message.push_str(head);
            }
            // denote end of headers
            message += "\r\n";
        }
        // if there are no headers
        else {
            // default type for POST and PUT requests (RFC9110 Section 8.3)
            message.push_str("Content-Type: application/json\r\n");
        }
    }

    // if request data is included...
    if let Some(data) = data {
        // convert to bytes
        let data_bytes: &[u8] = data.as_bytes();
        // append content-length header (RFC9110 Section 8.6)
        message.push_str(&format!("Content-Length: {}\r\n\r\n", data_bytes.len()));
        // append request body
        message.push_str(&format!("{data}\r\n"));
    }

    // end of message
    message += "\r\n";
    message
}
