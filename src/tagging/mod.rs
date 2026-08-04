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

//! Byte-level instrumentation: records the exact byte:bit [`Span`] of every field written or
//! read, tagged by [`TagKind`], so a `.spf` file's structure can be inspected after a parse
//! or serialize pass.

use crate::core::{byte::*, *};
use crate::{format, String, Vec};

/// A bit-precision position within a `.spf` file's byte stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteIndex {
    /// The byte offset from the start of the file.
    pub byte: usize,
    /// The bit offset within `byte`, `0`-`7`.
    pub bit: u8,
}

impl ByteIndex {
    /// Constructs a [`ByteIndex`] at an explicit byte and bit offset.
    pub fn new(byte: usize, bit: u8) -> Self {
        Self { byte, bit }
    }

    /// Constructs a [`ByteIndex`] at the start of `byte` (bit `0`).
    pub fn at_byte(byte: usize) -> Self {
        Self { byte, bit: 0 }
    }
}

/// A bit-precision byte range tagged as belonging to one [`TagKind`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// The inclusive start of the range.
    pub start: ByteIndex,
    /// The exclusive end of the range.
    pub end: ByteIndex,
}

impl Span {
    /// Constructs a [`Span`] from a start and end [`ByteIndex`].
    pub fn new(start: ByteIndex, end: ByteIndex) -> Self {
        Self { start, end }
    }
}

/// What piece of a `.spf` file a [`Tag`]'s [`Span`] corresponds to.
///
/// Most variants name a real item in [`crate::core`] — either the whole struct/enum
/// (`Tags [`CharacterTable`]`), a specific field (`Tags [`Character::code_points`]`), or a
/// bitflags constant (`Tags [`CharacterTableConfigurationFlags::ConstantCodePointCount`]`).
/// A handful of section-grouping variants (e.g. `CharacterTableConfigurations`,
/// `CharacterTableLinks`) span multiple fields at once and have no single backing item —
/// those are documented with a plain description of what they cover instead.
#[derive(Debug, Clone)]
pub enum TagKind {
    /// Tags unused padding bits within a bitflags byte.
    Reserved,
    /// Tags the fixed magic-byte signature at the start of every `.spf` file.
    Signature,
    /// Tags [`Layout::version`].
    Version {
        /// The parsed version value.
        value: Version,
    },
    /// Tags the file-level header region. Does not correspond to any single struct — the
    /// current `.spf` layout is flat, there is no `Header` type in [`crate::core`].
    Header,
    /// Tags [`Layout::compact`].
    CompactFlag {
        /// The parsed compact flag value.
        enabled: bool,
    },
    /// Tags [`TableType`].
    TableIdentifier {
        /// Which table kind this identifier names.
        table_type: TableType,
    },

    /// Tags [`CharacterTable`].
    CharacterTable {
        /// Index of this table within its `Layout`.
        index: u8,
    },

    /// Tags [`CharacterTableModifierFlags`].
    CharacterTableModifierFlags {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },

    /// Tags [`CharacterTableModifierFlags::UseAdvanceX`].
    CharacterTableUseAdvanceX {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: bool,
    },
    /// Tags [`CharacterTableModifierFlags::UsePixmapIndex`].
    CharacterTableUsePixmapIndex {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: bool,
    },
    /// Tags [`CharacterTableModifierFlags::UsePixmapTableIndex`].
    CharacterTableUsePixmapTableIndex {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: bool,
    },

    /// Tags the configuration flags and values together for a [`CharacterTable`].
    CharacterTableConfigurations {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },

    /// Tags [`CharacterTableConfigurationFlags`].
    CharacterTableConfigurationFlags {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },
    /// Tags [`CharacterTableConfigurationFlags::ConstantCodePointCount`].
    CharacterTableUseConstantCodePointCount {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: bool,
    },

    /// Tags the configuration values (following the flags byte) for a [`CharacterTable`].
    CharacterTableConfigurationValues {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },
    /// Tags [`CharacterTable::constant_code_point_count`].
    CharacterTableConstantCodePointCount {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: u8,
    },

    /// Tags the link flags and index arrays together for a [`CharacterTable`].
    CharacterTableLinks {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },

