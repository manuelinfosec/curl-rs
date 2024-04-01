use clap::ArgMatches;

mod cli;
mod constants;
mod http;
mod types;
mod utils;

fn main() {
    // parse command line arguments
    let matches: ArgMatches = cli::get_arguments();

    // check if verbose exists and collect its value
    let verbose_enabled: bool = matches.contains_id("verbose") && matches.get_flag("verbose");
    // parse URL from arguments
    let url: &String = matches.get_one::<String>("url").expect("URL not specified");
    // parse request data
    let data: &String = matches
        .get_one::<String>("data")
        .expect("Could not get request data");
    // parse request method
    let method: &String = matches
        .get_one::<String>("x-method")
        .unwrap_or(&String::from("GET"));
    // parse request headers
    let headers: Vec<&str> = matches
        .get_many::<String>("headers")
        .unwrap_or_default()
        .map(|s: &String| s.as_str())
        .collect();
}
