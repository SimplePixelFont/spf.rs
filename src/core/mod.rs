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
    /// Bit flags selecting which configuration values are constant for every [`Pixmap`] in a [`PixmapTable`].
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
    /// Bit flags selecting which other tables a [`PixmapTable`] links to.
    pub struct PixmapTableLinkFlags: u8 {
        #[doc = include_str!("../../res/snippets/pixmap_table/links/flag/link_color_tables.md")]
        const LinkColorTables = 0b00000001;
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    /// Bit flags selecting which optional fields are present on every [`Character`] record in a [`CharacterTable`].
    pub struct CharacterTableModifierFlags: u8 {
        #[doc = include_str!("../../res/snippets/character_table/modifiers/brief/use_advance_x.md")]
        #[doc = include_str!("../../res/snippets/character_table/modifiers/details/use_advance_x.md")]
        const UseAdvanceX = 0b00000001;
        #[doc = include_str!("../../res/snippets/character_table/modifiers/brief/use_pixmap_index.md")]
        #[doc = include_str!("../../res/snippets/character_table/modifiers/details/use_pixmap_index.md")]
        const UsePixmapIndex = 0b00000010;
        #[doc = include_str!("../../res/snippets/character_table/modifiers/brief/use_pixmap_table_index.md")]
        #[doc = include_str!("../../res/snippets/character_table/modifiers/details/use_pixmap_table_index.md")]
        const UsePixmapTableIndex = 0b00000100;
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    /// Bit flags selecting which other tables a [`CharacterTable`] links to.
    pub struct CharacterTableLinkFlags: u8 {
        #[doc = include_str!("../../res/snippets/character_table/links/flag/link_pixmap_tables.md")]
        const LinkPixmapTables = 0b00000001;
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    /// Bit flags selecting which configuration values are constant for every [`Character`] in a [`CharacterTable`].
    pub struct CharacterTableConfigurationFlags: u8 {
        #[doc = include_str!("../../res/snippets/character_table/configurations/flag/use_constant_code_point_count.md")]
        const ConstantCodePointCount = 0b00000001;
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    /// Bit flags selecting which optional fields are present on every [`Color`] record in a [`ColorTable`].
    pub struct ColorTableModifierFlags: u8 {
        #[doc = include_str!("../../res/snippets/color_table/modifiers/brief/use_color_type.md")]
        #[doc = include_str!("../../res/snippets/color_table/modifiers/details/use_color_type.md")]
        const UseColorType = 0b00000001;
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    /// Bit flags selecting which configuration values are constant for every [`Color`] in a [`ColorTable`].
    pub struct ColorTableConfigurationFlags: u8 {
        #[doc = include_str!("../../res/snippets/color_table/configurations/flag/use_constant_alpha.md")]
        const ConstantAlpha = 0b00000001;
    }

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    /// Bit flags selecting which other tables a [`FontTable`] links to.
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
/// The full, decoded contents of a `.spf` file: its format version, packing mode, and every table it defines.
pub struct Layout {
    /// The format version this layout was parsed from, or should be serialized as.
    pub version: Version,

    /// Whether partial trailing bytes are packed to the bit (`true`) or padded out to a full byte (`false`). See the pixel/bit packing rules on [`Pixmap`].
    pub compact: bool,

    /// The character tables defined in this file, indexed in declaration order (this index is what [`CharacterTable`] links elsewhere refer to).
    pub character_tables: Vec<CharacterTable>,
    /// The color tables defined in this file, indexed in declaration order.
    pub color_tables: Vec<ColorTable>,
    /// The pixmap tables defined in this file, indexed in declaration order.
    pub pixmap_tables: Vec<PixmapTable>,
    /// The font tables defined in this file, indexed in declaration order.
    pub font_tables: Vec<FontTable>,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[doc = include_str!("../../res/snippets/pixmap_table/brief.md")]
pub struct PixmapTable {
    /// Which configuration values below are constant for every pixmap. See [`PixmapTableConfigurationFlags`].
    pub configuration_flags: PixmapTableConfigurationFlags,
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/condition/constant_width.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/brief/constant_width.md")]
    pub constant_width: Option<u8>,
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/condition/constant_height.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/brief/constant_height.md")]
    pub constant_height: Option<u8>,
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/condition/constant_bits_per_pixel.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/brief/constant_bits_per_pixel.md")]
    pub constant_bits_per_pixel: Option<u8>,

    /// Which other tables this table links to. See [`PixmapTableLinkFlags`].
    pub link_flags: PixmapTableLinkFlags,
    #[doc = include_str!("../../res/snippets/pixmap_table/links/condition/color_tables.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/links/brief/color_tables.md")]
    pub color_table_indexes: Option<Vec<u8>>,

    /// The pixmaps stored in this table, indexed in declaration order.
    pub pixmaps: Vec<Pixmap>,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pixmap {
    #[doc = include_str!("../../res/snippets/pixmap_table/records/condition/custom_width.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/records/brief/custom_width.md")]
    pub custom_width: Option<u8>,
    #[doc = include_str!("../../res/snippets/pixmap_table/records/condition/custom_height.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/records/brief/custom_height.md")]
    pub custom_height: Option<u8>,
    #[doc = include_str!("../../res/snippets/pixmap_table/records/condition/custom_bits_per_pixel.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/records/brief/custom_bits_per_pixel.md")]
    pub custom_bits_per_pixel: Option<u8>,
    #[doc = include_str!("../../res/snippets/pixmap_table/records/condition/data.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/records/brief/data.md")]
    pub data: Vec<u8>,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[doc = include_str!("../../res/snippets/character_table/brief.md")]
pub struct CharacterTable {
    /// Which optional per-character fields are present. See [`CharacterTableModifierFlags`].
    pub modifier_flags: CharacterTableModifierFlags,

    /// Which configuration values below are constant for every character. See [`CharacterTableConfigurationFlags`].
    pub configuration_flags: CharacterTableConfigurationFlags,
    #[doc = include_str!("../../res/snippets/character_table/configurations/condition/constant_code_point_count.md")]
    #[doc = include_str!("../../res/snippets/character_table/configurations/brief/constant_code_point_count.md")]
    pub constant_code_point_count: Option<u8>,

    /// Which other tables this table links to. See [`CharacterTableLinkFlags`].
    pub link_flags: CharacterTableLinkFlags,
    #[doc = include_str!("../../res/snippets/character_table/links/condition/pixmap_tables.md")]
    #[doc = include_str!("../../res/snippets/character_table/links/brief/pixmap_tables.md")]
    pub pixmap_table_indexes: Option<Vec<u8>>,

    /// The characters stored in this table, indexed in declaration order.
    pub characters: Vec<Character>,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Character {
    #[doc = include_str!("../../res/snippets/character_table/records/condition/advance_x.md")]
    #[doc = include_str!("../../res/snippets/character_table/records/brief/advance_x.md")]
    pub advance_x: Option<u8>,
    #[doc = include_str!("../../res/snippets/character_table/records/condition/pixmap_index.md")]
    #[doc = include_str!("../../res/snippets/character_table/records/brief/pixmap_index.md")]
    pub pixmap_index: Option<u8>,
    #[doc = include_str!("../../res/snippets/character_table/records/condition/pixmap_table_index.md")]
    #[doc = include_str!("../../res/snippets/character_table/records/brief/pixmap_table_index.md")]
    pub pixmap_table_index: Option<u8>,

    #[doc = include_str!("../../res/snippets/character_table/records/condition/code_points.md")]
    #[doc = include_str!("../../res/snippets/character_table/records/brief/code_points.md")]
    pub code_points: String,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[doc = include_str!("../../res/snippets/color_table/brief.md")]
pub struct ColorTable {
    /// Which optional per-color fields are present. See [`ColorTableModifierFlags`].
    pub modifier_flags: ColorTableModifierFlags,

    /// Which configuration values below are constant for every color. See [`ColorTableConfigurationFlags`].
    pub configuration_flags: ColorTableConfigurationFlags,
    #[doc = include_str!("../../res/snippets/color_table/configurations/condition/constant_alpha.md")]
    #[doc = include_str!("../../res/snippets/color_table/configurations/brief/constant_alpha.md")]
    pub constant_alpha: Option<u8>,

    /// The colors stored in this table, indexed in declaration order.
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
    #[doc = include_str!("../../res/snippets/color_table/records/condition/color_type.md")]
    #[doc = include_str!("../../res/snippets/color_table/records/brief/color_type.md")]
    pub color_type: Option<ColorType>,
    #[doc = include_str!("../../res/snippets/color_table/records/condition/custom_alpha.md")]
    #[doc = include_str!("../../res/snippets/color_table/records/brief/custom_alpha.md")]
    pub custom_alpha: Option<u8>,
    #[doc = include_str!("../../res/snippets/color_table/records/condition/red.md")]
    #[doc = include_str!("../../res/snippets/color_table/records/brief/red.md")]
    pub red: u8,
    #[doc = include_str!("../../res/snippets/color_table/records/condition/green.md")]
    #[doc = include_str!("../../res/snippets/color_table/records/brief/green.md")]
    pub green: u8,
    #[doc = include_str!("../../res/snippets/color_table/records/condition/blue.md")]
    #[doc = include_str!("../../res/snippets/color_table/records/brief/blue.md")]
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
#[doc = include_str!("../../res/snippets/font_table/brief.md")]
pub struct FontTable {
    /// Which other tables this table links to. See [`FontTableLinkFlags`].
    pub link_flags: FontTableLinkFlags,
    #[doc = include_str!("../../res/snippets/font_table/links/condition/character_tables.md")]
    #[doc = include_str!("../../res/snippets/font_table/links/brief/character_tables.md")]
    pub character_table_indexes: Option<Vec<u8>>,

    /// The fonts stored in this table, indexed in declaration order.
    pub fonts: Vec<Font>,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Font {
    #[doc = include_str!("../../res/snippets/font_table/records/condition/name.md")]
    #[doc = include_str!("../../res/snippets/font_table/records/brief/name.md")]
    pub name: String,
    #[doc = include_str!("../../res/snippets/font_table/records/condition/author.md")]
    #[doc = include_str!("../../res/snippets/font_table/records/brief/author.md")]
    pub author: String,
    #[doc = include_str!("../../res/snippets/font_table/records/condition/version.md")]
    #[doc = include_str!("../../res/snippets/font_table/records/brief/version.md")]
    pub version: u8,
    #[doc = include_str!("../../res/snippets/font_table/records/condition/font_type.md")]
    #[doc = include_str!("../../res/snippets/font_table/records/brief/font_type.md")]
    pub font_type: FontType,
    #[doc = include_str!("../../res/snippets/font_table/records/condition/linked_character_table_indexes.md")]
    #[doc = include_str!("../../res/snippets/font_table/records/brief/linked_character_table_indexes.md")]
    pub linked_character_table_indexes: Vec<u8>,
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
/// Errors that can occur while parsing a `.spf` byte buffer into a [`Layout`].
pub enum DeserializeError {
    #[doc = include_str!("../../res/snippets/errors/unexpected_end_of_file.md")]
    UnexpectedEndOfFile,
    #[doc = include_str!("../../res/snippets/errors/invalid_signature.md")]
    InvalidSignature,
    #[doc = include_str!("../../res/snippets/errors/unsupported_version.md")]
    UnsupportedVersion,
    #[doc = include_str!("../../res/snippets/errors/unsupported_color_type.md")]
    UnsupportedColorType,
    #[doc = include_str!("../../res/snippets/errors/unsupported_table_identifier.md")]
    UnsupportedTableIdentifier,
    #[doc = include_str!("../../res/snippets/errors/unsupported_font_type.md")]
    UnsupportedFontType,
}

#[derive(Debug)]
/// Errors that can occur while serializing a [`Layout`] into a `.spf` byte buffer.
pub enum SerializeError {
    #[doc = include_str!("../../res/snippets/errors/static_vector_too_large.md")]
    StaticVectorTooLarge,
    #[doc = include_str!("../../res/snippets/errors/invalid_pixmap_data.md")]
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
    /// The [`Layout`] built up so far as tables are read from `bytes`.
    pub layout: Layout,
    #[cfg(feature = "tagging")]
    /// Collects the byte/bit span of every field read, when the `tagging` feature is enabled.
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
    /// The [`Layout`] being serialized into `bytes`.
    pub layout: &'a Layout,
    #[cfg(feature = "tagging")]
    /// Collects the byte/bit span of every field written, when the `tagging` feature is enabled.
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

/// Deserializes into `engine`'s [`Layout`] using an already-constructed [`DeserializeEngine`]. Prefer [`layout_from_data`] unless you need direct control over the engine (for example, a custom [`ByteReader`] or [`TagWriter`]).
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

/// Serializes `engine`'s [`Layout`] using an already-constructed [`SerializeEngine`]. Prefer [`layout_to_data`] unless you need direct control over the engine (for example, a custom [`TagWriter`]).
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
