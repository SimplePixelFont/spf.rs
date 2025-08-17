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
pub(crate) mod table;

use crate::{String, Vec};

#[cfg(feature = "log")]
use log::*;

#[repr(u8)]
#[non_exhaustive]
#[derive(Default, Debug, Clone)]
pub enum Version {
    #[default]
    FV0,
}

#[derive(Default, Debug, Clone)]
pub struct Layout {
    version: Version,

    compact: bool,

    character_tables: Vec<CharacterTable>,
    color_tables: Vec<ColorTable>,
    pixmap_tables: Vec<PixmapTable>,
}

#[derive(Default, Debug, Clone)]
pub struct PixmapTable {
    constant_width: Option<u8>,
    constant_height: Option<u8>,
    constant_bits_per_pixel: Option<u8>,

    color_tables_indices: Option<Vec<u8>>,

    pixmaps: Vec<Pixmap>,
}

#[derive(Default, Debug, Clone)]
pub struct Pixmap {
    custom_width: Option<u8>,
    custom_height: Option<u8>,
    custom_bits_per_pixel: Option<u8>,
    data: Vec<u8>,
}

#[derive(Default, Debug, Clone)]
pub struct CharacterTable {
    use_advance_x: bool,

    constant_cluster_codepoints: Option<u8>,

    pixmap_tables_indicies: Option<Vec<u8>>,

    characters: Vec<Character>,
}

#[derive(Default, Debug, Clone)]
pub struct Character {
    advance_x: Option<u8>,

    codepoint: String,
    pixmap_index: u8,
}

#[derive(Default, Debug, Clone)]
pub struct ColorTable {
    constant_alpha: Option<u8>,

    colors: Vec<Color>,
}

#[derive(Default, Debug, Clone)]
pub struct Color {
    custom_alpha: Option<u8>,
    r: u8,
    g: u8,
    b: u8,
}

#[repr(u8)]
#[rustfmt::skip]
enum TableIdentifier {
    MappingTable = 0b00000001,
    PixmapTable  = 0b00000010,
    ColorTable   = 0b00000011,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEndOfFile,
    InvalidSignature,
    UnsupportedVersion,
}

#[derive(Debug)]
pub enum SerializeError {
    StaticVectorTooLarge,
}

pub(crate) trait Table: Sized {
    fn deserialize(storage: &mut byte::ByteStorage, layout: &Layout) -> Result<Self, ParseError>;
    fn serialize(
        &self,
        buffer: &mut byte::ByteStorage,
        layout: &Layout,
    ) -> Result<(), SerializeError>;
}

/// Parses a [`Vec<u8>`] into a font [`Layout`].
pub fn layout_from_data(buffer: Vec<u8>) -> Result<Layout, ParseError> {
    let mut storage = byte::ByteStorage {
        bytes: buffer,
        pointer: 0,
        index: 0,
    };
    let mut layout = Layout::default();

    parsers::next_signature(&mut storage)?;
    // next_signature() offsets the index by one, so we need to undo that
    storage.index -= 1;

    parsers::next_version(&mut layout, &mut storage)?;
    parsers::next_header(&mut layout, &mut storage)?;

    let mut bits_per_pixel = 1;
    if layout.header.configuration_flags.custom_bits_per_pixel {
        bits_per_pixel = layout
            .header
            .configuration_values
            .custom_bits_per_pixel
            .unwrap();
    }

    while storage.index < storage.bytes.len() - 1 {
        let mut current_character = Character::default();

        parsers::next_grapheme_cluster(&mut storage, &layout.header, &mut current_character);

        // Raises a warning if added in next_grapheme_cluster.
        storage.index += 1;

        let current_character_width =
            parsers::next_width(&mut storage, &layout.header, &mut current_character);

        let current_character_height =
            parsers::next_height(&mut storage, &layout.header, &mut current_character);

        parsers::next_pixmap(
            &mut storage,
            &layout.header,
            &mut current_character,
            current_character_width,
            current_character_height,
            bits_per_pixel,
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
