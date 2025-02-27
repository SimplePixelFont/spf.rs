pub(crate) use super::*;

#[cfg(feature = "log")]
use super::super::log::{LogLevel, LOGGER};

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
            utf8_bit_string.push_str(&format!("{:08b} ", byte)); // part of log

            buffer.push(byte::Byte::from_u8(byte));
        }
    }

    #[cfg(feature = "log")]
    unsafe {
        let mut logger = LOGGER.lock().unwrap();
        if logger.log_level as u8 >= LogLevel::Info as u8 {
            logger.message.push_str(&format!(
                "Pushed character '{}' with the following bits: {}",
                character, utf8_bit_string
            ));
            logger.flush_info().unwrap();
        }
    }

    buffer
}

pub(crate) fn push_custom_size(
    buffer: &mut byte::ByteStorage,
    custom_size: u8,
) -> &mut byte::ByteStorage {
    buffer.push(byte::Byte::from_u8(custom_size));

    let size_bit_string = format!("{:08b}", custom_size);

    #[cfg(feature = "log")]
    unsafe {
        let mut logger = LOGGER.lock().unwrap();
        if logger.log_level as u8 >= LogLevel::Info as u8 {
            logger.message.push_str(&format!(
                "Pushed custom size '{}' with the following bits: {}",
                custom_size, size_bit_string
            ));
            logger.flush_info().unwrap();
        }
    }

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
    for byte in character_bytes.iter() {
        byte_map_bit_string.push_str(&format!("{:08b} ", byte));

        if header.modifier_flags.compact && index == used_bytes - 1 {
            buffer.incomplete_push(byte::Byte::from_u8(*byte), remaining_space);
        } else {
            buffer.push(byte::Byte::from_u8(*byte));
        }
        index += 1;
    }

    #[cfg(feature = "log")]
    unsafe {
        let mut logger = LOGGER.lock().unwrap();
        if logger.log_level as u8 >= LogLevel::Info as u8 {
            logger.message.push_str(&format!(
                "Pushed byte map with the following bits: {}",
                byte_map_bit_string
            ));
            logger.flush_info().unwrap();
        }
    }
}
