use crate::parsing::atoms::parse::AtomParse;
use crate::parsing::error::ParserError;
use crate::utils::reader::StreamReader;
use serde::Serialize;

#[derive(Serialize)]
pub struct Ftyp {
    pub name: String,
    major_brand: String,
    minor_brand: u32,
    compatible_brands: Vec<String>,
    len: usize,
}

impl AtomParse for Ftyp {
    fn parse(my_size: usize, reader: &StreamReader) -> Result<Self, ParserError> {
        let major_brand = reader
            .read_as_str(4)
            .ok_or_else(|| ParserError::StringConversionError)?
            .into();
        let minor_brand = reader
            .read_u32()
            .ok_or_else(|| ParserError::NumberConversionError)?;

        let mut compatible_brands = vec![];
        while reader.pos() < my_size {
            compatible_brands.push(
                reader
                    .read_as_str(4)
                    .ok_or_else(|| ParserError::StringConversionError)?
                    .into(),
            );
        }

        Ok(Ftyp {
            name: "ftyp".into(),
            major_brand,
            minor_brand,
            compatible_brands,
            len: my_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Ftyp;
    use crate::parsing::atoms::parse::AtomParse;
    use crate::utils::reader::StreamReader;
    #[test]
    fn parse_ftyp() {
        let ftyp = Ftyp::parse(
            0x14 - 8,
            &StreamReader::new(
                &[
                    [0x00, 0x00, 0x00, 0x14],
                    *b"ftyp",
                    *b"isom",
                    [0x00, 0x00, 0x00, 0x01],
                    *b"mp41",
                ]
                .concat()[8..],
            ),
        )
        .unwrap();
        assert_eq!(ftyp.name, "ftyp");
        assert_eq!(ftyp.major_brand, "isom");
        assert_eq!(ftyp.minor_brand, 1);
        assert_eq!(ftyp.compatible_brands.len(), 1);
        assert_eq!(ftyp.compatible_brands[0], "mp41");
    }
}
