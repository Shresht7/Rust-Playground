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
    find_matches(&mut reader, &args.pattern, &mut writer)?;

    log::info!("Finished processing file");
    Ok(())
}

fn find_matches<R, W>(reader: &mut R, pattern: &str, writer: &mut W) -> Result<()>
where
    R: BufRead,
    W: Write,
{
    loop {
        let mut buffer = String::new();
        let n = reader
            .read_line(&mut buffer)
            .with_context(|| format!("Could not read line"))?;
        if n <= 0 {
            break;
        }
        if buffer.contains(&pattern) {
            log::debug!("Found pattern in line: {}", buffer.trim_end());
            write!(writer, "{}", buffer)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_a_match() {
        let mut result = Vec::new();
        let mut reader = "test contents\n".as_bytes();
        find_matches(&mut reader, "test", &mut result).unwrap();
        assert_eq!(result, b"test contents\n")
    }
}
