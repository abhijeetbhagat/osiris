extern crate serde;
extern crate serde_derive;
use crate::parsing::atoms::mvhd::Mvhd;
use crate::parsing::atoms::parse::{atom_get, AtomParse};
use crate::parsing::error::ParserError;
use crate::utils::reader::StreamReader;
use serde::Serialize;

#[derive(Serialize)]
pub struct Moov {
    name: String,
    len: usize,
    atoms: Vec<Box<dyn erased_serde::Serialize>>,
}

impl AtomParse for Moov {
    fn parse(my_size: usize, reader: &StreamReader) -> Result<Self, ParserError> {
        let mut atoms: Vec<Box<dyn erased_serde::Serialize>> = vec![];
        println!("moov: reader pos {} mysize {}", reader.pos(), my_size);
        let my_size = my_size + reader.pos() - 8;
        println!("moov: mysize {}", my_size);

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
                "mvhd" => atoms.push(Box::new(atom_get::<Mvhd>(atom_len, reader)?)),
                _ => {
                    println!("atom {} not supported", atom);
                    reader.skip(atom_len - 8);
                    println!("moov: reader pos after skipping {}", reader.pos());
                }
            }
            println!("moov: atom len: {}, atom: {}", atom_len, atom);
        }

        println!("returning moov");
        Ok(Moov {
            name: "moov".into(),
            len: my_size,
            atoms,
        })
    }
}
