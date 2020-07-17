extern crate serde;
extern crate serde_derive;
use crate::parsing::atoms::parse::{atom_get, AtomParse};
use crate::parsing::atoms::{edts::Edts, mdia::Mdia, tkhd::Tkhd};
use crate::parsing::error::ParserError;
use crate::utils::reader::StreamReader;
use serde::Serialize;

#[derive(Serialize)]
pub struct Trak {
    name: String,
    len: usize,
    atoms: Vec<Box<dyn erased_serde::Serialize + Send + Sync>>,
}

impl AtomParse for Trak {
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
                "tkhd" => atoms.push(Box::new(atom_get::<Tkhd>(atom_len, reader)?)),
                "edts" => atoms.push(Box::new(atom_get::<Edts>(atom_len, reader)?)),
                "mdia" => atoms.push(Box::new(atom_get::<Mdia>(atom_len, reader)?)),
                _ => {
                    reader.skip(atom_len - 8);
                }
            }
            limit += atom_len;
        }

        Ok(Trak {
            name: "trak".into(),
            len: my_size,
            atoms,
        })
    }
}
