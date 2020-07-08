extern crate memmap;
use crate::parsing::{atoms::parser::parse, error::ParserError, info::Info};
use memmap::MmapOptions;
use std::fs::File;
use std::path::Path;

/// An ISO BMFF parser.
pub struct Parser;

impl Parser {
    /// Parses an ISO BMFF file represented by a `path`.
    ///
    /// Returns either an `Info` or a `ParseError`
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<Info, ParserError> {
        let file = File::open(path).map_err(|_| ParserError::IoError)?;
        let meta = file.metadata().map_err(|_| ParserError::IoError)?;
        let len = meta.len() as usize;

        let mmap = unsafe {
            MmapOptions::new()
                .map(&file)
                .map_err(|_| ParserError::IoError)?
        };

        parse(&mmap, len)
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    #[test]
    fn test_parsing() {
        let info = Parser::parse("spe.mp4");
        assert!(info.is_ok());
    }
}