    /// Tags [`CharacterTableLinkFlags`].
    CharacterTableLinkFlags {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },
    /// Tags [`CharacterTableLinkFlags::LinkPixmapTables`].
    CharacterTableLinkPixmapTables {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: bool,
    },

    /// Tags the pixmap table link section (length prefix + indexes) of a [`CharacterTable`].
    CharacterTablePixmapTableLinks {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },

    /// Tags the length prefix of [`CharacterTable::pixmap_table_indexes`].
    CharacterTablePixmapTableIndexesLength {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Number of entries in the following array.
        count: u8,
    },
    /// Tags [`CharacterTable::pixmap_table_indexes`].
    CharacterTablePixmapTableIndexes {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The recorded index values.
        indexes: Vec<u8>,
    },
    /// Tags one entry of [`CharacterTable::pixmap_table_indexes`].
    CharacterTablePixmapTableIndex {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The recorded index value.
        index: u8,
    },

    /// Tags the record-count prefix for [`CharacterTable::characters`].
    CharacterTableCharacterCount {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Number of entries in the following array.
        count: u8,
    },

    /// Tags [`Character`].
    CharacterRecord {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the character record within its `CharacterTable`.
        char_index: u8,
    },
    /// Tags [`Character::advance_x`].
    CharacterAdvanceX {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the character record within its `CharacterTable`.
        char_index: u8,
        /// The tagged value.
        value: u8,
    },
    /// Tags [`Character::pixmap_index`].
    CharacterPixmapIndex {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the character record within its `CharacterTable`.
        char_index: u8,
        /// The tagged value.
        value: u8,
    },
    /// Tags [`Character::pixmap_table_index`].
    CharacterPixmapTableIndex {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the character record within its `CharacterTable`.
        char_index: u8,
        /// The tagged value.
        value: u8,
    },
    /// Tags [`Character::code_points`].
    CharacterCodePoints {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the character record within its `CharacterTable`.
        char_index: u8,
        /// The tagged value.
        value: String,
    },

    /// Tags [`PixmapTable`].
    PixmapTable {
        /// Index of this table within its `Layout`.
        index: u8,
    },
    /// Tags the (always-empty) modifier flags byte of a [`PixmapTable`] — `PixmapTable` has
    /// no per-record optional fields toggled by a modifier byte, unlike `CharacterTable` and
    /// `ColorTable`; there is no `PixmapTableModifierFlags` struct in [`crate::core`].
    PixmapTableModifierFlags {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },

    /// Tags the configuration flags and values together for a [`PixmapTable`].
    PixmapTableConfigurations {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },

    /// Tags [`PixmapTableConfigurationFlags`].
    PixmapTableConfigurationFlags {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },
    /// Tags [`PixmapTableConfigurationFlags::ConstantWidth`].
    PixmapTableUseConstantWidth {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: bool,
    },
    /// Tags [`PixmapTableConfigurationFlags::ConstantHeight`].
    PixmapTableUseConstantHeight {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: bool,
    },
    /// Tags [`PixmapTableConfigurationFlags::ConstantBitsPerPixel`].
    PixmapTableUseConstantBitsPerPixel {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: bool,
    },

    /// Tags the configuration values (following the flags byte) for a [`PixmapTable`].
    PixmapTableConfigurationValues {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },
    /// Tags [`PixmapTable::constant_width`].
    PixmapTableConstantWidth {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: u8,
    },
    /// Tags [`PixmapTable::constant_height`].
    PixmapTableConstantHeight {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: u8,
    },
    /// Tags [`PixmapTable::constant_bits_per_pixel`].
    PixmapTableConstantBitsPerPixel {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: u8,
    },

    /// Tags the link flags and index arrays together for a [`PixmapTable`].
    PixmapTableLinks {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },

    /// Tags [`PixmapTableLinkFlags`].
    PixmapTableLinkFlags {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },
    /// Tags [`PixmapTableLinkFlags::LinkColorTables`].
    PixmapTableLinkColorTables {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: bool,
    },

