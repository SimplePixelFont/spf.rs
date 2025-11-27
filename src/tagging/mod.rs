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

use crate::core::*;
use crate::Vec;

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
        use_advance_x: bool,
        use_pixmap_index: bool,
    },
    CharacterTableConfigurationFlags {
        table_index: u8,
    },
    CharacterTableConstantClusterCodepoints {
        table_index: u8,
        value: u8,
    },
    CharacterTableLinkFlags {
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
    PixmapTableConfigurationFlags {
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
    PixmapTableLinkFlags {
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
        byte_length: u8,
    },

    ColorTable {
        index: u8,
    },
    ColorTableModifierFlags {
        table_index: u8,
    },
    ColorTableConfigurationFlags {
        table_index: u8,
    },
    ColorTableConstantAlpha {
        table_index: u8,
        value: u8,
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

pub trait TagWriter {
    fn add_tag(&mut self, kind: TagKind, span: Span);
}

pub struct TagWriterImpl {
    pub tags: Vec<Tag>,
}

impl TagWriterImpl {
    pub fn new() -> Self {
        Self { tags: Vec::new() }
    }
}

impl TagWriter for TagWriterImpl {
    fn add_tag(&mut self, kind: TagKind, span: Span) {
        self.tags.push(Tag::new(kind, span));
    }
}

pub struct TagWriterNoOp;

impl TagWriter for TagWriterNoOp {
    fn add_tag(&mut self, _kind: TagKind, _span: Span) {}
}
