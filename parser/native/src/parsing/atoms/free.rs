use crate::parsing::atoms::parse::AtomParse;
use crate::parsing::error::ParserError;
use crate::utils::reader::StreamReader;
use serde::Serialize;

#[derive(Serialize)]
pub struct Free {
    pub name: String,
    len: usize,
}

impl AtomParse for Free {
    fn parse(my_size: usize, reader: &StreamReader) -> Result<Self, ParserError> {
        reader.skip(my_size - 8);

        Ok(Free {
            name: "mvhd".into(),
            len: my_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Free;
    use crate::parsing::atoms::parse::AtomParse;
    use crate::utils::reader::StreamReader;

    #[test]
    fn parse_free() {}
}
