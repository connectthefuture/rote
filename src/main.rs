extern crate getopts;
extern crate lua;
extern crate term;
mod error;
mod project;

use getopts::Options;
use std::env;
use std::process;

/// Prints the program usage to the console.
fn print_usage(options: Options) {
    let brief = "Usage: rote [options]";
    print!("{}", options.usage(brief));
}

/// Parses command-line options and runs retest.
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut options = Options::new();
    options.optflag("h", "help", "Print this help menu and exit.");
    options.optflag("v", "version", "Print the program version and exit.");
    options.optopt("f", "file", "Specify a Rotefile to read.", "FILE");

    let opt_matches = match options.parse(&args[1..]) {
        Ok(matches) => { matches }
        Err(err) => {
            println!("rote: error: {}", err);
            process::exit(1);
        }
    };

    // If the help flag is present or the user forgot to specify a pattern, show
    // the usage message.
    if opt_matches.opt_present("h") {
        print_usage(options);
        return;
    }

    let filename = opt_matches.opt_str("f").unwrap_or("Rotefile".to_string());

    let mut project = project::Project::new();
    if let Err(e) = project.load(&filename) {
        e.die();
    }
    project.close();
}
