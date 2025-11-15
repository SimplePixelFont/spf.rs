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
pub(crate) mod deserialize;
pub(crate) mod serialize;
pub(crate) mod tables;

use crate::{String, Vec};

#[repr(u8)]
#[non_exhaustive]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Version {
    #[default]
    FV0 = 0b00000000,
}

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Layout {
    pub version: Version,

    pub compact: bool,

    pub character_tables: Vec<CharacterTable>,
    pub color_tables: Vec<ColorTable>,
    pub pixmap_tables: Vec<PixmapTable>,
}

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PixmapTable {
    pub constant_width: Option<u8>,
    pub constant_height: Option<u8>,
    pub constant_bits_per_pixel: Option<u8>,

    pub color_table_indexes: Option<Vec<u8>>,

    pub pixmaps: Vec<Pixmap>,
}

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pixmap {
    pub custom_width: Option<u8>,
    pub custom_height: Option<u8>,
    pub custom_bits_per_pixel: Option<u8>,
    pub data: Vec<u8>,
}

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharacterTable {
    pub use_advance_x: bool,
    pub use_pixmap_index: bool,

    pub constant_cluster_codepoints: Option<u8>,

    pub pixmap_table_indexes: Option<Vec<u8>>,

    pub characters: Vec<Character>,
}

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Character {
    pub advance_x: Option<u8>,
    pub pixmap_index: Option<u8>,

    pub grapheme_cluster: String,
}

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColorTable {
    pub constant_alpha: Option<u8>,

    pub colors: Vec<Color>,
}

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    pub custom_alpha: Option<u8>,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[repr(u8)]
#[rustfmt::skip]
enum TableIdentifier {
    Character = 0b00000001,
    Pixmap    = 0b00000010,
    Color     = 0b00000011,
}

impl TryFrom<u8> for TableIdentifier {
    type Error = DeserializeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00000001 => Ok(TableIdentifier::Character),
            0b00000010 => Ok(TableIdentifier::Pixmap),
            0b00000011 => Ok(TableIdentifier::Color),
            _ => Err(DeserializeError::UnsupportedTableIdentifier),
        }
    }
}

impl TryFrom<u8> for Version {
    type Error = DeserializeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00000000 => Ok(Version::FV0),
            _ => Err(DeserializeError::UnsupportedVersion),
        }
    }
}

#[derive(Debug)]
pub enum DeserializeError {
    UnexpectedEndOfFile,
    InvalidSignature,
    UnsupportedVersion,
    UnsupportedTableIdentifier,
}

#[derive(Debug)]
pub enum SerializeError {
    StaticVectorTooLarge,
    InvalidPixmapData,
}

pub(crate) trait Table: Sized {
    fn deserialize(
        storage: &mut byte::ByteStorage,
        layout: &Layout,
    ) -> Result<Self, DeserializeError>;
    fn serialize(
        &self,
        buffer: &mut byte::ByteStorage,
        layout: &Layout,
    ) -> Result<(), SerializeError>;
}

/// Parses a [`Vec<u8>`] into a font [`Layout`].
pub fn layout_from_data(buffer: Vec<u8>) -> Result<Layout, DeserializeError> {
    let mut storage = byte::ByteStorage {
        bytes: buffer,
        pointer: 0,
        index: 0,
    };
    let mut layout = Layout::default();

    deserialize::next_signature(&mut storage)?;
    deserialize::next_version(&mut layout, &mut storage)?;
    deserialize::next_header(&mut layout, &mut storage)?;

    while storage.index < storage.bytes.len() - 1 {
        match storage.next().try_into().unwrap() {
            TableIdentifier::Character => {
                layout
                    .character_tables
                    .push(CharacterTable::deserialize(&mut storage, &layout)?);
            }
            TableIdentifier::Pixmap => {
                layout
                    .pixmap_tables
                    .push(PixmapTable::deserialize(&mut storage, &layout)?);
            }
            TableIdentifier::Color => {
                layout
                    .color_tables
                    .push(ColorTable::deserialize(&mut storage, &layout)?);
            }
        };
    }
    Ok(layout)
}

/// Encodes the provided font [`Layout`] into a [`Vec<u8>`].
pub fn layout_to_data(layout: &Layout) -> Result<Vec<u8>, SerializeError> {
    let mut buffer = byte::ByteStorage::new();
    serialize::push_signature(&mut buffer);
    serialize::push_version(&mut buffer, &layout.version);
    serialize::push_header(&mut buffer, layout);

    for character_table in &layout.character_tables {
        character_table.serialize(&mut buffer, layout)?;
    }
    for pixmap_table in &layout.pixmap_tables {
        pixmap_table.serialize(&mut buffer, layout)?;
    }
    for color_table in &layout.color_tables {
        color_table.serialize(&mut buffer, layout)?;
    }

    Ok(buffer.bytes)
}
