use clap::Parser;
use std::io::{BufRead, BufReader};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let file = std::fs::File::open(&args.path).expect("Could not open file");
    let mut reader = BufReader::new(file);

    loop {
        let mut buffer = String::new();
        let n = reader.read_line(&mut buffer).expect("Failed to read");
        if n <= 0 {
            break;
        }
        if buffer.contains(&args.pattern) {
            print!("{}", buffer);
        }
    }
}