    /// Tags the color table link section (length prefix + indexes) of a [`PixmapTable`].
    PixmapTableColorTableLinks {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },
    /// Tags the length prefix of [`PixmapTable::color_table_indexes`].
    PixmapTableColorTableIndexesLength {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Number of entries in the following array.
        count: u8,
    },
    /// Tags [`PixmapTable::color_table_indexes`].
    PixmapTableColorTableIndexes {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The recorded index values.
        indexes: Vec<u8>,
    },
    /// Tags the record-count prefix for [`PixmapTable::pixmaps`].
    PixmapTablePixmapCount {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Number of entries in the following array.
        count: u8,
    },
    /// Tags one entry of [`PixmapTable::color_table_indexes`].
    PixmapTableColorTableIndex {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The recorded index value.
        index: u8,
    },

    /// Tags [`Pixmap`].
    PixmapRecord {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the pixmap record within its `PixmapTable`.
        pixmap_index: u8,
    },
    /// Tags [`Pixmap::custom_width`].
    PixmapCustomWidth {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the pixmap record within its `PixmapTable`.
        pixmap_index: u8,
        /// The tagged value.
        value: u8,
    },
    /// Tags [`Pixmap::custom_height`].
    PixmapCustomHeight {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the pixmap record within its `PixmapTable`.
        pixmap_index: u8,
        /// The tagged value.
        value: u8,
    },
    /// Tags [`Pixmap::custom_bits_per_pixel`].
    PixmapCustomBitsPerPixel {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the pixmap record within its `PixmapTable`.
        pixmap_index: u8,
        /// The tagged value.
        value: u8,
    },
    /// Tags [`Pixmap::data`].
    PixmapData {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the pixmap record within its `PixmapTable`.
        pixmap_index: u8,
        /// The tagged pixel data.
        data: Vec<u8>,
    },

    /// Tags [`ColorTable`].
    ColorTable {
        /// Index of this table within its `Layout`.
        index: u8,
    },
    /// Tags [`ColorTableModifierFlags`].
    ColorTableModifierFlags {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },
    /// Tags [`ColorTableModifierFlags::UseColorType`].
    ColorTableUseColorType {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: bool,
    },

    /// Tags the configuration flags and values together for a [`ColorTable`].
    ColorTableConfigurations {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },
    /// Tags [`ColorTableConfigurationFlags`].
    ColorTableConfigurationFlags {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },
    /// Tags the configuration values (following the flags byte) for a [`ColorTable`].
    ColorTableConfigurationValues {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },
    /// Tags [`ColorTableConfigurationFlags::ConstantAlpha`].
    ColorTableUseConstantAlpha {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: bool,
    },
    /// Tags [`ColorTable::constant_alpha`].
    ColorTableConstantAlpha {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: u8,
    },
    /// Tags the (always-empty) link flags byte of a [`ColorTable`] — `ColorTable` doesn't
    /// link to other tables, so there is no `ColorTableLinkFlags` struct in [`crate::core`].
    ColorTableLinks {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },
    /// Tags the (always-empty) link flags byte of a [`ColorTable`], same reason as
    /// [`TagKind::ColorTableLinks`].
    ColorTableLinkFlags {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },
    /// Tags the record-count prefix for [`ColorTable::colors`].
    ColorTableColorCount {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Number of entries in the following array.
        count: u8,
    },

    /// Tags [`Color`].
    ColorRecord {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the color record within its `ColorTable`.
        color_index: u8,
    },
    /// Tags [`Color::color_type`].
    ColorColorType {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the color record within its `ColorTable`.
        color_index: u8,
        /// The tagged value.
        value: ColorType,
    },
    /// Tags [`Color::custom_alpha`].
    ColorCustomAlpha {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the color record within its `ColorTable`.
        color_index: u8,
        /// The tagged value.
        value: u8,
    },
    /// Tags [`Color::red`].
    ColorRed {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the color record within its `ColorTable`.
        color_index: u8,
        /// The tagged value.
        value: u8,
    },
    /// Tags [`Color::green`].
    ColorGreen {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the color record within its `ColorTable`.
        color_index: u8,
        /// The tagged value.
        value: u8,
    },
    /// Tags [`Color::blue`].
    ColorBlue {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the color record within its `ColorTable`.
        color_index: u8,
        /// The tagged value.
        value: u8,
    },

    /// Tags [`FontTable`].
    FontTable {
        /// Index of this table within its `Layout`.
        index: u8,
    },

