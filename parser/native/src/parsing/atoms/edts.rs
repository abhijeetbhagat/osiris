extern crate serde;
extern crate serde_derive;
use crate::parsing::atoms::elst::Elst;
use crate::parsing::atoms::parse::{atom_get, AtomParse};
use crate::parsing::error::ParserError;
use crate::utils::reader::StreamReader;
use serde::Serialize;

#[derive(Serialize)]
pub struct Edts {
    name: String,
    len: usize,
    atoms: Vec<Box<dyn erased_serde::Serialize + Send + Sync>>,
}

impl AtomParse for Edts {
    fn parse(my_size: usize, reader: &StreamReader) -> Result<Self, ParserError> {
        let mut atoms: Vec<Box<dyn erased_serde::Serialize + Send + Sync>> = vec![];
        let mut limit = 8;

        while limit < my_size {
            let (atom_len, atom) = (
                reader
                    .read_u32()
                    .ok_or_else(|| ParserError::LengthConversionError)? as usize,
                reader
                    .read_as_str(4)
                    .ok_or_else(|| ParserError::InvalidAtomNameError)?,
            );

            match atom {
                "elst" => atoms.push(Box::new(atom_get::<Elst>(atom_len, reader)?)),
                _ => {
                    reader.skip(atom_len - 8);
                }
            }
            limit += atom_len;
        }

        Ok(Edts {
            name: "edts".into(),
            len: my_size,
            atoms,
        })
    }
}
