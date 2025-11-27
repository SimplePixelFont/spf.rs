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

#[cfg(not(feature = "tagging"))]
mod tagging_stub;
use core::marker::PhantomData;

#[cfg(not(feature = "tagging"))]
pub(crate) use tagging_stub::*;

#[cfg(feature = "tagging")]
pub use crate::tagging::*;

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
}

pub(crate) trait Table: Sized {
    fn deserialize<T: TagWriter>(
        engine: &mut DeserializeEngine<T>,
    ) -> Result<Self, DeserializeError>;
    fn serialize<T: TagWriter>(
        &self,
        engine: &mut SerializeEngine<T>,
    ) -> Result<(), SerializeError>;
}

pub(crate) struct DeserializeEngine<'a, T: TagWriter = TagWriterNoOp> {
    bytes: byte::ByteReader<'a>,
    layout: Layout,
    #[cfg(feature = "tagging")]
    tags: T,
    _phantom: PhantomData<T>,
}

pub(crate) struct SerializeEngine<'a, T: TagWriter = TagWriterNoOp> {
    bytes: byte::ByteWriter,
    layout: &'a Layout,
    #[cfg(feature = "tagging")]
    tags: T,
    _phantom: PhantomData<T>,
}

/// Parses a [`&[u8]`] into a font [`Layout`].
pub fn layout_from_data(buffer: &[u8]) -> Result<Layout, DeserializeError> {
    let storage = byte::ByteReader {
        bytes: buffer,
        pointer: 0,
        index: 0,
    };
    let layout = Layout::default();
    let mut engine = DeserializeEngine::<TagWriterNoOp> {
        bytes: storage,
        layout,
        #[cfg(feature = "tagging")]
        tags: TagWriterNoOp,
        _phantom: PhantomData,
    };

    deserialize::next_signature(&mut engine)?;
    deserialize::next_version(&mut engine)?;
    deserialize::next_header(&mut engine)?;

    while engine.bytes.index < engine.bytes.len() - 1 {
        match engine.bytes.next().try_into().unwrap() {
            TableIdentifier::Character => {
                let table = CharacterTable::deserialize(&mut engine)?;
                engine.layout.character_tables.push(table);
            }
            TableIdentifier::Pixmap => {
                let table = PixmapTable::deserialize(&mut engine)?;
                engine.layout.pixmap_tables.push(table);
            }
            TableIdentifier::Color => {
                let table = ColorTable::deserialize(&mut engine)?;
                engine.layout.color_tables.push(table);
            }
        };
    }
    Ok(engine.layout)
}

/// Encodes the provided font [`Layout`] into a [`Vec<u8>`].
pub fn layout_to_data(layout: Layout) -> Result<Vec<u8>, SerializeError> {
    let buffer = byte::ByteWriter::new();
    let mut engine = SerializeEngine::<TagWriterNoOp> {
        bytes: buffer,
        layout: &layout,
        #[cfg(feature = "tagging")]
        tags: TagWriterNoOp,
        _phantom: PhantomData,
    };

    serialize::push_signature(&mut engine);
    serialize::push_version(&mut engine);
    serialize::push_header(&mut engine);

    for character_table in &engine.layout.character_tables {
        character_table.serialize(&mut engine)?;
    }
    for pixmap_table in &engine.layout.pixmap_tables {
        pixmap_table.serialize(&mut engine)?;
    }
    for color_table in &engine.layout.color_tables {
        color_table.serialize(&mut engine)?;
    }

    Ok(engine.bytes.bytes)
}
