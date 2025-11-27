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
        index: usize,
    },
    CharacterTableModifierFlags {
        table_index: usize,
        use_advance_x: bool,
        use_pixmap_index: bool,
    },
    CharacterTableConfigurationFlags {
        table_index: usize,
    },
    CharacterTableConstantClusterCodepoints {
        table_index: usize,
        value: u8,
    },
    CharacterTableLinkFlags {
        table_index: usize,
    },
    CharacterTablePixmapTableIndexesLength {
        table_index: usize,
        count: usize,
    },
    CharacterTablePixmapTableIndexes {
        table_index: usize,
        indexes: Vec<u8>,
    },
    CharacterTableCharacterCount {
        table_index: usize,
        count: usize,
    },

    CharacterRecord {
        table_index: usize,
        char_index: usize,
    },
    CharacterAdvanceX {
        table_index: usize,
        char_index: usize,
        value: u8,
    },
    CharacterPixmapIndex {
        table_index: usize,
        char_index: usize,
        value: u8,
    },
    CharacterGraphemeCluster {
        table_index: usize,
        char_index: usize,
        value: String,
    },

    PixmapTable {
        index: usize,
    },
    PixmapTableModifierFlags {
        table_index: usize,
    },
    PixmapTableConfigurationFlags {
        table_index: usize,
    },
    PixmapTableConstantWidth {
        table_index: usize,
        value: u8,
    },
    PixmapTableConstantHeight {
        table_index: usize,
        value: u8,
    },
    PixmapTableConstantBitsPerPixel {
        table_index: usize,
        value: u8,
    },
    PixmapTableLinkFlags {
        table_index: usize,
    },
    PixmapTableColorTableIndexesLength {
        table_index: usize,
        count: usize,
    },
    PixmapTableColorTableIndexes {
        table_index: usize,
        indexes: Vec<u8>,
    },
    PixmapTablePixmapCount {
        table_index: usize,
        count: usize,
    },

    PixmapRecord {
        table_index: usize,
        pixmap_index: usize,
    },
    PixmapCustomWidth {
        table_index: usize,
        pixmap_index: usize,
        value: u8,
    },
    PixmapCustomHeight {
        table_index: usize,
        pixmap_index: usize,
        value: u8,
    },
    PixmapCustomBitsPerPixel {
        table_index: usize,
        pixmap_index: usize,
        value: u8,
    },
    PixmapData {
        table_index: usize,
        pixmap_index: usize,
        byte_length: usize,
    },

    ColorTable {
        index: usize,
    },
    ColorTableModifierFlags {
        table_index: usize,
    },
    ColorTableConfigurationFlags {
        table_index: usize,
    },
    ColorTableConstantAlpha {
        table_index: usize,
        value: u8,
    },
    ColorTableLinkFlags {
        table_index: usize,
    },
    ColorTableColorCount {
        table_index: usize,
        count: usize,
    },

    ColorRecord {
        table_index: usize,
        color_index: usize,
    },
    ColorCustomAlpha {
        table_index: usize,
        color_index: usize,
        value: u8,
    },
    ColorR {
        table_index: usize,
        color_index: usize,
        value: u8,
    },
    ColorG {
        table_index: usize,
        color_index: usize,
        value: u8,
    },
    ColorB {
        table_index: usize,
        color_index: usize,
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

pub struct TagStorage {
    pub tags: Vec<Tag>,
}

impl TagStorage {
    pub fn new() -> Self {
        Self { tags: Vec::new() }
    }

    pub fn add_tag(&mut self, kind: TagKind, span: Span) {
        self.tags.push(Tag::new(kind, span));
    }
}
