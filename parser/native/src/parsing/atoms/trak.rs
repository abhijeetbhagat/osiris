extern crate serde;
extern crate serde_derive;
use crate::parsing::atoms::{tkhd::Tkhd};
use crate::parsing::atoms::parse::{atom_get, AtomParse};
use crate::parsing::error::ParserError;
use crate::utils::reader::StreamReader;
use serde::Serialize;

#[derive(Serialize)]
pub struct Trak {
    name: String,
    len: usize,
    atoms: Vec<Box<dyn erased_serde::Serialize>>,
}

impl AtomParse for Trak {
    fn parse(my_size: usize, reader: &StreamReader) -> Result<Self, ParserError> {
        let mut atoms: Vec<Box<dyn erased_serde::Serialize>> = vec![];
        println!("trak: reader pos {} mysize {}", reader.pos(), my_size);
        let my_size = my_size + reader.pos() - 8;
        println!("trak: mysize {}", my_size);

        while reader.pos() < my_size {
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
                _ => {
                    println!("atom {} not supported", atom);
                    reader.skip(atom_len - 8);
                    println!("trak: reader pos after skipping {}", reader.pos());
                }
            }
            println!("trak: atom len: {}, atom: {}", atom_len, atom);
        }

        println!("returning trak");
        Ok(Trak {
            name: "trak".into(),
            len: my_size,
            atoms,
        })
    }
}
