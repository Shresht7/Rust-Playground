// $ cargo run -- searchstring example-filename.txt

use std::{env, error, fs, process};

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

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    // Read file contents
    let contents = fs::read_to_string(config.filepath)?;
    // cargo run -- needle poem.txt
    println!("With text:\n{contents}");
    Ok(())
}

struct Config {
    query: String,
    filepath: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();
        Ok(Config { query, filepath })
    }
}
