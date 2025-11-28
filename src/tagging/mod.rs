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

use crate::core::{byte::*, *};
use crate::{format, String, Vec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteIndex {
    pub byte: usize,
    pub bit: u8,
}

impl ByteIndex {
    pub fn new(byte: usize, bit: u8) -> Self {
        Self { byte, bit }
    }

    pub fn at_byte(byte: usize) -> Self {
        Self { byte, bit: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: ByteIndex,
    pub end: ByteIndex,
}

impl Span {
    pub fn new(start: ByteIndex, end: ByteIndex) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone)]
pub enum TagKind {
    Reserved,
    Signature,
    Version {
        value: Version,
    },
    Header,
    CompactFlag {
        enabled: bool,
    },
    TableIdentifier {
        table_type: TableType,
    },

    CharacterTable {
        index: u8,
    },

    CharacterTableModifierFlags {
        table_index: u8,
    },

    CharacterTableUseAdvanceX {
        table_index: u8,
        value: bool,
    },
    CharacterTableUsePixmapIndex {
        table_index: u8,
        value: bool,
    },

    CharacterTableConfigurations {
        table_index: u8,
    },

    CharacterTableConfigurationFlags {
        table_index: u8,
    },
    CharacterTableUseConstantClusterCodepoints {
        table_index: u8,
        value: bool,
    },

    CharacterTableConfigurationValues {
        table_index: u8,
    },
    CharacterTableConstantClusterCodepoints {
        table_index: u8,
        value: u8,
    },

    CharacterTableLinks {
        table_index: u8,
    },

    CharacterTableLinkFlags {
        table_index: u8,
    },
    CharacterTableLinkPixmapTables {
        table_index: u8,
        value: bool,
    },

    CharacterTablePixmapTableLinks {
        table_index: u8,
    },

    CharacterTablePixmapTableIndexesLength {
        table_index: u8,
        count: u8,
    },
    CharacterTablePixmapTableIndexes {
        table_index: u8,
        indexes: Vec<u8>,
    },
    CharacterTablePixmapTableIndex {
        table_index: u8,
        index: u8,
    },

    CharacterTableCharacterCount {
        table_index: u8,
        count: u8,
    },

    CharacterRecord {
        table_index: u8,
        char_index: u8,
    },
    CharacterAdvanceX {
        table_index: u8,
        char_index: u8,
        value: u8,
    },
    CharacterPixmapIndex {
        table_index: u8,
        char_index: u8,
        value: u8,
    },
    CharacterGraphemeCluster {
        table_index: u8,
        char_index: u8,
        value: String,
    },

    PixmapTable {
        index: u8,
    },
    PixmapTableModifierFlags {
        table_index: u8,
    },

    PixmapTableConfigurations {
        table_index: u8,
    },

    PixmapTableConfigurationFlags {
        table_index: u8,
    },
    PixmapTableUseConstantWidth {
        table_index: u8,
        value: bool,
    },
    PixmapTableUseConstantHeight {
        table_index: u8,
        value: bool,
    },
    PixmapTableUseConstantBitsPerPixel {
        table_index: u8,
        value: bool,
    },

    PixmapTableConfigurationValues {
        table_index: u8,
    },
    PixmapTableConstantWidth {
        table_index: u8,
        value: u8,
    },
    PixmapTableConstantHeight {
        table_index: u8,
        value: u8,
    },
    PixmapTableConstantBitsPerPixel {
        table_index: u8,
        value: u8,
    },

    PixmapTableLinks {
        table_index: u8,
    },

    PixmapTableLinkFlags {
        table_index: u8,
    },
    PixmapTableLinkColorTables {
        table_index: u8,
        value: bool,
    },

    PixmapTableColorTableLinks {
        table_index: u8,
    },
    PixmapTableColorTableIndexesLength {
        table_index: u8,
        count: u8,
    },
    PixmapTableColorTableIndexes {
        table_index: u8,
        indexes: Vec<u8>,
    },
    PixmapTablePixmapCount {
        table_index: u8,
        count: u8,
    },
    PixmapTableColorTableIndex {
        table_index: u8,
        index: u8,
    },

    PixmapRecord {
        table_index: u8,
        pixmap_index: u8,
    },
    PixmapCustomWidth {
        table_index: u8,
        pixmap_index: u8,
        value: u8,
    },
    PixmapCustomHeight {
        table_index: u8,
        pixmap_index: u8,
        value: u8,
    },
    PixmapCustomBitsPerPixel {
        table_index: u8,
        pixmap_index: u8,
        value: u8,
    },
    PixmapData {
        table_index: u8,
        pixmap_index: u8,
        data: Vec<u8>,
    },

    ColorTable {
        index: u8,
    },
    ColorTableModifierFlags {
        table_index: u8,
    },

    ColorTableConfigurations {
        table_index: u8,
    },
    ColorTableConfigurationFlags {
        table_index: u8,
    },
    ColorTableConfigurationValues {
        table_index: u8,
    },
    ColorTableUseConstantAlpha {
        table_index: u8,
        value: bool,
    },
    ColorTableConstantAlpha {
        table_index: u8,
        value: u8,
    },
    ColorTableLinks {
        table_index: u8,
    },
    ColorTableLinkFlags {
        table_index: u8,
    },
    ColorTableColorCount {
        table_index: u8,
        count: u8,
    },

    ColorRecord {
        table_index: u8,
        color_index: u8,
    },
    ColorCustomAlpha {
        table_index: u8,
        color_index: u8,
        value: u8,
    },
    ColorR {
        table_index: u8,
        color_index: u8,
        value: u8,
    },
    ColorG {
        table_index: u8,
        color_index: u8,
        value: u8,
    },
    ColorB {
        table_index: u8,
        color_index: u8,
        value: u8,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableType {
    Character,
    Pixmap,
    Color,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub kind: TagKind,
    pub span: Span,
}

impl Tag {
    pub fn new(kind: TagKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl ByteWriter {
    pub(crate) fn byte_index(&self) -> ByteIndex {
        ByteIndex::new(self.index, self.pointer)
    }
}

impl ByteReader<'_> {
    pub(crate) fn byte_index(&self) -> ByteIndex {
        ByteIndex::new(self.index, self.pointer)
    }
}

pub trait TagWriter {
    fn tag_span(&mut self, kind: TagKind, span: Span);
    fn tag_byte(&mut self, kind: TagKind, end_byte: ByteIndex);
    fn tag_bitflag(&mut self, kind: TagKind, kinds: Vec<TagKind>, end_byte: ByteIndex);
}

#[derive(Clone)]
pub struct TagWriterImpl {
    pub tags: Vec<Tag>,
}

impl core::fmt::Display for TagWriterImpl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for tag in &self.tags {
            let debug_string = format!("{:?}", tag.kind);
            let varient_name = debug_string
                .split(|c| c == '(' || c == '{')
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

impl TagWriterImpl {
    pub fn new() -> Self {
        Self { tags: Vec::new() }
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

pub struct TagWriterNoOp;

impl TagWriter for TagWriterNoOp {
    fn tag_span(&mut self, _kind: TagKind, _span: Span) {}
    fn tag_byte(&mut self, _kind: TagKind, _end_byte: ByteIndex) {}
    fn tag_bitflag(&mut self, _kind: TagKind, _kinds: Vec<TagKind>, _end_byte: ByteIndex) {}
}
