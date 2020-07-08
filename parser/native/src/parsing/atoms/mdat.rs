extern crate serde;
extern crate serde_derive;
use crate::parsing::atoms::parse::AtomParse;
use crate::parsing::error::ParserError;
use crate::utils::reader::StreamReader;
use serde::Serialize;

#[derive(Serialize)]
pub struct Mdat {
    name: String,
    len: usize,
}

impl AtomParse for Mdat {
    fn parse(my_size: usize, reader: &StreamReader) -> Result<Self, ParserError> {
        reader.skip(my_size - 8);
        Ok(Mdat {
            name: "mdat".into(),
            len: my_size,
        })
    }
}
