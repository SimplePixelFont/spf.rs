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

#[derive(Default, Debug, Clone)]
#[non_exhaustive]
/// Defines the configuration flags for a font [`Layout`] struct.
///
/// Each field is a [`bool`] and in the binary file will be represented by a single bit.
pub struct ConfigurationFlags {
    pub constant_cluster_codepoints: bool,
    pub constant_width: bool,
    pub constant_height: bool,
}

#[derive(Default, Debug, Clone)]
#[non_exhaustive]
/// Defines the modifier flags for a font [`Layout`] struct.
///
/// If the field is set to true, then the modifer will be applied to the font [`Layout`] struct.
/// Each field is a [`bool`] and in the binary file will be represented by a single bit.
pub struct ModifierFlags {
    /// If enabled (value set to true), font body will be compacted, removing padding bytes after each character definition. Without compact enabled, [`layout_to_data`] will end each character bitmap with padding 0's if `(constant_size * custom_size) % 8` results in a remainder that is not 0. The number of padding 0's is the remainder of the formula above.
    pub compact: bool,
}

#[derive(Default, Debug, Clone)]
#[non_exhaustive]
/// Defines the required values for a [`Layout`] structs.
pub struct ConfigurationValues {
    /// Sets a constant number of utf8 encoded codepoints
    /// that will be used for each grapheme cluster within a character definition.
    pub constant_cluster_codepoints: Option<u8>,
    pub constant_width: Option<u8>,
    pub constant_height: Option<u8>,
}

#[derive(Default, Debug, Clone)]
/// Represents the header of a font [`Layout`] struct.
///
/// The [`Header`] struct contains the configuration flags, modifier flags and required values
/// of a [`Layout`]. These values are essential in determining how the font will be interpreted
/// by [`layout_to_data`] and [`layout_from_data`] functions.
pub struct Header {
    pub configuration_flags: ConfigurationFlags,
    pub modifier_flags: ModifierFlags,
    pub configuration_values: ConfigurationValues,
}

#[derive(Default, Debug, Clone)]
/// Represents a charater in the font.
///
/// The [`Character`] struct contains the utf8 character, custom size and byte map of a character.
/// Please note that while the pixmap uses a u8 for each pixel, when the font is converted to
/// a data vector, each pixel will be represented by a single bit.
pub struct Character {
    pub grapheme_cluster: String,
    pub custom_width: Option<u8>,
    pub custom_height: Option<u8>,
    pub pixmap: Vec<u8>,
}

#[derive(Default, Debug, Clone)]
/// Represents the body of a font [`Layout`] struct.
///
/// The [`Body`] struct contains the characters of a [`Layout`] as a Vector.
pub struct Body {
    pub characters: Vec<Character>,
}

#[derive(Default, Debug, Clone)]
/// Represents the entire font [`Layout`] struct.
///
/// The [`Layout`] struct aims to reflect the structure of a `SimplePixelFont` binary file.
pub struct Layout {
    pub header: Header,
    pub body: Body,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEndOfFile,
}

