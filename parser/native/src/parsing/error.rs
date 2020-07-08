#[derive(Debug)]
pub enum ParserError {
    IoError,
    LengthConversionError,
    NumberConversionError,
    StringConversionError,
    InvalidAtomNameError,
    UnknownError,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ParserError::IoError => f.write_str("IO error"),
            ParserError::LengthConversionError => f.write_str("Atom length conversion error"),
            ParserError::InvalidAtomNameError => f.write_str("Atom name conversion error"),
            _ => f.write_str("Unknown error"),
        }
    }
}

impl std::error::Error for ParserError {
    fn description(&self) -> &str {
        match *self {
            ParserError::IoError => "There was problem reading the file",
            ParserError::LengthConversionError => {
                "There was a problem in converting bytes to atom length"
            }
            ParserError::InvalidAtomNameError => {
                "There was a problem in converting bytes to atom name"
            }
            _ => "Unknown error",
        }
    }
}
