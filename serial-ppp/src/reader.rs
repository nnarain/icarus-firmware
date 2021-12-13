//
// reader.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 11 2021
//
use bincode::de::read::Reader;

/// A bincode [Reader] used to read data from a ring buffer
pub struct RingBufferReader<'a> {
    buf: &'a [u8],
    start_idx: usize,
}

impl<'a> RingBufferReader<'a> {
    /// Construct a new [RingBufferReader] with a reference to a data buffer and a start index into that
    /// buffer
    pub fn new(buf: &'a [u8], start_idx: usize) -> Self {
        RingBufferReader {
            buf,
            start_idx,
        }
    }
}

impl Reader for RingBufferReader<'_> {
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), bincode::error::DecodeError> {
        let len = bytes.len();
        // Ring buffer iterator
        // Cycle allows looping back to the start of the buffer
        let iter = self.buf.iter().cycle().skip(self.start_idx).take(len);
        for (byte, read_byte) in bytes.iter_mut().zip(iter) {
            *byte = *read_byte;
        }

        self.start_idx += len;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use bincode::{Encode, Decode};

    #[derive(Encode, Decode, PartialEq, Debug)]
    enum Bar {
        A, B, C,
    }

    #[derive(Encode, Decode, PartialEq, Debug)]
    struct Foo {
        x: u8,
        b: Bar,
    }

    #[test]
    fn ring_buffer_no_wrap() {
        let mut buf: [u8; 10] = [0; 10];
        
        let foo = Foo {x: 10, b: Bar::B};
        let _ = bincode::encode_into_slice(foo, &mut buf, bincode::config::Configuration::standard()).unwrap();

        let reader = RingBufferReader::new(&buf, 0);
        let foo: Foo = bincode::decode_from_reader(reader, bincode::config::Configuration::standard()).unwrap();

        assert_eq!(foo, Foo {x: 10, b: Bar::B});
    }

    #[test]
    fn ring_buffer_wrap() {
        let mut buf1: [u8; 10] = [0; 10];
        let mut buf2: [u8; 10] = [0; 10];

        let foo = Foo {x: 10, b: Bar::B};
        let len = bincode::encode_into_slice(foo, &mut buf1, bincode::config::Configuration::standard()).unwrap();

        // shift bytes to the end and wrap around
        let mut idx = buf2.len() - 1;
        for i in 0..len {
            buf2[idx] = buf1[i];
            idx = (idx + 1) % buf1.len();
        }

        let reader = RingBufferReader::new(&buf2, 9);
        let foo: Foo = bincode::decode_from_reader(reader, bincode::config::Configuration::standard()).unwrap();

        assert_eq!(foo, Foo {x: 10, b: Bar::B});
    }
}

