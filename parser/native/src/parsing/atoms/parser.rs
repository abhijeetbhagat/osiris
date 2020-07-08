use crate::parsing::{
    atoms::{free::Free, ftyp::Ftyp, mdat::Mdat, moov::Moov, parse::atom_get},
    error::ParserError,
    info::Info,
};
use crate::utils::reader::StreamReader;

/// Starts the parsing process by initializing a `StreamReader` from the `buf`
/// and return either an `Info` or a `ParserError`
pub fn parse(buf: &[u8], len: usize) -> Result<Info, ParserError> {
    let start = 0usize;
    let mut _result = String::from("");
    let mut atoms: Vec<Box<dyn erased_serde::Serialize>> = vec![];
    let reader = StreamReader::new(&buf[start..]);

    while reader.pos() < len - start {
        println!("parser: reader pos {} len {}", reader.pos(), len);
        let (atom_len, atom) = (
            reader
                .read_u32()
                .ok_or_else(|| ParserError::LengthConversionError)? as usize,
            reader
                .read_as_str(4)
                .ok_or_else(|| ParserError::InvalidAtomNameError)?
        );
        println!("reading ...");
        match atom {
            "ftyp" => atoms.push(Box::new(atom_get::<Ftyp>(atom_len, &reader)?)),
            "mdat" => atoms.push(Box::new(atom_get::<Mdat>(atom_len, &reader)?)),
            "moov" => atoms.push(Box::new(atom_get::<Moov>(atom_len, &reader)?)),
            "free" => atoms.push(Box::new(atom_get::<Free>(atom_len, &reader)?)),
            _ => {
                println!("parser: unsupported atom {}", atom);
                return Err(ParserError::InvalidAtomNameError);
            }
        }
        println!("atom len: {}, atom: {}", atom_len, atom);
    }

    Ok(Info::new(len, atoms))
}
