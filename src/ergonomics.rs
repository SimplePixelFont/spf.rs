//! Rust-only module to abstract, and make writing `spf.rs` code easier.

pub(crate) use crate::core::*;

/// Magic bytes of `SimplePixelFont` files
///
/// The magic bytes can be used to determine if a file is a `SimplePixelFont` regardless of
/// the file extension. That being said the magic bytes as u8 are are follows: `102, 115, 70`.
/// In utf8 encoding this spells out `fsF`.
pub const MAGIC_BYTES: [u8; 3] = [102, 115, 70];

/// Constant for width alignment value.
pub const ALIGNMENT_WIDTH: bool = false;

/// Constant for height alignment value.
pub const ALIGNMENT_HEIGHT: bool = true;

/// [`LayoutBuilder`] lets you create [`Layout`]'s without all the nested structs.
pub struct LayoutBuilder {
    pub header_configuration_flags_alignment: bool,
    pub header_modifier_flags_compact: bool,
    pub header_required_values_constant_size: u8,
    pub body_characters: Vec<Character>,
}

impl LayoutBuilder {
    /// Creates a new [`LayoutBuilder`] which you can chain methods to.
    pub fn new() -> Self {
        Self {
            header_configuration_flags_alignment: false,
            header_modifier_flags_compact: false,
            header_required_values_constant_size: 0,
            body_characters: Vec::new(),
        }
    }

    /// Sets the [`ConfigurationFlags::alignment`] field of the builder.
    pub fn alignment(&mut self, header_configuration_flags_alignment: bool) -> &mut Self {
        self.header_configuration_flags_alignment = header_configuration_flags_alignment;
        self
    }

    /// Sets the [`ModifierFlags::compact`] field of the builder.
    pub fn compact(&mut self, header_modifier_flags_compact: bool) -> &mut Self {
        self.header_modifier_flags_compact = header_modifier_flags_compact;
        self
    }

    /// Sets the [`RequiredValues::constant_size`] field of the builder.
    pub fn size(&mut self, header_required_values_constant_size: u8) -> &mut Self {
        self.header_required_values_constant_size = header_required_values_constant_size;
        self
    }

    /// Pushes a new character to the [`Body::characters`] field of the builder.
    pub fn character(
        &mut self,
        character_utf8: char,
        character_custom_size: u8,
        character_pixmap: &[u8],
    ) -> &mut Self {
        self.body_characters.push(Character {
            utf8: character_utf8,
            custom_size: character_custom_size,
            pixmap: character_pixmap.to_vec(),
        });
        self
    }

    /// Pushes a new character with a inffered `Character::custom_size` to the [`Body::characters`]
    /// field of the builder.
    pub fn inffered(&mut self, character_utf8: char, character_pixmap: &[u8]) -> &mut Self {
        if self.header_required_values_constant_size == 0 {
            panic!("Constant size required to add inffered characters.");
        }
        if character_pixmap.len() % self.header_required_values_constant_size as usize != 0 {
            panic!("Character custom size cannot be inferred.");
        }

        let character_custom_size =
            (character_pixmap.len() / self.header_required_values_constant_size as usize) as u8;

        self.body_characters.push(Character {
            utf8: character_utf8,
            custom_size: character_custom_size,
            pixmap: character_pixmap.to_vec(),
        });
        self
    }

    /// Converts the [`LayoutBuilder`] into a [`Layout`]
    pub fn build(self) -> Layout {
        Layout {
            header: Header {
                configuration_flags: ConfigurationFlags {
                    alignment: self.header_configuration_flags_alignment,
                },
                modifier_flags: ModifierFlags {
                    compact: self.header_modifier_flags_compact,
                },
                required_values: RequiredValues {
                    constant_size: self.header_required_values_constant_size,
                },
            },
            body: Body {
                characters: self.body_characters,
            },
        }
    }
}
