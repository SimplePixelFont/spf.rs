pub(crate) use super::super::byte;
pub(crate) use super::super::common;
pub(crate) use super::super::MAGIC_BYTES;
pub(crate) use super::Character;

#[cfg(feature = "log")]
use super::super::log::{LogLevel, LOGGER};

pub const ALIGNMENT_WIDTH: bool = false;
pub const ALIGNMENT_HEIGHT: bool = true;

#[derive(Debug, Default)]
/// Defines the configuration flags for a `SimplePixelFont` structs.
///
/// Each field is a boolean, however you many use the constants defined in this module
/// to set the values of the fields. (such as `ALIGNMENT_HEIGHT` or `ALIGNMENT_WIDTH` for the
/// `alignment` field to increase readability).
pub struct ConfigurationFlags {
    /// Determines if the font characters are alligned by width (false) or height (true).
    pub alignment: bool,
}
#[derive(Debug, Default)]
/// Defines the modifier flags for a `SimplePixelFont` structs.
///
/// If the field is set to true, then the modifer will be applied to the `SimplePixelFont` struct.
pub struct ModifierFlags {
    /// If enabled, font body will be compacted with no padding bytes.
    pub compact: bool,
}

#[derive(Debug)]
pub struct RequiredValues {
    pub constant_size: u8,
}

#[derive(Debug)]
pub struct Header {
    pub configuration_flags: ConfigurationFlags,
    pub modifier_flags: ModifierFlags,
    pub required_values: RequiredValues,
}

#[derive(Debug)]
pub struct Body {
    pub characters: Vec<Character>,
}

#[derive(Debug)]
pub struct Layout {
    pub header: Header,
    pub body: Body,
}

impl Layout {
    /// Creates a new `SimplePixelFont` struct with the header properties.
    ///
    /// This function will return a `SimplePixelFont` struct with its format version,
    /// character alignment, and size, The struct will have no characters defined, you
    /// may use the `add_character` method to add characters to the struct.
    ///
    /// # Example
    /// ```
    /// # use spf::core::ConfigurationFlags;
    /// # use spf::core::ModifierFlags;
    /// # use spf::core::ALIGNMENT_HEIGHT;
    /// # use spf::core::Layout;
    ///
    /// let font = Layout::new(
    ///     ConfigurationFlags { alignment: ALIGNMENT_HEIGHT },
    ///     ModifierFlags { compact: false },
    ///     RequiredValues { constant_size: 8 },
    /// );
    /// ```
    pub fn new(header: Header, body: Body) -> Self {
        return Self {
            header: header,
            body: body,
        };
    }
    /// Adds a new character to the `SimplePixelFont` struct.
    ///
    /// This method will automatically handle both inffered and non-infferred
    /// characters and set their appropiate dimensions if possible (for inffered characters).
    /// If the method fails to add character an error will be returned and character will
    /// not be added. If `cache` feature is enabled, this method will also add the character
    /// to the `cache` HashMap field.
    pub fn add_character(&mut self, character: Character) -> Result<(), String> {
        if self.header.configuration_flags.alignment == ALIGNMENT_HEIGHT {
            self.body.characters.push(character);
            return Ok(());
        } else {
            todo!();
        }
    }
    /// Decodes a `Vec<u8>` and parses it into a struct, this method will ignore the checksum values.
    pub fn from_data(buffer: Vec<u8>) -> Self {
        let mut current_index = 0;
        let mut chunks = buffer.iter();

        let mut configurations: ConfigurationFlags = ConfigurationFlags { alignment: false };
        let mut modifiers: ModifierFlags = ModifierFlags { compact: false };
        let mut size: u8 = 0;
        let mut characters: Vec<Character> = Vec::new();
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
                if !chunk == MAGIC_BYTES[current_index] {
                    panic!("File is not signed")
                }
            } else if current_index == 3 {
                let file_properties = byte::Byte::from_u8(chunk.clone()).bits;
                configurations.alignment = file_properties[0];
                modifiers.compact = file_properties[4];
            } else if current_index == 4 {
                size = chunk.clone();
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
                current_character.utf8 = common::next_character(&mut body_buffer, current_index);
                current_index += 1;
                character_definition_stage += 1;

                #[cfg(feature = "log")]
                unsafe {
                    let mut logger = LOGGER.lock().unwrap();
                    if logger.log_level as u8 >= LogLevel::Debug as u8 {
                        logger.message.push_str(
                            format!("Identified utf8 character: {:?}", current_character.utf8)
                                .as_str(),
                        );
                        logger.flush_debug().unwrap();
                    }
                }
            }
            if character_definition_stage == 1 {
                current_character.custom_size = body_buffer.get(current_index).to_u8();
                current_index += 1;
                character_definition_stage += 1
            }
            if character_definition_stage == 2 {
                let bytes_used = (((current_character.custom_size as f32 * size as f32) as f32
                    / 8.0) as f32)
                    .ceil() as u8;

                let remainder = bytes_used as usize * 8 as usize
                    - (current_character.custom_size as usize * size as usize);

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

                //println!("{:?}", current_character);
                characters.push(current_character.clone());
                current_index += 1;

                if modifiers.compact {
                    if body_buffer.pointer + (8 - remainder) < 8 {
                        current_index -= 1;
                    }
                    body_buffer.pointer = ((8 - remainder) as usize + body_buffer.pointer) % 8;
                    // println!(
                    //     "-{:?}, {:?} and now {:?}",
                    //     remainder, body_buffer.pointer, current_index
                    // );
                }

                current_character.byte_map = vec![];
                character_definition_stage = 0;
            }
        }

        //println!("{:?}", body_buffer);

        Self {
            header: Header {
                configuration_flags: configurations,
                modifier_flags: modifiers,
                required_values: RequiredValues {
                    constant_size: size,
                },
            },
            body: Body {
                characters: characters,
            },
        }
    }
    /// Encodes the structure into a `Vec<u8>` that can then be written to a file using `std::fs`
    pub fn to_data(&self) -> Vec<u8> {
        let mut buffer = byte::ByteStorage::new();
        common::sign_buffer(&mut buffer);
        common::push_header(&mut buffer, &self.header);

        let mut saved_space = 0;

        for character in &self.body.characters {
            common::push_character(&mut buffer, character.utf8);
            common::push_custom_size(&mut buffer, character.custom_size);

            let result = character.segment_into_u8s();
            let character_bytes = result.0;
            let remaining_space = result.1;

            common::push_byte_map(&mut buffer, &self.header, character_bytes, remaining_space);

            if self.header.modifier_flags.compact {
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
}
