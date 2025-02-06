use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let file = std::fs::File::open(&args.path).with_context(|| {
        format!(
            "Could not open file {}",
            &args.path.to_str().unwrap_or_default()
        )
    })?;
    let mut reader = BufReader::new(file);

    let mut writer = BufWriter::new(io::stdout().lock());

    loop {
        let mut buffer = String::new();
        let n = reader
            .read_line(&mut buffer)
            .with_context(|| format!("Could not read line"))?;
        if n <= 0 {
            break;
        }
        if buffer.contains(&args.pattern) {
            write!(writer, "{}", buffer)?;
        }
    }

    Ok(())
}
