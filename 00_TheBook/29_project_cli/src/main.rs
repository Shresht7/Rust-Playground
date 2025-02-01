// $ cargo run -- searchstring example-filename.txt

use std::{env, fs};

fn main() {
    // Retrieve the command-line arguments that were passed into this program
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    // cargo run -- needle haystack
    println!("Searching for {}", config.query);
    println!("In file {}", config.filepath);

    // Read file contents
    let contents =
        fs::read_to_string(config.filepath).expect("Should have been able to read the file");

    // cargo run -- needle poem.txt
    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    filepath: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filepath = args[2].clone();
        Config { query, filepath }
    }
}
