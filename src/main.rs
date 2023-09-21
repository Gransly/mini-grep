use std::{env, process};
use minigrep::{Config, run};
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching querry is {}", config.query);
    println!("File path is {}", config.file_path);

    if let Err(e ) = run(config) {
        println!("Application error {e}");
        process::exit(1);
    }
}

