/// A parsed URL
pub struct URL<'a> {
    pub protocol: &'a str,
    pub hostname: &'a str,
    pub pathname: &'a str,
    pub socket_addr: String,
}

impl<'a> URL<'a> {
    pub fn new(
        protocol: &'a str,
        hostname: &'a str,
        pathname: &'a str,
        socket_addr: String,
    ) -> URL<'a> {
        URL {
            protocol,
            hostname,
            pathname,
            socket_addr,
        }
    }
}
