use crate::parsing::atoms::parse::AtomParse;
use crate::parsing::error::ParserError;
use crate::utils::{atom_utils::get_version, reader::StreamReader};
use either::Either;
use serde::Serialize;

#[derive(Serialize)]
pub struct Elst {
    pub name: String,
    pub entry_count: u32,
    pub segment_durations: Vec<Either<u32, u64>>,
    pub media_times: Vec<Either<u32, u64>>,
    pub media_rate_integers: Vec<u16>,
    pub media_rate_fractions: Vec<u16>,
    len: usize,
}

impl AtomParse for Elst {
    fn parse(my_size: usize, reader: &StreamReader) -> Result<Self, ParserError> {
        let version_flags: u32 = reader
            .read_u32()
            .ok_or_else(|| ParserError::NumberConversionError)?;

        let version = get_version(version_flags);
        let entry_count = reader
            .read_u32()
            .ok_or_else(|| ParserError::NumberConversionError)?;

        let mut segment_durations = vec![];
        let mut media_times = vec![];
        let mut media_rate_integers = vec![];
        let mut media_rate_fractions = vec![];

        for _ in 0..entry_count {
            let (segment_duration, media_time) = if version == 0 {
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
            segment_durations.push(segment_duration);
            media_times.push(media_time);

            media_rate_integers.push(
                reader
                    .read_u16()
                    .ok_or_else(|| ParserError::NumberConversionError)?,
            );

            media_rate_fractions.push(
                reader
                    .read_u16()
                    .ok_or_else(|| ParserError::NumberConversionError)?,
            );
        }

        Ok(Elst {
            name: "elst".into(),
            entry_count,
            segment_durations,
            media_times,
            media_rate_integers,
            media_rate_fractions,
            len: my_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Elst;
    use crate::parsing::atoms::parse::AtomParse;
    use crate::utils::reader::StreamReader;

    #[test]
    fn parse_elst() {}
}
