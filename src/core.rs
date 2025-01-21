pub(crate) use super::byte;
pub(crate) use super::MAGIC_BYTES;

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

/// Represents a bitmap for a character in your font.
/// Note: This is a one dimensional vector, you can use the `get_pixel()` method to get a two dimensional-like interface.
/// Note: Only the first `width * height` items are used, the rest are ignored when encoding and decoding from/to a `Vec<u8>`
#[derive(Debug, Clone)]
pub struct Bitmap {
    pub width: u8,
    pub height: u8,
    pub data: Vec<bool>,
    inferred: bool,
}

impl Bitmap {
    /// Creates a standard non-inferred `Bitmap` struct with all fields.
    ///
    /// This function is provided to create a `Bitmap` for characters providing all
    /// fields; width, height, and data. The `Bitmap` returned will have the inffered
    /// field set to false and can also be used within the `add_character` method of a
    /// `SimplePixelFont` struct. Keep in mind that this function requires a `Vec<u8>`
    /// for the data field instead of a `&[u8]` like the `Bitmap::inferred()` function.
    ///
    /// # Example:
    /// ```
    /// # use spf::core::Bitmap;
    /// let bitmap = Bitmap::new(4, 4, vec![
    ///     false, false, false, false,
    ///     false, true, true, false,
    ///     false, true, true, false,
    ///     false, false, false, false
    /// ]).unwrap();
    ///
    /// assert_eq!(bitmap.is_inferred(), false);
    pub fn new(width: u8, height: u8, data: Vec<bool>) -> Result<Self, String> {
        if width as usize * height as usize == data.len() {
            return Ok(Self {
                width: width,
                height: height,
                data: data,
                inferred: false,
            });
        } else {
            return Err("Bitmap width*height does not equal data.len()!".to_string());
        }
    }
    /// Creates an inferred `Bitmap` struct which dimensions are unknown.
    ///
    /// This function is provided to make creating bitmaps for character much easier.
    /// Rather then providing the width and height, this Bitmap will automatically choose
    /// the right dimensions for the character bitmap depending on the `SimplePixelFont`
    /// struct `alignment`, and `size` fields. As such it is advised to use only inferred
    /// `Bitmap`'s when you use the `unchecked_add_character` or `add_character` methods of
    /// a `SimplePixelFont`
    ///
    /// # Example
    /// ```
    /// # use spf::core::Bitmap;
    /// # use spf::core::SimplePixelFont;
    /// # use spf::core::Character;
    /// # use spf::core::ConfigurationFlags;
    /// # use spf::core::ModifierFlags;
    /// # use spf::core::ALIGNMENT_HEIGHT;
    ///
    /// let mut font = SimplePixelFont::new(
    ///     ConfigurationFlags { alignment: ALIGNMENT_HEIGHT },
    ///     ModifierFlags { compact: false },
    ///     4
    /// );
    /// font.add_character(Character::inferred('o', Bitmap::inferred(&[
    ///     false, true, true, false,
    ///     true, false, false, true,
    ///     true, false, false, true,
    ///     false, true, true, false
    /// ])));
    /// ```
    pub fn inferred(data: &[bool]) -> Self {
        Self {
            width: 0,
            height: 0,
            data: data.to_owned(),
            inferred: true,
        }
    }
    /// Returns a boolean depending if the Bitmap is inferred or not.
    ///
    /// Inferred Bitmap's can only be used when creating inferred characters.
    pub fn is_inferred(&self) -> bool {
        return self.inferred;
    }
    pub(crate) fn segment_into_u8s(&self) -> (Vec<u8>, usize) {
        let mut chunks = self.data.chunks(8);
        let mut buffer: Vec<u8> = Vec::new();
        let mut remainder = 0;

        let mut iter = chunks.next();
        while !iter.is_none() {
            let chunk = iter.unwrap();
            remainder = 8 - chunk.len();
            let mut byte = byte::Byte { bits: [false; 8] };
            let mut index: usize = 0;
            for pixel in chunk {
                byte.bits[index] = *pixel;
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
/// Represents a charater in the font.
#[derive(Clone, Debug)]
pub struct Character {
    pub utf8: char,
    pub size: u8,
    pub bitmap: Bitmap,
}

impl Character {
    pub fn new(utf8: char, size: u8, bitmap: Bitmap) -> Result<Self, String> {
        if !bitmap.is_inferred() {
            Ok(Self {
                utf8: utf8,
                size: size,
                bitmap: bitmap,
            })
        } else {
            Err("Bitmap provided is inferred, use Character::inferred() instead!".to_string())
        }
    }
    pub fn inferred(utf8: char, bitmap: Bitmap) -> Self {
        if bitmap.is_inferred() {
            return Self {
                utf8: utf8,
                size: 0,
                bitmap: bitmap,
            };
        }
        panic!("Not an inferred bitmap.")
    }
}
/// Main structure that supports encoding and decoding with its defined methods.
#[derive(Debug, Default)]
pub struct SimplePixelFont {
    pub configurations: ConfigurationFlags,
    pub modifiers: ModifierFlags,
    pub size: u8,
    pub characters: Vec<Character>,
}

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
        if character.bitmap.is_inferred() {
            if self.configurations.alignment == ALIGNMENT_HEIGHT {
                let remainder = (character.bitmap.data.len() as u16 % self.size as u16) as u8;
                if remainder == 0 {
                    let width = (character.bitmap.data.len() as u16 / self.size as u16) as u8;
                    self.characters.push(
                        Character::new(
                            character.utf8,
                            width,
                            Bitmap::new(width, self.size, character.bitmap.data).unwrap(),
                        )
                        .unwrap(),
                    );
                    return Ok(());
                } else {
                    return Err("Character's bitmap dimensions cannot be inffered.".to_string());
                }
            } else {
                todo!();
            }
        } else {
            if self.configurations.alignment == ALIGNMENT_HEIGHT {
                self.characters.push(
                    Character::new(character.utf8, character.bitmap.width, character.bitmap)
                        .unwrap(),
                );
                return Ok(());
            } else {
                todo!();
            }
        }
    }
    /// Encodes the structure into a `Vec<u8>` that can then be written to a file using `std::fs`
    pub fn to_vec_u8(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.push(102);
        buffer.push(115);
        buffer.push(70);

        buffer.push(
            byte::Byte {
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
            }
            .to_u8(),
        );

        buffer.push(self.size);
        let mut body_buffer = byte::ByteStorage::new();

        for character in &self.characters {
            let mut char_buffer = [0; 4];
            character.utf8.encode_utf8(&mut char_buffer);
            for byte in char_buffer {
                if byte != 0 {
                    body_buffer.push(byte::Byte::from_u8(byte));
                }
            }

            body_buffer.push(byte::Byte::from_u8(character.size));

            let result = character.bitmap.segment_into_u8s();
            let character_buffer = result.0;
            for buffer in character_buffer {
                body_buffer.push(byte::Byte::from_u8(buffer));
            }
            if self.modifiers.compact {
                body_buffer.pointer = (body_buffer.pointer + result.1) % 8;
            }
        }
        buffer.append(&mut body_buffer.to_vec_u8());
        return buffer;
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
            size: 0,
            bitmap: Bitmap {
                width: 0,
                height: 0,
                data: vec![],
                inferred: false,
            },
        };

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
                if character_definition_stage == 0 {
                    let utf81 = byte::Byte::from_u8(chunk.clone());
                    let mut utf8_bytes: [u8; 4] = [0, 0, 0, 0];

                    if utf81.bits[7..] == [false] {
                        utf8_bytes[0] = utf81.to_u8();
                    } else if utf81.bits[5..] == [false, true, true, true] {
                        utf8_bytes[0] = utf81.to_u8();
                        iter = chunks.next();
                        utf8_bytes[1] = iter.unwrap().clone();
                    } else if utf81.bits[4..] == [false, true, true, true] {
                        utf8_bytes[0] = utf81.to_u8();
                        iter = chunks.next();
                        utf8_bytes[1] = iter.unwrap().clone();
                        iter = chunks.next();
                        utf8_bytes[2] = iter.unwrap().clone();
                    } else if utf81.bits[3..] == [false, true, true, true, true] {
                        utf8_bytes[0] = utf81.to_u8();
                        iter = chunks.next();
                        utf8_bytes[1] = iter.unwrap().clone();
                        iter = chunks.next();
                        utf8_bytes[2] = iter.unwrap().clone();
                        iter = chunks.next();
                        utf8_bytes[3] = iter.unwrap().clone();
                    }

                    current_character.utf8 = String::from_utf8(utf8_bytes.to_vec())
                        .unwrap()
                        .chars()
                        .next()
                        .unwrap();
                    character_definition_stage += 1;
                } else if character_definition_stage == 1 {
                    current_character.size = chunk.clone();
                    character_definition_stage += 1
                } else if character_definition_stage == 2 {
                    if configurations.alignment == ALIGNMENT_HEIGHT {
                        current_character.bitmap.height = size;
                        current_character.bitmap.width = current_character.size;
                    } else {
                        current_character.bitmap.height = current_character.size;
                        current_character.bitmap.width = size;
                    }

                    let bytes_used = (((current_character.bitmap.height as f32
                        * current_character.bitmap.width as f32)
                        as f32
                        / 8.0) as f32)
                        .ceil() as u8;

                    let remainder = bytes_used as i32 * 8 as i32
                        - (current_character.bitmap.height * current_character.bitmap.width) as i32;

                    let mut current_byte = byte::Byte::from_u8(iter.unwrap().clone());
                    for i in 0..bytes_used {
                        let mut counter = 0;
                        for bit in current_byte.bits {
                            if !(i == bytes_used - 1 && counter >= 8 - remainder) {
                                current_character.bitmap.data.push(bit);
                            }
                            counter += 1;
                        }

                        if i != bytes_used - 1 {
                            iter = chunks.next();
                            current_byte = byte::Byte::from_u8(iter.unwrap().clone());
                        }
                    }

                    characters.push(current_character.clone());
                    current_character.bitmap.data = vec![];
                    character_definition_stage = 0;
                }
            }
            iter = chunks.next();
            current_index += 1;
        }

        Self {
            configurations: configurations,
            modifiers: modifiers,
            size: size,
            characters: characters,
        }
    }
}
