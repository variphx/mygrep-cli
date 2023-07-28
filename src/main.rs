use std::env;
use std::process;

use mygrep_cli::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Querying: {}", config.query);
    println!("In file: {}", config.filename);

    if let Err(err) = mygrep_cli::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
