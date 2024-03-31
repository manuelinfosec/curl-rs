use clap::{Arg, ArgMatches, Command};

/// Retrieves and parses command-line arguments using Clap for a custom Curl application in Rust.
///
/// # Returns
///
/// `ArgMatches`: A structure containing parsed command-line arguments.
///
/// # Example
///
/// ```rust
/// use clap::ArgMatches;
///
/// let args = get_arguments();
/// let url = args.value_of("url").unwrap_or("");
/// let method = args.value_of("x-method").unwrap_or("GET");
/// let data = args.value_of("data").unwrap_or("");
/// let verbose = args.is_present("verbose");
/// ```
pub fn get_arguments() -> ArgMatches {
    // Initialize a command-line program
    Command::new("curl-rs")
        // Program description
        .about("Transfer data over the HTTP protocol")
        // Program version
        .version("1.0")
        // Program author
        .author("Manuel <manuelinfosec@gmail.com>")
        // First mandatory argument
        .arg(Arg::new("url").index(1).required(true))
        // Other arguments
        .arg(
            Arg::new("x-method")
                .short('X')
                .long("X-Method")
                .help("HTTP method"),
        )
        .arg(Arg::new("data").short('d').long("data").help("Payload"))
        .arg(
            Arg::new("headers")
                .short('H')
                .long("header")
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .help("Verbose mode"),
        )
        .get_matches()
}
