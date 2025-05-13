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
pub(crate) mod parsers;

use alloc::string::String;
use alloc::vec::Vec;
use log::*;

#[derive(Default, Debug, Clone)]
#[non_exhaustive]
/// Defines the configuration flags for a font [`Layout`] struct.
///
/// Each field is a [`bool`] and in the binary file will be represented by a single bit.
pub struct ConfigurationFlags {
    pub constant_cluster_codepoints: bool,
    pub constant_width: bool,
    pub constant_height: bool,
    pub custom_bits_per_pixel: bool,
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
    pub custom_bits_per_pixel: Option<u8>,
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

    let mut storage = byte::ByteStorage {
        bytes: buffer,
        pointer: 0,
    };

    let mut layout = Layout::default();

    current_index = parsers::next_signature(&storage, current_index);
    current_index = parsers::next_header(&mut layout, &storage, current_index);

    let mut bits_per_pixel = 1;
    if layout.header.configuration_flags.custom_bits_per_pixel {
        bits_per_pixel = layout
            .header
            .configuration_values
            .custom_bits_per_pixel
            .unwrap();
    }

    while current_index < storage.bytes.len() - 1 {
        let mut current_character = Character::default();

        current_index = parsers::next_grapheme_cluster(
            &storage,
            &layout.header,
            &mut current_character,
            current_index,
        );

        // Raises a warning if added in next_grapheme_cluster.
        current_index += 1;

        let result = parsers::next_width(
            &storage,
            &layout.header,
            &mut current_character,
            current_index,
        );
        let current_character_width = result.0;
        current_index = result.1;

        let result = parsers::next_height(
            &storage,
            &layout.header,
            &mut current_character,
            current_index,
        );
        let current_character_height = result.0;
        current_index = result.1;

        current_index = parsers::next_pixmap(
            &mut storage,
            &layout.header,
            &mut current_character,
            current_character_width,
            current_character_height,
            bits_per_pixel,
            current_index,
        );

        layout.body.characters.push(current_character.clone());
    }
    Ok(layout)
}

/// Encodes the provided font [`Layout`] into a [`Vec<u8>`].
pub fn layout_to_data(layout: &Layout) -> Vec<u8> {
    let mut buffer = byte::ByteStorage::new();
    composers::push_signature(&mut buffer);
    composers::push_header(&mut buffer, &layout.header);

    // let mut saved_space = 0;

    for character in &layout.body.characters {
        composers::push_grapheme_cluster(&mut buffer, &layout.header, &character.grapheme_cluster);
        composers::push_width(&mut buffer, &layout.header, character.custom_width);
        composers::push_height(&mut buffer, &layout.header, character.custom_height);

        composers::push_pixmap(&mut buffer, &layout.header, &character.pixmap);

        // if layout.header.modifier_flags.compact {
        //     saved_space += remaining_space;
        //     buffer.pointer = ((8 - remaining_space) + buffer.pointer) % 8;
        // }
    }

    // #[cfg(feature = "log")]
    // debug!(
    //     "Total bits compacted: {} (saved {} bytes)",
    //     saved_space,
    //     saved_space / 8
    // );

    buffer.bytes
}
