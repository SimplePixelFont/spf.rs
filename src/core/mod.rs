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
//! This module provides raw composite structs that aim to reflect the structure of a `SimplePixelFont`
//! binary file. Additionally it defines the [`layout_to_data`] and [`layout_from_data`] functions that
//! can be used to convert between the structs and the binary data.

pub mod byte;
pub(crate) mod deserialize;
pub(crate) mod serialize;
pub(crate) mod tables;

use bitflags::bitflags;
use byte::{ByteReader, ByteReaderImpl};

#[cfg(not(feature = "tagging"))]
mod tagging_stub;

#[cfg(feature = "tagging")]
pub(crate) use crate::tagging::*;
#[cfg(not(feature = "tagging"))]
pub(crate) use tagging_stub::*;

use crate::{String, Vec};
use core::marker::PhantomData;

bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct PixmapTableConfigurationFlags: u8 {
        #[doc = include_str!("../../res/snippets/pixmap_table/configurations/flag/use_constant_width.md")]
        const ConstantWidth = 0b00000001;
        #[doc = include_str!("../../res/snippets/pixmap_table/configurations/flag/use_constant_height.md")]
        const ConstantHeight = 0b00000010;
        #[doc = include_str!("../../res/snippets/pixmap_table/configurations/flag/use_constant_bits_per_pixel.md")]
        const ConstantBitsPerPixel = 0b00000100;
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct PixmapTableLinkFlags: u8 {
        #[doc = include_str!("../../res/snippets/pixmap_table/links/flag/link_color_tables.md")]
        const LinkColorTables = 0b00000001;
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct CharacterTableModifierFlags: u8 {
        const UseAdvanceX = 0b00000001;
        const UsePixmapIndex = 0b00000010;
        const UsePixmapTableIndex = 0b00000100;
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct CharacterTableLinkFlags: u8 {
        #[doc = include_str!("../../res/snippets/character_table/links/flag/link_pixmap_tables.md")]
        const LinkPixmapTables = 0b00000001;
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct CharacterTableConfigurationFlags: u8 {
        const ConstantCodePointCount = 0b00000001;
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct ColorTableModifierFlags: u8 {
        const UseColorType = 0b00000001;
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct ColorTableConfigurationFlags: u8 {
        #[doc = include_str!("../../res/snippets/color_table/configurations/flag/use_constant_alpha.md")]
        const ConstantAlpha = 0b00000001;
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct FontTableLinkFlags: u8 {
        #[doc = include_str!("../../res/snippets/font_table/links/flag/link_character_tables.md")]
        const LinkCharacterTables = 0b00000001;
    }
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Version {
    #[default]
    FV0 = 0b00000000,
}

impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let version = *self as u8;
        write!(f, "FV{:b}", version)
    }
}

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Layout {
    pub version: Version,

    pub compact: bool,

    pub character_tables: Vec<CharacterTable>,
    pub color_tables: Vec<ColorTable>,
    pub pixmap_tables: Vec<PixmapTable>,
    pub font_tables: Vec<FontTable>,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[doc = include_str!("../../res/snippets/pixmap_table/brief.md")]
pub struct PixmapTable {
    pub configuration_flags: PixmapTableConfigurationFlags,
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/condition/constant_bits_per_pixel.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/brief/constant_bits_per_pixel.md")]
    pub constant_width: Option<u8>,
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/condition/constant_height.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/brief/constant_height.md")]
    pub constant_height: Option<u8>,
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/condition/constant_bits_per_pixel.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/brief/constant_bits_per_pixel.md")]
    pub constant_bits_per_pixel: Option<u8>,

    pub link_flags: PixmapTableLinkFlags,
    #[doc = include_str!("../../res/snippets/pixmap_table/links/condition/color_tables.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/links/brief/color_tables.md")]
    pub color_table_indexes: Option<Vec<u8>>,

    pub pixmaps: Vec<Pixmap>,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pixmap {
    #[doc = include_str!("../../res/snippets/pixmap_table/records/condition/width.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/records/brief/width.md")]
    pub custom_width: Option<u8>,
    #[doc = include_str!("../../res/snippets/pixmap_table/records/condition/height.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/records/brief/height.md")]
    pub custom_height: Option<u8>,
    #[doc = include_str!("../../res/snippets/pixmap_table/records/condition/bits_per_pixel.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/records/brief/bits_per_pixel.md")]
    pub custom_bits_per_pixel: Option<u8>,
    #[doc = include_str!("../../res/snippets/pixmap_table/records/condition/data.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/records/brief/data.md")]
    pub data: Vec<u8>,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharacterTable {
    pub modifier_flags: CharacterTableModifierFlags,

    pub configuration_flags: CharacterTableConfigurationFlags,
    pub constant_code_point_count: Option<u8>,

    pub link_flags: CharacterTableLinkFlags,
    pub pixmap_table_indexes: Option<Vec<u8>>,

    pub characters: Vec<Character>,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Character {
    pub advance_x: Option<u8>,
    pub pixmap_index: Option<u8>,
    pub pixmap_table_index: Option<u8>,

    pub code_points: String,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColorTable {
    pub modifier_flags: ColorTableModifierFlags,

    pub configuration_flags: ColorTableConfigurationFlags,
    pub constant_alpha: Option<u8>,

    pub colors: Vec<Color>,
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ColorType {
    #[default]
    Dynamic,
    Absolute,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    pub color_type: Option<ColorType>,
    pub custom_alpha: Option<u8>,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FontType {
    #[default]
    Regular,
    Bold,
    Italic,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FontTable {
    pub link_flags: FontTableLinkFlags,
    pub character_table_indexes: Option<Vec<u8>>,

    pub fonts: Vec<Font>,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Font {
    pub name: String,
    pub author: String,
    pub version: u8,
    pub font_type: FontType,
    pub character_table_indexes: Vec<u8>,
}

#[repr(u8)]
#[rustfmt::skip]
enum TableIdentifier {
    Character = 0b00000001,
    Pixmap    = 0b00000010,
    Color     = 0b00000011,
    Font      = 0b00000100,
}

impl TryFrom<u8> for TableIdentifier {
    type Error = DeserializeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00000001 => Ok(TableIdentifier::Character),
            0b00000010 => Ok(TableIdentifier::Pixmap),
            0b00000011 => Ok(TableIdentifier::Color),
            0b00000100 => Ok(TableIdentifier::Font),
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

impl TryFrom<u8> for ColorType {
    type Error = DeserializeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ColorType::Dynamic),
            1 => Ok(ColorType::Absolute),
            _ => Err(DeserializeError::UnsupportedColorType),
        }
    }
}

impl TryFrom<u8> for FontType {
    type Error = DeserializeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FontType::Regular),
            1 => Ok(FontType::Bold),
            2 => Ok(FontType::Italic),
            _ => Err(DeserializeError::UnsupportedFontType),
        }
    }
}

#[derive(Debug)]
pub enum DeserializeError {
    UnexpectedEndOfFile,
    InvalidSignature,
    UnsupportedVersion,
    UnsupportedColorType,
    UnsupportedTableIdentifier,
    UnsupportedFontType,
}

#[derive(Debug)]
pub enum SerializeError {
    StaticVectorTooLarge,
    InvalidPixmapData,
}

pub(crate) trait Table: Sized {
    fn deserialize<R: ByteReader, T: TagWriter>(
        engine: &mut DeserializeEngine<R, T>,
    ) -> Result<Self, DeserializeError>;
    fn serialize<T: TagWriter>(
        &self,
        engine: &mut SerializeEngine<T>,
    ) -> Result<(), SerializeError>;
}

pub struct DeserializeEngine<'a, R: ByteReader = ByteReaderImpl<'a>, T: TagWriter = TagWriterNoOp> {
    bytes: R,
    pub layout: Layout,
    #[cfg(feature = "tagging")]
    pub tags: T,
    #[cfg(feature = "tagging")]
    tagging_data: TaggingData,
    _phantom: PhantomData<T>,
    _phantom2: &'a PhantomData<R>,
}

#[derive(Default)]
pub(crate) struct TaggingData {
    current_table_index: u8,
    current_record_index: u8,
}

pub struct SerializeEngine<'a, T: TagWriter = TagWriterNoOp> {
    bytes: byte::ByteWriter,
    pub layout: &'a Layout,
    #[cfg(feature = "tagging")]
    pub tags: T,
    #[cfg(feature = "tagging")]
    tagging_data: TaggingData,
    _phantom: PhantomData<T>,
}

pub(crate) fn deserialize_layout<R: ByteReader, T: TagWriter>(
    engine: &mut DeserializeEngine<R, T>,
) -> Result<(), DeserializeError> {
    deserialize::next_signature(engine)?;
    deserialize::next_version(engine)?;
    deserialize::next_header(engine)?;

    while engine.bytes.index() < engine.bytes.len() - 1 {
        match engine.bytes.next().try_into()? {
            TableIdentifier::Character => {
                #[cfg(feature = "tagging")]
                {
                    engine.tagging_data.current_table_index =
                        engine.layout.character_tables.len() as u8;
                }
                let table = CharacterTable::deserialize(engine)?;
                engine.layout.character_tables.push(table);
            }
            TableIdentifier::Pixmap => {
                #[cfg(feature = "tagging")]
                {
                    engine.tagging_data.current_table_index =
                        engine.layout.pixmap_tables.len() as u8;
                }
                let table = PixmapTable::deserialize(engine)?;
                engine.layout.pixmap_tables.push(table);
            }
            TableIdentifier::Color => {
                #[cfg(feature = "tagging")]
                {
                    engine.tagging_data.current_table_index =
                        engine.layout.color_tables.len() as u8;
                }
                let table = ColorTable::deserialize(engine)?;
                engine.layout.color_tables.push(table);
            }
            TableIdentifier::Font => {
                #[cfg(feature = "tagging")]
                {
                    engine.tagging_data.current_table_index = engine.layout.font_tables.len() as u8;
                }
                let table = FontTable::deserialize(engine)?;
                engine.layout.font_tables.push(table);
            }
        };
    }
    Ok(())
}

pub fn deserialize_with_engine<R: ByteReader, T: TagWriter>(
    engine: &mut DeserializeEngine<R, T>,
) -> Result<(), DeserializeError> {
    deserialize_layout(engine)?;
    Ok(())
}

/// Parses a [`&[u8]`] into a font [`Layout`]. This function interally creates a [`DeserializeEngine`]
/// and calls [`deserialize_with_engine`].
pub fn layout_from_data(buffer: &[u8]) -> Result<Layout, DeserializeError> {
    let mut engine = DeserializeEngine::from_data(buffer);
    deserialize_with_engine(&mut engine)?;
    Ok(engine.layout)
}

pub(crate) fn serialize_layout<T: TagWriter>(
    engine: &mut SerializeEngine<T>,
) -> Result<(), SerializeError> {
    serialize::push_signature(engine);
    serialize::push_version(engine);
    serialize::push_header(engine);

    for (index, character_table) in engine.layout.character_tables.iter().enumerate() {
        #[cfg(feature = "tagging")]
        {
            engine.tagging_data.current_table_index = index as u8;
        }
        character_table.serialize(engine)?;
    }
    for (index, pixmap_table) in engine.layout.pixmap_tables.iter().enumerate() {
        #[cfg(feature = "tagging")]
        {
            engine.tagging_data.current_table_index = index as u8;
        }
        pixmap_table.serialize(engine)?;
    }
    for (index, color_table) in engine.layout.color_tables.iter().enumerate() {
        #[cfg(feature = "tagging")]
        {
            engine.tagging_data.current_table_index = index as u8;
        }
        color_table.serialize(engine)?;
    }
    for (index, font_table) in engine.layout.font_tables.iter().enumerate() {
        #[cfg(feature = "tagging")]
        {
            engine.tagging_data.current_table_index = index as u8;
        }
        font_table.serialize(engine)?;
    }

    Ok(())
}

pub fn serialize_with_engine<T: TagWriter>(
    engine: &mut SerializeEngine<T>,
) -> Result<(), SerializeError> {
    serialize_layout(engine)?;
    Ok(())
}

/// Encodes the provided font [`Layout`] into a [`Vec<u8>`]. This function interally creates a
/// [`SerializeEngine`] and calls [`serialize_with_engine`].
pub fn layout_to_data(layout: &Layout) -> Result<Vec<u8>, SerializeError> {
    let mut engine = SerializeEngine::from_layout(layout);
    serialize_with_engine(&mut engine)?;
    Ok(engine.data_owned())
}
