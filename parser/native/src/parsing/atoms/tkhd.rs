use crate::parsing::atoms::parse::AtomParse;
use crate::parsing::error::ParserError;
use crate::utils::{atom_utils::get_version, reader::StreamReader};
use either::Either;
use serde::Serialize;

#[derive(Serialize)]
pub struct Tkhd {
    pub name: String,
    pub creation_time: Either<u32, u64>,
    pub modification_time: Either<u32, u64>,
    pub track_id: u32,
    pub duration: Either<u32, u64>,
    pub width: u32,
    pub height: u32,
}

impl AtomParse for Tkhd {
    fn parse(_: usize, reader: &StreamReader) -> Result<Self, ParserError> {
        let version_flags: u32 = reader
            .read_u32()
            .ok_or_else(|| ParserError::NumberConversionError)?;

        let version = get_version(version_flags);

        let (creation_time, modification_time) = if version == 0 {
            (
                Either::Left(
                    reader
                        .read_u32()
                        .ok_or_else(|| ParserError::NumberConversionError)?,
                ),
                Either::Left(
                    reader
                        .read_u32()
                        .ok_or_else(|| ParserError::NumberConversionError)?,
                ),
            )
        } else {
            (
                Either::Right(
                    reader
                        .read_u64()
                        .ok_or_else(|| ParserError::NumberConversionError)?,
                ),
                Either::Right(
                    reader
                        .read_u64()
                        .ok_or_else(|| ParserError::NumberConversionError)?,
                ),
            )
        };

        let track_id = reader
            .read_u32()
            .ok_or_else(|| ParserError::NumberConversionError)?;

        reader.skip(4);

        let duration = if version == 0 {
            Either::Left(
                reader
                    .read_u32()
                    .ok_or_else(|| ParserError::NumberConversionError)?,
            )
        } else {
            Either::Right(
                reader
                    .read_u64()
                    .ok_or_else(|| ParserError::NumberConversionError)?,
            )
        };

        /*
        reader.skip(8); //const unsigned int(32)[2] reserved = 0;
        reader.skip(2); //template int(16) layer = 0;
        reader.skip(2); //template int(16) alternate_group = 0
        reader.skip(2); //volume
        reader.skip(2); //const unsigned int(16) reserved = 0;
        reader.skip(36); //matrix
        */
        reader.skip(52);

        let width = reader
            .read_u32()
            .ok_or_else(|| ParserError::NumberConversionError)?;
        let height = reader
            .read_u32()
            .ok_or_else(|| ParserError::NumberConversionError)?;

        Ok(Tkhd {
            name: "tkhd".into(),
            creation_time,
            modification_time,
            duration,
            track_id,
            width,
            height,
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
