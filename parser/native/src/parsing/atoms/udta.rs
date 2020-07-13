use crate::parsing::atoms::parse::AtomParse;
use crate::parsing::error::ParserError;
use crate::utils::reader::StreamReader;
use serde::Serialize;

#[derive(Serialize)]
pub struct Udta {
    pub name: String,
}

impl AtomParse for Udta {
    fn parse(my_size: usize, reader: &StreamReader) -> Result<Self, ParserError> {
        reader.skip(my_size - 8);

        Ok(Udta {
            name: "udta".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Udta;
    use crate::parsing::atoms::parse::AtomParse;
    use crate::utils::reader::StreamReader;

    #[test]
    fn parse_udta() {}
}
