use crate::parsing::atoms::parse::AtomParse;
use crate::parsing::error::ParserError;
use crate::utils::reader::StreamReader;
use either::Either;
use serde::Serialize;

#[derive(Serialize)]
pub struct Tkhd {
    pub name: String,
    pub creation_time: Either<u32, u64>,
    pub modification_time: Either<u32, u64>,
    pub timescale: Either<u32, u64>,
    pub duration: Either<u32, u64>,
    pub next_track_id: u32,
}

impl AtomParse for Tkhd {
    fn parse(_: usize, reader: &StreamReader) -> Result<Self, ParserError> {
        let version: u32 = reader
            .read_u32()
            .ok_or_else(|| ParserError::NumberConversionError)?
            .into();

        let (creation_time, modification_time) = if version == 0 {
            (
                Either::Left(
                    reader
                        .read_u32()
                        .ok_or_else(|| ParserError::NumberConversionError)?
                        .into(),
                ),
                Either::Left(
                    reader
                        .read_u32()
                        .ok_or_else(|| ParserError::NumberConversionError)?
                        .into(),
                ),
            )
        } else {
            (
                Either::Right(
                    reader
                        .read_u64()
                        .ok_or_else(|| ParserError::NumberConversionError)?
                        .into(),
                ),
                Either::Right(
                    reader
                        .read_u64()
                        .ok_or_else(|| ParserError::NumberConversionError)?
                        .into(),
                ),
            )
        };

        let track_id = reader
            .read_u32()
            .ok_or_else(|| ParserError::NumberConversionError)?
            .into();

        reader.skip(4);

        let next_track_id = reader
            .read_u32()
            .ok_or_else(|| ParserError::NumberConversionError)?
            .into();

        println!("tkhd: reader pos {}", reader.pos());

        Ok(Tkhd {
            name: "tkhd".into(),
            creation_time,
            modification_time,
            timescale,
            duration,
            next_track_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Tkhd;
    use crate::parsing::atoms::parse::AtomParse;
    use crate::utils::reader::StreamReader;

    #[test]
    fn parse_tkhd() {}
}
