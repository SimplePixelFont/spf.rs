pub(crate) use super::*;

pub(crate) fn character_pixmap_to_data(character: &Character) -> (Vec<u8>, usize) {
    let mut chunks = character.pixmap.chunks(8);
    let mut buffer: Vec<u8> = Vec::new();
    let mut remainder = 0;

    let mut iter = chunks.next();
    while iter.is_some() {
        let chunk = iter.unwrap();
        remainder = 8 - chunk.len();
        let mut byte = byte::Byte { bits: [false; 8] };
        for (index, pixel) in chunk.iter().enumerate() {
            byte.bits[index] = *pixel == 1; // will need to be changed later
        }
        for index in 8 - remainder..8 {
            byte.bits[index] = false;
        }
        buffer.push(byte.to_u8());
        iter = chunks.next();
    }
    (buffer, remainder)
}

pub(crate) fn sign_buffer(buffer: &mut byte::ByteStorage) -> &mut byte::ByteStorage {
    buffer.bytes.insert(0, byte::Byte::from_u8(70));
    buffer.bytes.insert(0, byte::Byte::from_u8(115));
    buffer.bytes.insert(0, byte::Byte::from_u8(102));

    #[cfg(feature = "log")]
    info!("Signed font data.");

    buffer
}
