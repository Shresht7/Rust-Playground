use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, BufReader, BufWriter};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Cli::parse();

    log::info!("Opening file: {}", args.path.display());
    let file = std::fs::File::open(&args.path).with_context(|| {
        format!(
            "Could not open file {}",
            &args.path.to_str().unwrap_or_default()
        )
    })?;
    let mut reader = BufReader::new(file);

    let mut writer = BufWriter::new(io::stdout().lock());

    log::debug!("Starting scan loop...");
    grrs::find_matches(&mut reader, &args.pattern, &mut writer)?;

    log::info!("Finished processing file");
    Ok(())
}
