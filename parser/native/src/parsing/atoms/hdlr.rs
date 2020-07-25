use crate::parsing::atoms::parse::AtomParse;
use crate::parsing::error::ParserError;
use crate::utils::{atom_utils::get_version, reader::StreamReader};
use either::Either;
use serde::Serialize;

#[derive(Serialize)]
pub struct Hdlr {
    pub name: String,
    pub predefined: u32,
    pub handler_type: String,
    pub reserved: Vec<u32>,
    pub handler_name: String,
    len: usize,
}

impl AtomParse for Hdlr {
    fn parse(my_size: usize, reader: &StreamReader) -> Result<Self, ParserError> {
        let version_flags: u32 = reader
            .read_u32()
            .ok_or_else(|| ParserError::NumberConversionError)?;
        let version = get_version(version_flags);

        let predefined = reader
            .read_u32()
            .ok_or_else(|| ParserError::NumberConversionError)?;
        let handler_type = reader
            .read_as_str(4)
            .ok_or_else(|| ParserError::StringConversionError)?;

        let v = vec![
            reader
                .read_u32()
                .ok_or_else(|| ParserError::NumberConversionError)?,
            reader
                .read_u32()
                .ok_or_else(|| ParserError::NumberConversionError)?,
            reader
                .read_u32()
                .ok_or_else(|| ParserError::NumberConversionError)?,
        ];

        let handler_name = reader
            .read_as_str(my_size - 32)
            .ok_or_else(|| ParserError::StringConversionError)?;

        Ok(Hdlr {
            name: "hdlr".into(),
            reserved: v,
            predefined,
            handler_name: handler_name.into(),
            handler_type: handler_type.into(),
            len: my_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Hdlr;
    use crate::parsing::atoms::parse::AtomParse;
    use crate::utils::reader::StreamReader;

    #[test]
    fn parse_hdlr() {
        let hdlr = Hdlr::parse(
            0x2d,
            &StreamReader::new(
                &[
                    &[0x00u8, 0x00, 0x00, 0x2d] as &[_],
                    &*b"hdlr",
                    &[
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x76, 0x69, 0x64, 0x65,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x56, 0x69, 0x64, 0x65, 0x6f, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72,
                        0x00,
                    ],
                ]
                .concat()[8..],
            ),
        )
        .unwrap();
        assert_eq!(hdlr.name, "hdlr");
        assert_eq!(hdlr.handler_type, "vide");
        assert_eq!(hdlr.handler_name, "VideoHandler\0");
    }
}
