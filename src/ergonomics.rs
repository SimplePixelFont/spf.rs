pub(crate) use super::core::*;

/// Magic bytes of `*.spf` files
///
/// The magic bytes can be used to determine if a file is a SimplePixelFont regardless of
/// the file extension. That being said the magic bytes as u8 are are follows: `102, 115, 70`.
/// In utf8 encoding this spells out `fsF`.
pub const MAGIC_BYTES: [u8; 3] = [102, 115, 70];
pub const ALIGNMENT_WIDTH: bool = false;
pub const ALIGNMENT_HEIGHT: bool = true;

pub struct LayoutBuilder {
    /* Header */
    pub configuration_flags_alignment: bool,
    pub modifier_flags_compact: bool,
    pub required_values_constant_size: u8,
    /* Body */
    pub characters: Vec<Character>,
}

impl LayoutBuilder {
    pub fn new() -> Self {
        Self {
            configuration_flags_alignment: false,
            modifier_flags_compact: false,
            required_values_constant_size: 0,
            characters: Vec::new(),
        }
    }
    pub fn alignment(&mut self, alignment: bool) -> &mut Self {
        self.configuration_flags_alignment = alignment;
        self
    }
    pub fn compact(&mut self, compact: bool) -> &mut Self {
        self.modifier_flags_compact = compact;
        self
    }
    pub fn size(&mut self, required_values_constant_size: u8) -> &mut Self {
        self.required_values_constant_size = required_values_constant_size;
        self
    }
    pub fn character(
        &mut self,
        character_utf8: char,
        character_custom_size: u8,
        character_byte_map: &[u8],
    ) -> &mut Self {
        self.characters.push(Character {
            utf8: character_utf8,
            custom_size: character_custom_size,
            byte_map: character_byte_map.to_vec(),
        });
        self
    }
    pub fn inffered(&mut self, character_utf8: char, character_byte_map: &[u8]) -> &mut Self {
        if self.required_values_constant_size == 0 {
            panic!("Constant size required to add inffered characters.");
        }
        if character_byte_map.len() % self.required_values_constant_size as usize != 0 {
            panic!("Character custom size cannot be inferred.");
        }

        let character_custom_size =
            (character_byte_map.len() / self.required_values_constant_size as usize) as u8;

        self.characters.push(Character {
            utf8: character_utf8,
            custom_size: character_custom_size,
            byte_map: character_byte_map.to_vec(),
        });
        self
    }
    pub fn build(self) -> Layout {
        Layout {
            header: Header {
                configuration_flags: ConfigurationFlags {
                    alignment: self.configuration_flags_alignment,
                },
                modifier_flags: ModifierFlags {
                    compact: self.modifier_flags_compact,
                },
                required_values: RequiredValues {
                    constant_size: self.required_values_constant_size,
                },
            },
            body: Body {
                characters: self.characters,
            },
        }
    }
}
