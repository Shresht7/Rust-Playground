use anyhow::{Context, Result};
use std::io::{BufRead, Write};

pub fn find_matches<R, W>(reader: &mut R, pattern: &str, writer: &mut W) -> Result<()>
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
