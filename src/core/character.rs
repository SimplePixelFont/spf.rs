pub(crate) use super::super::byte;

/// Represents a charater in the font.
#[derive(Clone, Debug)]
pub struct Character {
    pub utf8: char,
    pub custom_size: u8,
    pub byte_map: Vec<u8>,
}

impl Character {
    pub fn new(utf8: char, custom_size: u8, byte_map: Vec<u8>) -> Self {
        Self {
            utf8: utf8,
            custom_size: custom_size,
            byte_map: byte_map,
        }
    }
    pub(crate) fn segment_into_u8s(&self) -> (Vec<u8>, usize) {
        let mut chunks = self.byte_map.chunks(8);
        let mut buffer: Vec<u8> = Vec::new();
        let mut remainder = 0;

        let mut iter = chunks.next();
        while !iter.is_none() {
            let chunk = iter.unwrap();
            remainder = 8 - chunk.len();
            let mut byte = byte::Byte { bits: [false; 8] };
            let mut index: usize = 0;
            for pixel in chunk {
                byte.bits[index] = *pixel == 1; // will need to be changed later
                index += 1;
            }
            for index in 8 - remainder..8 {
                byte.bits[index] = false;
            }
            buffer.push(byte.to_u8());
            iter = chunks.next();
        }
        return (buffer, remainder);
    }
}