    /// Tags the (always-empty) modifier flags byte of a [`FontTable`] — `Font` records have
    /// no optional fields toggled by a modifier byte, so there is no `FontTableModifierFlags`
    /// struct in [`crate::core`].
    FontTableModifierFlags {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },

    /// Tags the (always-empty) configuration flags and values byte of a [`FontTable`] —
    /// `FontTable` has no configuration values, so there is no `FontTableConfigurationFlags`
    /// struct in [`crate::core`].
    FontTableConfigurations {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },

    /// Tags the (always-empty) configuration flags byte of a [`FontTable`], same reason as
    /// [`TagKind::FontTableConfigurations`].
    FontTableConfigurationFlags {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },

    /// Tags the link flags and index array together for a [`FontTable`].
    FontTableLinks {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },

    /// Tags [`FontTableLinkFlags`].
    FontTableLinkFlags {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },
    /// Tags [`FontTableLinkFlags::LinkCharacterTables`].
    FontTableLinkCharacterTables {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The tagged value.
        value: bool,
    },

    /// Tags the character table link section (length prefix + indexes) of a [`FontTable`].
    FontTableCharacterTableLinks {
        /// Index of the table this tag belongs to.
        table_index: u8,
    },

    /// Tags the length prefix of [`FontTable::character_table_indexes`].
    FontTableCharacterTableIndexesLength {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Number of entries in the following array.
        count: u8,
    },
    /// Tags [`FontTable::character_table_indexes`].
    FontTableCharacterTableIndexes {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The recorded index values.
        indexes: Vec<u8>,
    },
    /// Tags one entry of [`FontTable::character_table_indexes`].
    FontTableCharacterTableIndex {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// The recorded index value.
        index: u8,
    },

    /// Tags the record-count prefix for [`FontTable::fonts`].
    FontTableFontCount {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Number of entries in the following array.
        count: u8,
    },

    /// Tags [`Font`].
    FontRecord {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the font record within its `FontTable`.
        font_index: u8,
    },
    /// Tags [`Font::name`].
    FontName {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the font record within its `FontTable`.
        font_index: u8,
        /// The tagged value.
        value: String,
    },
    /// Tags [`Font::author`].
    FontAuthor {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the font record within its `FontTable`.
        font_index: u8,
        /// The tagged value.
        value: String,
    },
    /// Tags [`Font::version`].
    FontVersion {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the font record within its `FontTable`.
        font_index: u8,
        /// The tagged value.
        value: u8,
    },
    /// Tags [`Font::font_type`].
    FontFontType {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the font record within its `FontTable`.
        font_index: u8,
        /// The tagged value.
        value: FontType,
    },
    /// Tags [`Font::linked_character_table_indexes`].
    FontLinkedCharacterTableIndexes {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the font record within its `FontTable`.
        font_index: u8,
        /// The tagged value.
        value: Vec<u8>,
    },
    /// Tags the length prefix of [`Font::linked_character_table_indexes`].
    FontLinkedCharacterTableIndexesLength {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the font record within its `FontTable`.
        font_index: u8,
        /// Number of entries in the following array.
        count: u8,
    },
    /// Tags one entry of [`Font::linked_character_table_indexes`].
    FontLinkedCharacterTableIndexesIndex {
        /// Index of the table this tag belongs to.
        table_index: u8,
        /// Index of the font record within its `FontTable`.
        font_index: u8,
        /// Index of this table within its `Layout`.
        index: u8,
    },
}

/// Identifies which of the four table kinds a [`TagKind::TableIdentifier`] tag names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableType {
    /// A [`CharacterTable`].
    Character,
    /// A [`PixmapTable`].
    Pixmap,
    /// A [`ColorTable`].
    Color,
    /// A [`FontTable`].
    Font,
}

/// One [`TagKind`] paired with the byte:bit [`Span`] it was written to or read from.
#[derive(Debug, Clone)]
pub struct Tag {
    /// What this tag identifies.
    pub kind: TagKind,
    /// Where in the file this tag's data lives.
    pub span: Span,
}

