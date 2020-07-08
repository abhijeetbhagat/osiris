use crate::parsing::error::ParserError;
use crate::utils::reader::StreamReader;

pub trait AtomParse {
    fn parse(my_size: usize, reader: &StreamReader) -> Result<Self, ParserError>
    where
        Self: Sized;
}

#[inline]
pub fn atom_get<P: AtomParse>(atom_len: usize, reader: &StreamReader) -> Result<P, ParserError> {
    P::parse(atom_len, reader)
}