/// Parses a [`Vec<u8>`] into a font [`Layout`].
pub fn layout_from_data(buffer: Vec<u8>) -> Result<Layout, ParseError> {
    let mut current_index = 0;

    let mut chunks = buffer.iter();

    let mut layout = Layout::default();

    let mut current_configuration_flag_index = 0;
    let mut configuration_flag_booleans = [false; 4];
    let mut configuration_flag_values = [None; 4];

    let mut character_definition_stage = 0;
    let mut current_character = Character::default();
    let mut current_character_width = 0;
    let mut current_character_height = 0;

    let mut body_buffer = byte::ByteStorage::new();

    let mut iter = chunks.next();
    while !iter.is_none() {
        let chunk = iter.unwrap();

        match current_index {
            0..3 => {
                if !chunk == [102, 115, 70][current_index] {
                    panic!("File is not signed")
                }
            }
            3 => {
                let file_properties = byte::Byte::from_u8(chunk.clone()).bits;

                configuration_flag_booleans = [
                    file_properties[0],
                    file_properties[1],
                    file_properties[2],
                    file_properties[3],
                ];

                layout.header.modifier_flags.compact = file_properties[4];
            }
            _ => {
                for index in current_configuration_flag_index..4 {
                    current_configuration_flag_index = index + 1;
                    if configuration_flag_booleans[index] {
                        configuration_flag_values[index] = Some(chunk.clone());
                        break;
                    }
                }
                if current_configuration_flag_index == 4 {
                    body_buffer.push(byte::Byte::from_u8(chunk.clone()));
                }
            }
        }
        iter = chunks.next();
        current_index += 1;
    }

    layout
        .header
        .configuration_flags
        .constant_cluster_codepoints = configuration_flag_booleans[0];
    layout.header.configuration_flags.constant_width = configuration_flag_booleans[1];
    layout.header.configuration_flags.constant_height = configuration_flag_booleans[2];
    //layout.header.configuration_flags.custom_bits_per_pixel = configuration_flag_booleans[3];

    layout
        .header
        .configuration_values
        .constant_cluster_codepoints = configuration_flag_values[0];
    layout.header.configuration_values.constant_width = configuration_flag_values[1];
    layout.header.configuration_values.constant_height = configuration_flag_values[2];
    //layout.header.configuration_values.custom_bits_per_pixel = configuration_flag_values[3];

    current_index = 0;
    let length = body_buffer.bytes.len();
    while current_index < length - 1 {
        if character_definition_stage == 0 {
            let result =
                parsers::next_grapheme_cluster(&mut body_buffer, &layout.header, current_index);
            current_character.grapheme_cluster = result.0;
            current_index = result.1;
            current_index += 1;
            character_definition_stage += 1;

            #[cfg(feature = "log")]
            unsafe {
                let mut logger = LOGGER.lock().unwrap();
                if logger.log_level as u8 >= LogLevel::Debug as u8 {
                    logger.message.push_str(&format!(
                        "Identified grapheme cluster: {:?}",
                        current_character.grapheme_cluster
                    ));
                    logger.flush_debug().unwrap();
                }
            }
        }

        if character_definition_stage == 1 {
            if !layout.header.configuration_flags.constant_width {
                current_character.custom_width = Some(body_buffer.get(current_index).to_u8());
                current_character_width = current_character.custom_width.unwrap();
                current_index += 1;
            } else {
                current_character_width =
                    layout.header.configuration_values.constant_width.unwrap();
            }
            character_definition_stage += 1;
        }

        if character_definition_stage == 2 {
            if !layout.header.configuration_flags.constant_height {
                current_character.custom_height = Some(body_buffer.get(current_index).to_u8());
                current_character_height = current_character.custom_height.unwrap();
                current_index += 1;
            } else {
                current_character_height =
                    layout.header.configuration_values.constant_height.unwrap();
            }
            character_definition_stage += 1;
        }

        if character_definition_stage == 3 {
            let bytes_used = (((current_character_width as f32 * current_character_height as f32)
                as f32
                / 8.0) as f32)
                .ceil() as u8;

            let remainder = bytes_used as usize * 8 as usize
                - (current_character_width as usize * current_character_height as usize);

            let mut current_byte = body_buffer.get(current_index);
            for i in 0..bytes_used {
                let mut counter = 0;
                for bit in current_byte.bits {
                    if !(i == bytes_used - 1 && counter >= 8 - remainder) {
                        current_character.pixmap.push(bit as u8);
                    }
                    counter += 1;
                }

                if i < bytes_used - 1 {
                    current_index += 1;
                    current_byte = body_buffer.get(current_index);
                }
            }

            layout.body.characters.push(current_character.clone());
            current_index += 1;

            if layout.header.modifier_flags.compact {
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

            current_character.pixmap = vec![];
            character_definition_stage = 0;
        }
    }
    Ok(layout)
}

/// Encodes the provided font [`Layout`] into a [`Vec<u8>`].
pub fn layout_to_data(layout: &Layout) -> Vec<u8> {
    let mut buffer = byte::ByteStorage::new();
    helpers::sign_buffer(&mut buffer);
    composers::push_header(&mut buffer, &layout.header);

    let mut saved_space = 0;

    for character in &layout.body.characters {
        composers::push_grapheme_cluster(&mut buffer, &layout.header, &character.grapheme_cluster);
        composers::push_width(&mut buffer, &layout.header, character.custom_width);
        composers::push_height(&mut buffer, &layout.header, character.custom_height);

        let result = helpers::character_pixmap_to_data(&character);
        let character_bytes = result.0;
        let remaining_space = result.1;

        composers::push_pixmap(
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