impl Tag {
    /// Constructs a [`Tag`] from a kind and its span.
    pub fn new(kind: TagKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl ByteWriter {
    pub(crate) fn byte_index(&self) -> ByteIndex {
        ByteIndex::new(self.index, self.pointer)
    }
}

/// Records the byte:bit span of every field written or read, as it happens.
///
/// Implemented by [`TagWriterImpl`] (collects real [`Tag`]s) and [`TagWriterNoOp`] (the
/// zero-cost default when the `tagging` feature is disabled). Every `push_*`/`next_*`
/// function in `core` threads a call to one of these methods alongside its actual byte
/// write/read.
pub trait TagWriter {
    /// Records `kind` as spanning exactly `span`.
    fn tag_span(&mut self, kind: TagKind, span: Span);
    /// Records `kind` as spanning the single byte ending at `end_byte`.
    fn tag_byte(&mut self, kind: TagKind, end_byte: ByteIndex);
    /// Records `kind` as spanning a bitflags byte ending at `end_byte`, splitting it into
    /// one sub-span per entry in `kinds` (one per flag bit, in bit order) plus a
    /// [`TagKind::Reserved`] span for any unused trailing bits.
    fn tag_bitflag(&mut self, kind: TagKind, kinds: Vec<TagKind>, end_byte: ByteIndex);
}

/// The real [`TagWriter`]: collects every [`Tag`] into `tags`, in write/read order.
#[derive(Clone, Default)]
pub struct TagWriterImpl {
    /// Every tag recorded so far.
    pub tags: Vec<Tag>,
}

impl core::fmt::Display for TagWriterImpl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for tag in &self.tags {
            let debug_string = format!("{:?}", tag.kind);
            let varient_name = debug_string
                .split(['(', '{'])
                .next()
                .unwrap_or(&debug_string);

            writeln!(
                f,
                "{} {}:{} - {}:{}",
                varient_name,
                tag.span.start.byte,
                tag.span.start.bit,
                tag.span.end.byte,
                tag.span.end.bit
            )?;
        }
        Ok(())
    }
}

impl TagWriter for TagWriterImpl {
    fn tag_span(&mut self, kind: TagKind, span: Span) {
        self.tags.push(Tag::new(kind, span));
    }
    fn tag_byte(&mut self, kind: TagKind, byte_end: ByteIndex) {
        let byte_start = ByteIndex::new(byte_end.byte - 1, byte_end.bit);
        self.tag_span(kind, Span::new(byte_start, byte_end));
    }
    fn tag_bitflag(&mut self, kind: TagKind, kinds: Vec<TagKind>, byte_end: ByteIndex) {
        let byte_start = ByteIndex::new(byte_end.byte - 1, byte_end.bit);

        // Tag bitflags
        let reserved_bits = 8 - kinds.len();
        for (index, tag_kind) in kinds.iter().enumerate() {
            let bit_offset = byte_start.bit + index as u8;
            let bit_start =
                ByteIndex::new(byte_start.byte + (bit_offset / 8) as usize, bit_offset % 8);
            let bit_offset_end = bit_offset + 1;
            let bit_end = ByteIndex::new(
                byte_start.byte + (bit_offset_end / 8) as usize,
                bit_offset_end % 8,
            );
            self.tag_span(tag_kind.clone(), Span::new(bit_start, bit_end));
        }

        // Tag reserved bits
        if reserved_bits > 0 {
            let reserved_start_offset = byte_start.bit + kinds.len() as u8;
            let reserved_start = ByteIndex::new(
                byte_start.byte + (reserved_start_offset / 8) as usize,
                reserved_start_offset % 8,
            );
            let reserved_end_offset = byte_start.bit + 8;
            let reserved_end = ByteIndex::new(
                byte_start.byte + (reserved_end_offset / 8) as usize,
                reserved_end_offset % 8,
            );
            self.tag_span(TagKind::Reserved, Span::new(reserved_start, reserved_end));
        }

        // Tag the entire byte
        self.tag_span(kind, Span::new(byte_start, byte_end));
    }
}

/// The zero-cost [`TagWriter`] used when the `tagging` feature is disabled — records nothing.
pub struct TagWriterNoOp;

impl TagWriter for TagWriterNoOp {
    fn tag_span(&mut self, _kind: TagKind, _span: Span) {}
    fn tag_byte(&mut self, _kind: TagKind, _end_byte: ByteIndex) {}
    fn tag_bitflag(&mut self, _kind: TagKind, _kinds: Vec<TagKind>, _end_byte: ByteIndex) {}
}
