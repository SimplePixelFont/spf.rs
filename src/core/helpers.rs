pub(crate) use super::*;

#[cfg(feature = "log")]
use super::super::log::{LogLevel, LOGGER};

pub(crate) fn character_byte_map_to_data(character: &Character) -> (Vec<u8>, usize) {
    let mut chunks = character.byte_map.chunks(8);
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

pub(crate) fn sign_buffer(buffer: &mut byte::ByteStorage) -> &mut byte::ByteStorage {
    buffer.bytes.insert(0, byte::Byte::from_u8(70));
    buffer.bytes.insert(0, byte::Byte::from_u8(115));
    buffer.bytes.insert(0, byte::Byte::from_u8(102));

    #[cfg(feature = "log")]
    unsafe {
        let mut logger = LOGGER.lock().unwrap();
        if logger.log_level as u8 >= LogLevel::Info as u8 {
            logger.message.push_str("Signed font data.");
            logger.flush_info().unwrap();
        }
    }
    buffer
}
