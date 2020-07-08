use std::cell::RefCell;
use std::convert::TryInto;
/// An abstraction over reading strings/integers from a stream of bytes
pub struct StreamReader<'a> {
    buf: &'a [u8],
    pos: RefCell<usize>,
}

impl<'a> StreamReader<'a> {
    /// Returns a new `StreamReader` instance with an underlying `buf`
    pub fn new(buf: &'a [u8]) -> Self {
        StreamReader {
            buf,
            pos: RefCell::new(0),
        }
    }

    /// Reads the next byte from cursor position
    pub fn read_u8(&self) -> Option<u8> {
        let byte = self.buf[*self.pos.borrow()];
        *self.pos.borrow_mut() += 1;
        Some(byte)
    }

    /// Reads the next 2 bytes from cursor position into an u16
    pub fn read_u16(&self) -> Option<u16> {
        let result = u16::from_be_bytes(
            self.buf[*self.pos.borrow()..*self.pos.borrow() + 2]
                .try_into()
                .unwrap(),
        );
        *self.pos.borrow_mut() += 2;
        Some(result)
    }

    /// Reads the next 4 bytes from cursor position into an u32
    pub fn read_u32(&self) -> Option<u32> {
        let result = u32::from_be_bytes(
            self.buf[*self.pos.borrow()..*self.pos.borrow() + 4]
                .try_into()
                .unwrap(),
        );
        *self.pos.borrow_mut() += 4;
        Some(result)
    }

    /// Reads the next 8 bytes from cursor position into an u64
    pub fn read_u64(&self) -> Option<u64> {
        let result = u64::from_be_bytes(
            self.buf[*self.pos.borrow()..*self.pos.borrow() + 8]
                .try_into()
                .unwrap(),
        );
        *self.pos.borrow_mut() += 8;
        Some(result)
    }

    /// Converts `n` bytes from cursor position into an `&str`
    pub fn read_as_str(&self, n: usize) -> Option<&str> {
        let result =
            std::str::from_utf8(&self.buf[*self.pos.borrow()..*self.pos.borrow() + n]).unwrap();
        *self.pos.borrow_mut() += n;
        Some(result)
    }

    /// Moves the cursor in the bytes stream by `n` bytes
    #[inline]
    pub fn skip(&self, n: usize) {
        *self.pos.borrow_mut() += n;
    }

    /// Returns current cursor position in the stream
    #[inline]
    pub fn pos(&self) -> usize {
        *self.pos.borrow_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::StreamReader;

    #[test]
    fn test_bytes_reading() {
        let rdr = StreamReader::new(&[9, 2, 3, 4, 5, 6, 7, 8, 0x66, 0x74, 0x79, 0x70]);
        assert_eq!(rdr.read_u8(), Some(9));
        assert_eq!(rdr.read_u8(), Some(2));
        assert_eq!(rdr.pos(), 2);
        assert_eq!(rdr.read_u16(), Some(772));
        assert_eq!(rdr.read_u32(), Some(84281096));
        assert_eq!(rdr.pos(), 8);
        assert_eq!(rdr.read_as_str(4), Some("ftyp"));
    }
}
