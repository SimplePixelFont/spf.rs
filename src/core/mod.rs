//! Essential functions and structs used by both the native crate and FFI interface.
//!
//! <div class="warning">
//!
//! If you are using `spf.rs` as a native Rust crate you may instead want to use the interface exposed
//! from the [`crate::ergonomics`] feature module.
//!
//! </div>
//!
//! This module provides raw composite structs that aim to reflect the structure of a `SimplePixelFont`
//! binary file. Additionally it defines the [`layout_to_data`] and [`layout_from_data`] functions that
//! can be used to convert between the structs and the binary data.

pub(crate) mod byte;
pub(crate) mod composers;
pub(crate) mod helpers;
pub(crate) mod parsers;

#[cfg(feature = "log")]
use crate::log::{LogLevel, LOGGER};

#[derive(Debug, Clone)]
/// Defines the configuration flags for a font [`Layout`] struct.
///
/// Each field is a [`bool`] and in the binary file will be represented by a single bit.
pub struct ConfigurationFlags {
    /// Determines if all the font characters are alligned by width (false) and in turn have the same width. Or aligned by height (true), in which case all letters will have the same height. This value for width or height is storee in [`RequiredValues.constant_size`].
    pub alignment: bool,
}
#[derive(Debug, Clone)]
/// Defines the modifier flags for a font [`Layout`] struct.
///
/// If the field is set to true, then the modifer will be applied to the font [`Layout`] struct.
/// Each field is a [`bool`] and in the binary file will be represented by a single bit.
pub struct ModifierFlags {
    /// If enabled (value set to true), font body will be compacted, removing padding bytes after each character definition. Without compact enabled, [`layout_to_data`] will end each character bitmap with padding 0's if `(constant_size * custom_size) % 8` results in a remainder that is not 0. The number of padding 0's is the remainder of the formula above.
    pub compact: bool,
}

#[derive(Debug, Clone)]
/// Defines the required values for a [`Layout`] structs.
pub struct RequiredValues {
    // The size that each character defined in this font [`Layout`] will use. This may be the width or height of each character depending on the [`ConfigurationFlags.alignment`].
    pub constant_size: u8,
}

#[derive(Debug, Clone)]
/// Represents the header of a font [`Layout`] struct.
///
/// The [`Header`] struct contains the configuration flags, modifier flags and required values
/// of a [`Layout`]. These values are essential in determining how the font will be interpreted
/// by [`layout_to_data`] and [`layout_from_data`] functions.
pub struct Header {
    pub configuration_flags: ConfigurationFlags,
    pub modifier_flags: ModifierFlags,
    pub required_values: RequiredValues,
}

/// Represents a charater in the font.
///
/// The [`Character`] struct contains the utf8 character, custom size and byte map of a character.
/// Please note that while the byte_map uses a u8 for each pixel, when the font is converted to
/// a data vector, each pixel will be represented by a single bit.
#[derive(Debug, Clone)]
pub struct Character {
    pub utf8: char,
    pub custom_size: u8,
    pub byte_map: Vec<u8>,
}

/// Represents the body of a font [`Layout`] struct.
///
/// The [`Body`] struct contains the characters of a [`Layout`] as a Vector.
#[derive(Debug, Clone)]
pub struct Body {
    pub characters: Vec<Character>,
}

/// Represents the entire font [`Layout`] struct.
///
/// The [`Layout`] struct aims to reflect the structure of a `SimplePixelFont` binary file.
#[derive(Debug, Clone)]
pub struct Layout {
    pub header: Header,
    pub body: Body,
}

