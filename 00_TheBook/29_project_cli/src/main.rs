// $ cargo run -- searchstring example-filename.txt

use project_cli::{run, Config};
use std::{env, process};

fn main() {
    // Retrieve the command-line arguments that were passed into this program
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // cargo run -- needle haystack
    println!("Searching for {}", config.query);
    println!("In file {}", config.filepath);

    if let Err(e) = run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}
