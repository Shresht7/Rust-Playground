use std::{error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    // Read file contents
    let contents = fs::read_to_string(config.filepath)?;
    // cargo run -- needle poem.txt
    println!("With text:\n{contents}");
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filepath: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();
        Ok(Config { query, filepath })
    }
}
