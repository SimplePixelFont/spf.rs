/*
 * Copyright 2025 SimplePixelFont
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Rust-only module to abstract, and make writing `spf.rs` code easier.

pub(crate) use crate::core::*;

use alloc::string::ToString;
use alloc::vec::Vec;

/// Magic bytes of `SimplePixelFont` files
///
/// The magic bytes can be used to determine if a file is a `SimplePixelFont` regardless of
/// the file extension. That being said the magic bytes as u8 are are follows: `102, 115, 70`.
/// In utf8 encoding this spells out `fsF`.
pub const MAGIC_BYTES: [u8; 3] = [102, 115, 70];

#[derive(Default, Debug, Clone)]
#[non_exhaustive]
/// [`LayoutBuilder`] lets you create [`Layout`]'s without all the nested structs.
pub struct LayoutBuilder {
    pub header_configuration_flags_constant_cluster_codepoints: bool,
    pub header_configuration_flags_constant_width: bool,
    pub header_configuration_flags_constant_height: bool,
    pub header_configuration_flags_custom_bits_per_pixel: bool,

    pub header_modifier_flags_compact: bool,

    pub header_configuration_values_constant_cluster_codepoints: Option<u8>,
    pub header_configuration_values_constant_width: Option<u8>,
    pub header_configuration_values_constant_height: Option<u8>,
    pub header_configuration_values_custom_bits_per_pixel: Option<u8>,

    pub body_characters: Vec<Character>,
}

impl LayoutBuilder {
    /// Sets the [`ConfigurationFlags::constant_cluster_codepoints`] field of the builder.
    pub fn constant_cluster_codepoints(
        &mut self,
        header_configuration_values_constant_cluster_codepoints: u8,
    ) -> &mut Self {
        self.header_configuration_flags_constant_cluster_codepoints = true;
        self.header_configuration_values_constant_cluster_codepoints =
            Some(header_configuration_values_constant_cluster_codepoints);

        self
    }

    /// Sets the [`ConfigurationFlags::constant_width`] field of the builder.
    pub fn constant_width(&mut self, header_configuration_values_constant_width: u8) -> &mut Self {
        self.header_configuration_flags_constant_width = true;
        self.header_configuration_values_constant_width =
            Some(header_configuration_values_constant_width);

        self
    }

    /// Sets the [`ConfigurationFlags::constant_height`] field of the builder.
    pub fn constant_height(
        &mut self,
        header_configuration_values_constant_height: u8,
    ) -> &mut Self {
        self.header_configuration_flags_constant_height = true;
        self.header_configuration_values_constant_height =
            Some(header_configuration_values_constant_height);

        self
    }

    pub fn custom_bits_per_pixel(
        &mut self,
        header_configuration_values_custom_bits_per_pixel: u8,
    ) -> &mut Self {
        self.header_configuration_flags_custom_bits_per_pixel = true;
        self.header_configuration_values_custom_bits_per_pixel =
            Some(header_configuration_values_custom_bits_per_pixel);

        self
    }

    /// Sets the [`ModifierFlags::compact`] field of the builder.
    pub fn compact(&mut self, header_modifier_flags_compact: bool) -> &mut Self {
        self.header_modifier_flags_compact = header_modifier_flags_compact;
        self
    }

    /// Pushes a new character to the [`Body::characters`] field of the builder.
    pub fn character(
        &mut self,
        character_grapheme_cluster: &'static str,
        character_custom_width: Option<u8>,
        character_custom_height: Option<u8>,
        character_pixmap: &[u8],
    ) -> &mut Self {
        self.body_characters.push(Character {
            grapheme_cluster: character_grapheme_cluster.to_string(),
            custom_width: character_custom_width,
            custom_height: character_custom_height,
            pixmap: character_pixmap.to_vec(),
        });
        self
    }

    // pub fn inffered(&mut self, character_utf8: char, character_pixmap: &[u8]) -> &mut Self {
    //     if self.header_required_values_constant_size == 0 {
    //         panic!("Constant size required to add inffered characters.");
    //     }
    //     if character_pixmap.len() % self.header_required_values_constant_size as usize != 0 {
    //         panic!("Character custom size cannot be inferred.");
    //     }

    //     let character_custom_size =
    //         (character_pixmap.len() / self.header_required_values_constant_size as usize) as u8;

    //     self.body_characters.push(Character {
    //         utf8: character_utf8,
    //         custom_size: character_custom_size,
    //         pixmap: character_pixmap.to_vec(),
    //     });
    //     self
    // }

    /// Converts the [`LayoutBuilder`] into a [`Layout`]
    pub fn build(self) -> Layout {
        Layout {
            header: Header {
                configuration_flags: ConfigurationFlags {
                    constant_cluster_codepoints: self
                        .header_configuration_flags_constant_cluster_codepoints,
                    constant_width: self.header_configuration_flags_constant_width,
                    constant_height: self.header_configuration_flags_constant_height,
                    custom_bits_per_pixel: self.header_configuration_flags_custom_bits_per_pixel,
                },
                modifier_flags: ModifierFlags {
                    compact: self.header_modifier_flags_compact,
                },
                configuration_values: ConfigurationValues {
                    constant_cluster_codepoints: self
                        .header_configuration_values_constant_cluster_codepoints,
                    constant_width: self.header_configuration_values_constant_width,
                    constant_height: self.header_configuration_values_constant_height,
                    custom_bits_per_pixel: self.header_configuration_values_custom_bits_per_pixel,
                },
            },
            body: Body {
                characters: self.body_characters,
            },
        }
    }
}
