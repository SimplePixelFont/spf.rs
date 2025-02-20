pub(crate) use super::super::byte;

pub(crate) fn next_character(
    body_buffer: &mut byte::ByteStorage,
    mut current_index: usize,
) -> (char, usize) {
    let utf81 = body_buffer.get(current_index);
    let mut utf8_bytes: [u8; 4] = [0, 0, 0, 0];

    if utf81.bits[7..] == [false] {
        utf8_bytes[0] = utf81.to_u8();
    } else if utf81.bits[5..] == [false, true, true, true] {
        utf8_bytes[0] = utf81.to_u8();
        current_index += 1;
        utf8_bytes[1] = body_buffer.get(current_index).to_u8();
    } else if utf81.bits[4..] == [false, true, true, true] {
        utf8_bytes[0] = utf81.to_u8();
        current_index += 1;
        utf8_bytes[1] = body_buffer.get(current_index).to_u8();
        current_index += 1;
        utf8_bytes[2] = body_buffer.get(current_index).to_u8();
    } else if utf81.bits[3..] == [false, true, true, true, true] {
        utf8_bytes[0] = utf81.to_u8();
        current_index += 1;
        utf8_bytes[1] = body_buffer.get(current_index).to_u8();
        current_index += 1;
        utf8_bytes[2] = body_buffer.get(current_index).to_u8();
        current_index += 1;
        utf8_bytes[3] = body_buffer.get(current_index).to_u8();
    }

    return (
        String::from_utf8(utf8_bytes.to_vec())
            .unwrap()
            .chars()
            .next()
            .unwrap(),
        current_index,
    );
}
