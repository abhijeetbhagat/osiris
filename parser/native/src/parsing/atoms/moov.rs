extern crate crossbeam_utils;
use crate::parsing::atoms::parse::{atom_get, AtomParse};
use crate::parsing::atoms::{mvhd::Mvhd, trak::Trak};
use crate::parsing::error::ParserError;
use crate::utils::reader::StreamReader;
use crossbeam_utils::thread;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::thread as std_thread;

#[derive(Serialize)]
pub struct Moov {
    name: String,
    len: usize,
    atoms: Vec<Box<dyn erased_serde::Serialize + Send + Sync>>,
}

/// Moov parsing involves parallel parsing of all the trak atoms. This works by 'cloning' the reader
/// and then kicking of the thread per trak atom in a crossbeam scope.
/// The trak atoms are stored in a vector that is wrapped in an Arc<Mutex<...>>.

impl AtomParse for Moov {
    fn parse(my_size: usize, reader: &StreamReader) -> Result<Self, ParserError> {
        //let my_size = my_size + reader.pos() - 8;
        let mut limit = 8;
        let mut atoms: Arc<Mutex<Vec<Box<dyn erased_serde::Serialize + Send + Sync>>>> =
            Arc::new(Mutex::new(vec![]));

        thread::scope(|s| {
            while limit < my_size {
                let (atom_len, atom) = (
                    reader
                        .read_u32()
                        .ok_or_else(|| ParserError::LengthConversionError)
                        .unwrap() as usize,
                    reader
                        .read_as_str(4)
                        .ok_or_else(|| ParserError::InvalidAtomNameError)
                        .unwrap(),
                );

                match atom {
                    "mvhd" => {
                        let mut atoms = atoms.lock().unwrap();
                        atoms.push(Box::new(atom_get::<Mvhd>(atom_len, reader).unwrap()));
                    }
                    "trak" => {
                        let mut atoms = atoms.clone();

                        let trak_reader = reader.clone_from_current_pos();
                        s.spawn(move |_| {
                            let mut atoms = atoms.lock().unwrap();
                            atoms.push(Box::new(atom_get::<Trak>(atom_len, &trak_reader).unwrap()));
                        });
                        reader.skip(atom_len - 8);
                    }
                    _ => {
                        reader.skip(atom_len - 8);
                    }
                }
                limit += atom_len;
            }
        });

        let atoms: Vec<Box<dyn erased_serde::Serialize + Send + Sync>> = Arc::try_unwrap(atoms)
            .unwrap_or_default()
            .into_inner()
            .unwrap();

        Ok(Moov {
            name: "moov".into(),
            len: my_size,
            atoms,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Moov;
    use crate::parsing::atoms::parse::AtomParse;
    use crate::parsing::atoms::trak::Trak;
    use crate::utils::reader::StreamReader;
    use memmap::MmapOptions;
    use std::fs::File;

    #[test]
    fn test_moov() {
        let file = File::open("moov-atom").unwrap();
        let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
        let reader = StreamReader::new(&mmap);
        reader.skip(8);
        let moov = Moov::parse(file.metadata().unwrap().len() as usize, &reader);
        assert!(moov.is_ok());
        let moov = moov.unwrap();
        println!("moov atoms length {}", moov.atoms.len());
        //assert_eq!(moov.atoms.len(), 3);
    }
}
