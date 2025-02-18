pub(crate) use super::byte;
pub(crate) use super::core::Header;
#[cfg(feature = "log")]
pub(crate) use super::log::{LogLevel, LOGGER};

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
    // let string = " ok ok";

    // let string2: Vec<&str> = string.split("\n").collect();
    // println!("{:?}", string2);
}

pub(crate) fn push_header<'a>(
    buffer: &'a mut byte::ByteStorage,
    header: &Header,
) -> &'a mut byte::ByteStorage {
    buffer.push(byte::Byte {
        bits: [
            header.configuration_flags.alignment,
            false,
            false,
            false,
            header.modifier_flags.compact,
            false,
            false,
            false,
        ],
    });

    buffer.push(byte::Byte::from_u8(header.required_values.constant_size));

    buffer
}

pub(crate) fn push_character(
    buffer: &mut byte::ByteStorage,
    character: char,
) -> &mut byte::ByteStorage {
    let mut char_buffer = [0; 4];
    character.encode_utf8(&mut char_buffer);

    let mut utf8_bit_string = String::new(); // part of log

    for byte in char_buffer {
        if byte != 0 {
            utf8_bit_string.push_str(&format!("{:0b} ", byte)); // part of log

            buffer.push(byte::Byte::from_u8(byte));
        }
    }

    buffer
}

pub(crate) fn push_custom_size(
    buffer: &mut byte::ByteStorage,
    custom_size: u8,
) -> &mut byte::ByteStorage {
    buffer.push(byte::Byte::from_u8(custom_size));

    let size_bit_string = format!("{:0b}", custom_size); // part of log

    buffer
}

pub(crate) fn push_byte_map(
    buffer: &mut byte::ByteStorage,
    header: &Header,
    character_bytes: Vec<u8>,
    remaining_space: usize,
) {
    let mut byte_map_bit_string = String::new();

    let used_bytes = character_bytes.len();
    let mut index = 0;
    for byte in character_bytes {
        byte_map_bit_string.push_str(&format!("{:0b} ", byte));

        if header.modifier_flags.compact && index == used_bytes - 1 {
            buffer.incomplete_push(byte::Byte::from_u8(byte), remaining_space);
        } else {
            buffer.push(byte::Byte::from_u8(byte));
        }
        index += 1;
    }
}

pub(crate) fn next_character(
    body_buffer: &mut byte::ByteStorage,
    mut current_index: usize,
) -> char {
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

    String::from_utf8(utf8_bytes.to_vec())
        .unwrap()
        .chars()
        .next()
        .unwrap()
}
