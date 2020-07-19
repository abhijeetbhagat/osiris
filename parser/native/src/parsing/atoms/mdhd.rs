use crate::parsing::atoms::parse::AtomParse;
use crate::parsing::error::ParserError;
use crate::utils::{atom_utils::get_version, reader::StreamReader};
use either::Either;
use serde::Serialize;

#[derive(Serialize)]
pub struct Mdhd {
    pub name: String,
    pub creation_time: Either<u32, u64>,
    pub modification_time: Either<u32, u64>,
    pub timescale: u32,
    pub duration: Either<u32, u64>,
    pub pad: u8,
    pub language: String,
    pub predefined: u16,
    len: usize,
}

impl AtomParse for Mdhd {
    fn parse(my_size: usize, reader: &StreamReader) -> Result<Self, ParserError> {
        let version_flags: u32 = reader
            .read_u32()
            .ok_or_else(|| ParserError::NumberConversionError)?;
        let version = get_version(version_flags);

        let (creation_time, modification_time, timescale, duration) = if version == 0 {
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
                reader
                    .read_u32()
                    .ok_or_else(|| ParserError::NumberConversionError)?,
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
                reader
                    .read_u32()
                    .ok_or_else(|| ParserError::NumberConversionError)?,
                Either::Right(
                    reader
                        .read_u64()
                        .ok_or_else(|| ParserError::NumberConversionError)?,
                ),
            )
        };

        let data = reader
            .read_u16()
            .ok_or_else(|| ParserError::NumberConversionError)?;

        let pad = ((data >> 15) & 1) as u8;
        let language = data & 0x7FFF;
        let language = String::from_utf8(
            [
                (97u8 + (language >> 10) as u8 - 1 % 97),
                (97u8 + (language >> 5 & 0x1F) as u8 - 1 % 97),
                (97u8 + (language & 0x1F) as u8 - 1 % 97),
            ]
            .to_vec(),
        )
        .map_err(|_| ParserError::StringConversionError)?;

        let predefined = reader
            .read_u16()
            .ok_or_else(|| ParserError::NumberConversionError)?;

        Ok(Mdhd {
            name: "mdhd".into(),
            creation_time,
            modification_time,
            timescale,
            duration,
            pad,
            language,
            predefined,
            len: my_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Mdhd;
    use crate::parsing::atoms::parse::AtomParse;
    use crate::utils::reader::StreamReader;

    #[test]
    fn parse_mdhd() {
        let mdhd = Mdhd::parse(
            0x20,
            &StreamReader::new(
                &[
                    &[0x00u8, 0x00, 0x00, 0x20] as &[_],
                    &*b"mdhd",
                    &[
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x3c, 0x00, 0x00, 0x04, 0x12, 0x00, 0x55, 0xc4, 0x00, 0x00,
                    ],
                ]
                .concat()[8..],
            ),
        )
        .unwrap();
        assert_eq!(mdhd.name, "mdhd");
        assert!(mdhd.modification_time.is_left());
        assert!(mdhd.duration.is_left());
        assert_eq!(mdhd.language, "und");
    }
}
