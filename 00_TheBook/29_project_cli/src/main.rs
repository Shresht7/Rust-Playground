// $ cargo run -- searchstring example-filename.txt

use std::{env, fs};

fn main() {
    // Retrieve the command-line arguments that were passed into this program
    let args: Vec<String> = env::args().collect();

    // Extract the query and filepath from the arguments
    let query = &args[1];
    let filepath = &args[2];

    // cargo run -- needle haystack
    println!("Searching for {query}");
    println!("In file {filepath}");

    // Read file contents
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");

    // cargo run -- needle poem.txt
    println!("With text:\n{contents}");
}
