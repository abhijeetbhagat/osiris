use crate::parsing::atoms::parse::AtomParse;
use crate::parsing::error::ParserError;
use crate::utils::reader::StreamReader;
use either::Either;
use serde::Serialize;

#[derive(Serialize)]
pub struct Mvhd {
    pub name: String,
    pub creation_time: Either<u32, u64>,
    pub modification_time: Either<u32, u64>,
    pub timescale: Either<u32, u64>,
    pub duration: Either<u32, u64>,
    pub next_track_id: u32,
}

impl AtomParse for Mvhd {
    fn parse(_: usize, reader: &StreamReader) -> Result<Self, ParserError> {
        let version: u32 = reader
            .read_u32()
            .ok_or_else(|| ParserError::NumberConversionError)?;

        let (creation_time, modification_time, timescale, duration) = if version == 0 {
            (
                Either::Left(
                    reader
                        .read_u32()
                        .ok_or_else(|| ParserError::NumberConversionError)?
                ),
                Either::Left(
                    reader
                        .read_u32()
                        .ok_or_else(|| ParserError::NumberConversionError)?
                ),
                Either::Left(
                    reader
                        .read_u32()
                        .ok_or_else(|| ParserError::NumberConversionError)?
                ),
                Either::Left(
                    reader
                        .read_u32()
                        .ok_or_else(|| ParserError::NumberConversionError)?
                ),
            )
        } else {
            (
                Either::Right(
                    reader
                        .read_u64()
                        .ok_or_else(|| ParserError::NumberConversionError)?
                ),
                Either::Right(
                    reader
                        .read_u64()
                        .ok_or_else(|| ParserError::NumberConversionError)?
                ),
                Either::Right(
                    reader
                        .read_u64()
                        .ok_or_else(|| ParserError::NumberConversionError)?
                ),
                Either::Right(
                    reader
                        .read_u64()
                        .ok_or_else(|| ParserError::NumberConversionError)?
                ),
            )
        };

        reader.skip(76);

        let next_track_id = reader
            .read_u32()
            .ok_or_else(|| ParserError::NumberConversionError)?;

        println!("mvhd: reader pos {}", reader.pos());

        Ok(Mvhd {
            name: "mvhd".into(),
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
    use super::Mvhd;
    use crate::parsing::atoms::parse::AtomParse;
    use crate::utils::reader::StreamReader;

    #[test]
    fn parse_mvhd() {
        let mvhd = Mvhd::parse(
            0x6c - 8,
            &StreamReader::new(
                &[
                    &[0x00u8, 0x00, 0x00, 0x6c] as &[_],
                    &*b"mvhd",
                    &[
                        0x00, 0x00, 0x00, 0x00, 0xDB, 0x07, 0xAF, 0x7D, 0xDB, 0x07, 0xAF, 0x7D,
                        0x00, 0x00, 0x03, 0xE9, 0x00, 0x00, 0x7B, 0x06, 0x00, 0x01, 0x00, 0x00,
                        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x02,
                    ],
                ]
                .concat()[8..],
            ),
        )
        .unwrap();
        assert_eq!(mvhd.name, "mvhd");
        assert!(mvhd.modification_time.is_left());
        assert!(mvhd.timescale.is_left());
        assert!(mvhd.duration.is_left());
    }
}
