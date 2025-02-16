pub(crate) use super::super::byte;
pub(crate) use super::super::common;
pub(crate) use super::super::MAGIC_BYTES;
pub(crate) use super::character::Character;

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

pub const ALIGNMENT_WIDTH: bool = false;
pub const ALIGNMENT_HEIGHT: bool = true;

/// Main structure that supports encoding and decoding with its defined methods.
#[derive(Debug, Default)]
pub struct SimplePixelFont {
    pub configurations: ConfigurationFlags,
    pub modifiers: ModifierFlags,
    pub size: u8,
    pub characters: Vec<Character>,
}

use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

impl SimplePixelFont {
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
    /// # use spf::core::SimplePixelFont;
    ///
    /// let font = SimplePixelFont::new(
    ///     ConfigurationFlags { alignment: ALIGNMENT_HEIGHT },
    ///     ModifierFlags { compact: false },
    ///     8
    /// );
    /// ```
    pub fn new(configurations: ConfigurationFlags, modifiers: ModifierFlags, size: u8) -> Self {
        return Self {
            configurations: configurations,
            modifiers: modifiers,
            size: size,
            characters: Vec::new(),
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
        if self.configurations.alignment == ALIGNMENT_HEIGHT {
            self.characters.push(character);
            return Ok(());
        } else {
            todo!();
        }
    }
    /// Encodes the structure into a `Vec<u8>` that can then be written to a file using `std::fs`
    pub fn to_vec_u8(&self) -> Vec<u8> {
        let mut buffer = byte::ByteStorage::new();
        common::sign_buffer(&mut buffer);

        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        let mut saved_space = 0;

        buffer.push(byte::Byte {
            bits: [
                self.configurations.alignment,
                false,
                false,
                false,
                self.modifiers.compact,
                false,
                false,
                false,
            ],
        });

        buffer.push(byte::Byte::from_u8(self.size));
        let mut last_write = 0;
        for character in &self.characters {
            let mut char_buffer = [0; 4];
            let mut utf8_bit_string = String::new();
            character.utf8.encode_utf8(&mut char_buffer);
            for byte in char_buffer {
                if byte != 0 {
                    byte::Byte::from_u8(byte).bits.iter().for_each(|x| {
                        if x.to_owned() {
                            utf8_bit_string.push('1');
                        } else {
                            utf8_bit_string.push('0');
                        }
                    });

                    buffer.push(byte::Byte::from_u8(byte));
                }
            }

            buffer.push(byte::Byte::from_u8(character.custom_size));
            // let mut size_bit_string = String::new();

            // byte::Byte::from_u8(character.size)
            //     .bits
            //     .iter()
            //     .for_each(|x| {
            //         if x.to_owned() {
            //             size_bit_string.push('1');
            //         } else {
            //             size_bit_string.push('0');
            //         }
            //     });

            let result = character.segment_into_u8s();

            let mut bits = vec![];
            let character_bytes = result.0;
            let used_bytes = character_bytes.len();
            let mut index = 0;
            for byte in character_bytes {
                bits.append(&mut byte::Byte::from_u8(byte).bits.to_vec());
                if self.modifiers.compact && index == used_bytes - 1 {
                    buffer.incomplete_push(byte::Byte::from_u8(byte), result.1);
                } else {
                    buffer.push(byte::Byte::from_u8(byte));
                }
                index += 1;
            }
            let test = vec![0..4, 0..2];

            let mut bbits = vec![];
            for bit in bits {
                if bit {
                    bbits.push(1);
                } else {
                    bbits.push(0);
                }
            }

            // stdout
            //     .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
            //     .unwrap();
            // write!(&mut stdout, "[ Info: ");
            // stdout.reset().unwrap();

            // write!(
            //     &mut stdout,
            //     "Added {:?} with dimensions {:?}x{:?} and the following bits: ",
            //     character.utf8, character.bitmap.width, character.bitmap.height
            // );

            // stdout
            //     .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
            //     .unwrap();

            // write!(&mut stdout, "{} {} ", utf8_bit_string, size_bit_string);

            let mut index = 0;
            let green = bbits.len() - result.1;
            for i in 0..green {
                write!(&mut stdout, "{}", bbits[i]);
                index += 1;
                if index == 8 {
                    write!(&mut stdout, " ");
                    index = 0;
                }
            }

            stdout.reset().unwrap();
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                .unwrap();
            for _ in 0..result.1 {
                write!(&mut stdout, "0");
            }
            stdout.reset().unwrap();
            writeln!(&mut stdout, "");
            if self.modifiers.compact {
                saved_space += result.1 as i32;
                buffer.pointer = ((8 - result.1) + buffer.pointer) % 8;
            }
            let mut endbuffer = vec![];
            for byte in buffer.bytes.clone() {
                for bit in byte.bits {
                    endbuffer.push(bit as u8);
                }
            }

            let mut index = 0;
            for bit in endbuffer {
                write!(&mut stdout, "{}", bit);
                index += 1;
                if index == 8 {
                    write!(&mut stdout, " ");
                    index = 0;
                }
            }
            writeln!(&mut stdout, "\n\n\n");
        }

        println!("{:?}", saved_space);

        return buffer.to_vec_u8();
    }

    /// Decodes a `Vec<u8>` and parses it into a struct, this method will ignore the checksum values.
    pub fn from_vec_u8(buffer: Vec<u8>) -> Self {
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

                current_character.utf8 = String::from_utf8(utf8_bytes.to_vec())
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap();
                current_index += 1;
                character_definition_stage += 1;
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

                println!("{:?}", current_character);
                characters.push(current_character.clone());
                current_index += 1;

                // if !(modifiers.compact && remainder != 0) {
                //     current_index += 1;
                // }
                // if (8 - remainder) as usize + body_buffer.pointer > 8 && modifiers.compact {
                //     current_index += 1;
                // }
                //
                if modifiers.compact {
                    if body_buffer.pointer + (8 - remainder) < 8 {
                        current_index -= 1;
                    }
                    body_buffer.pointer = ((8 - remainder) as usize + body_buffer.pointer) % 8;
                    println!(
                        "-{:?}, {:?} and now {:?}",
                        remainder, body_buffer.pointer, current_index
                    );
                }

                current_character.byte_map = vec![];
                character_definition_stage = 0;
            }
        }

        println!("{:?}", body_buffer);

        Self {
            configurations: configurations,
            modifiers: modifiers,
            size: size,
            characters: characters,
        }
    }
}