/// Parses a [`Vec<u8>`] into a font [`Layout`].
pub fn layout_from_data(buffer: Vec<u8>) -> Layout {
    let mut current_index = 0;
    let mut chunks = buffer.iter();

    let mut header = Header {
        configuration_flags: ConfigurationFlags { alignment: false },
        modifier_flags: ModifierFlags { compact: false },
        required_values: RequiredValues { constant_size: 0 },
    };
    let mut body = Body { characters: vec![] };

    let mut character_definition_stage = 0;
    let mut current_character: Character = Character {
        utf8: ' ',
        custom_size: 0,
        byte_map: vec![],
    };

    let mut body_buffer = byte::ByteStorage::new();

    let mut iter = chunks.next();
    while !iter.is_none() {
        let chunk = iter.unwrap();
        if current_index < 3 {
            if !chunk == [102, 115, 70][current_index] {
                panic!("File is not signed")
            }
        } else if current_index == 3 {
            let file_properties = byte::Byte::from_u8(chunk.clone()).bits;
            header.configuration_flags.alignment = file_properties[0];
            header.modifier_flags.compact = file_properties[4];
        } else if current_index == 4 {
            header.required_values.constant_size = chunk.clone();
        } else {
            body_buffer.push(byte::Byte::from_u8(chunk.clone()));
        }
        iter = chunks.next();
        current_index += 1;
    }

    current_index = 0;
    let length = body_buffer.bytes.len();
    while current_index < length - 1 {
        if character_definition_stage == 0 {
            let result = parsers::next_character(&mut body_buffer, current_index);
            current_character.utf8 = result.0;
            current_index = result.1;
            current_index += 1;
            character_definition_stage += 1;

            #[cfg(feature = "log")]
            unsafe {
                let mut logger = LOGGER.lock().unwrap();
                if logger.log_level as u8 >= LogLevel::Debug as u8 {
                    logger.message.push_str(&format!(
                        "Identified utf8 character: {:?}",
                        current_character.utf8
                    ));
                    logger.flush_debug().unwrap();
                }
            }
        }
        if character_definition_stage == 1 {
            current_character.custom_size = body_buffer.get(current_index).to_u8();
            current_index += 1;
            character_definition_stage += 1;
        }

        if character_definition_stage == 2 {
            let bytes_used = (((current_character.custom_size as f32
                * header.required_values.constant_size as f32)
                as f32
                / 8.0) as f32)
                .ceil() as u8;

            let remainder = bytes_used as usize * 8 as usize
                - (current_character.custom_size as usize
                    * header.required_values.constant_size as usize);

            let mut current_byte = body_buffer.get(current_index);
            for i in 0..bytes_used {
                let mut counter = 0;
                for bit in current_byte.bits {
                    if !(i == bytes_used - 1 && counter >= 8 - remainder) {
                        current_character.byte_map.push(bit as u8);
                    }
                    counter += 1;
                }

                if i < bytes_used - 1 {
                    current_index += 1;
                    current_byte = body_buffer.get(current_index);
                }
            }

            body.characters.push(current_character.clone());
            current_index += 1;

            if header.modifier_flags.compact {
                if body_buffer.pointer + (8 - remainder) < 8 {
                    current_index -= 1;
                }
                body_buffer.pointer = ((8 - remainder) as usize + body_buffer.pointer) % 8;

                #[cfg(feature = "log")]
                unsafe {
                    let mut logger = LOGGER.lock().unwrap();
                    if logger.log_level as u8 >= LogLevel::Debug as u8 {
                        logger.message.push_str(
                            &format!("Last character pushed had {} padding bits, now reading with offset of {}, starting at byte {}",
                                remainder, body_buffer.pointer, current_index),
                        );
                        logger.flush_debug().unwrap();
                    }
                }
            }

            current_character.byte_map = vec![];
            character_definition_stage = 0;
        }
    }

    Layout {
        header: header,
        body: body,
    }
}

/// Encodes the provided font [`Layout`] into a [`Vec<u8>`].
pub fn layout_to_data(layout: &Layout) -> Vec<u8> {
    let mut buffer = byte::ByteStorage::new();
    helpers::sign_buffer(&mut buffer);
    composers::push_header(&mut buffer, &layout.header);

    let mut saved_space = 0;

    for character in &layout.body.characters {
        composers::push_character(&mut buffer, character.utf8);
        composers::push_custom_size(&mut buffer, character.custom_size);

        let result = helpers::character_byte_map_to_data(&character);
        let character_bytes = result.0;
        let remaining_space = result.1;

        composers::push_byte_map(
            &mut buffer,
            &layout.header,
            character_bytes,
            remaining_space,
        );

        if layout.header.modifier_flags.compact {
            saved_space += remaining_space;
            buffer.pointer = ((8 - remaining_space) + buffer.pointer) % 8;
        }
    }

    #[cfg(feature = "log")]
    unsafe {
        let mut logger = LOGGER.lock().unwrap();
        if logger.log_level as u8 >= LogLevel::Debug as u8 {
            logger.message.push_str(&format!(
                "Total bits compacted: {} (saved {} bytes)",
                saved_space,
                saved_space / 8
            ));
            logger.flush_debug().unwrap();
        }
    }

    return buffer.to_vec_u8();
}
