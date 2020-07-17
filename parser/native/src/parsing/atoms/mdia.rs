extern crate serde;
extern crate serde_derive;
use crate::parsing::atoms::parse::{atom_get, AtomParse};
use crate::parsing::error::ParserError;
use crate::utils::reader::StreamReader;
use serde::Serialize;

#[derive(Serialize)]
pub struct Mdia {
    name: String,
    len: usize,
    atoms: Vec<Box<dyn erased_serde::Serialize + Send + Sync>>,
}

impl AtomParse for Mdia {
    fn parse(my_size: usize, reader: &StreamReader) -> Result<Self, ParserError> {
        reader.skip(my_size - 8);
        Ok(Mdia {
            name: "mdia".into(),
            len: my_size,
            atoms: vec![],
        })
    }
}
