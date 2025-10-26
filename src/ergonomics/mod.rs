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

//! Rust-only module to abstract, and make writing `spf.rs` code easier.

use std::cell::RefCell;
use std::rc::Rc;

pub(crate) use crate::core::*;

pub mod builders;

pub use crate::ergonomics::builders::*;

// remove ToString trait
use crate::Vec;

/// Magic bytes of `SimplePixelFont` files
///
/// The magic bytes can be used to determine if a file is a `SimplePixelFont` regardless of
/// the file extension. That being said the magic bytes as u8 are are follows: `102, 115, 70`.
/// In utf8 encoding this spells out `fsF` with a .
pub const MAGIC_BYTES: [u8; 4] = [127, 102, 115, 70];

#[derive(Default, Debug, Clone)]
#[non_exhaustive]
/// [`LayoutBuilder`] lets you create [`Layout`]'s without all the nested structs.
pub struct LayoutBuilder {
    pub version: Version,
    pub compact: bool,
    pub character_table_builders: Vec<CharacterTableBuilder>,
    pub color_table_builders: Vec<ColorTableBuilder>,
    pub pixmap_table_builders: Vec<PixmapTableBuilder>,
}

pub enum TableBuilderIdentifier {
    Character(CharacterTableBuilder),
    Color(ColorTableBuilder),
    Pixmap(PixmapTableBuilder),
}

pub enum TableBuilderResult {
    Character(CharacterTable),
    Color(ColorTable),
    Pixmap(PixmapTable),
}

impl From<TableBuilderResult> for CharacterTable {
    fn from(table_builder_result: TableBuilderResult) -> Self {
        match table_builder_result {
            TableBuilderResult::Character(character_table) => character_table,
            _ => panic!("Cannot convert TableBuilderResult to CharacterTable"),
        }
    }
}

impl From<TableBuilderResult> for ColorTable {
    fn from(table_builder_result: TableBuilderResult) -> Self {
        match table_builder_result {
            TableBuilderResult::Color(color_table) => color_table,
            _ => panic!("Cannot convert TableBuilderResult to ColorTable"),
        }
    }
}

impl From<TableBuilderResult> for PixmapTable {
    fn from(table_builder_result: TableBuilderResult) -> Self {
        match table_builder_result {
            TableBuilderResult::Pixmap(pixmap_table) => pixmap_table,
            _ => panic!("Cannot convert TableBuilderResult to PixmapTable"),
        }
    }
}

pub trait TableBuilder {
    fn resolve(&mut self);
    fn build(&mut self) -> TableBuilderResult;
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone)]
pub struct CharacterTableIndex(Rc<RefCell<u8>>);
#[derive(Default, Debug, Clone)]
pub struct ColorTableIndex(Rc<RefCell<u8>>);
#[derive(Default, Debug, Clone)]
pub struct PixmapTableIndex(Rc<RefCell<u8>>);

#[allow(dead_code)]
#[derive(Default, Debug, Clone)]
pub struct PixmapIndex(PixmapTableIndex, Rc<RefCell<u8>>);

#[derive(Default, Debug, Clone)]
pub struct CharacterTableBuilder {
    pub use_advance_x: bool,
    pub use_pixmap_index: bool,

    pub constant_cluster_codepoints: Option<u8>,

    pub pixmap_table_indexes: Option<Vec<PixmapTableIndex>>,

    pub characters: Vec<CharacterBuilder>,
}

#[derive(Default, Debug, Clone)]
pub struct CharacterBuilder {
    pub advance_x: Option<u8>,
    pub pixmap_index: Option<PixmapIndex>,
    pub grapheme_cluster: String,
}

#[derive(Default, Debug, Clone)]
pub struct PixmapTableBuilder {
    pub constant_width: Option<u8>,
    pub constant_height: Option<u8>,
    pub constant_bits_per_pixel: Option<u8>,
    pub color_table_indexes: Option<Vec<ColorTableIndex>>,
    pub pixmaps: Vec<PixmapBuilder>,
    index: PixmapTableIndex,
}

#[derive(Default, Debug, Clone)]
pub struct PixmapBuilder {
    pub custom_width: Option<u8>,
    pub custom_height: Option<u8>,
    pub custom_bits_per_pixel: Option<u8>,
    pub data: Vec<u8>,
    index: Option<PixmapIndex>,
}

#[derive(Default, Debug, Clone)]
pub struct ColorTableBuilder {
    pub constant_alpha: Option<u8>,
    pub colors: Vec<ColorBuilder>,
    index: ColorTableIndex,
}

#[derive(Default, Debug, Clone)]
pub struct ColorBuilder {
    pub custom_alpha: Option<u8>,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl LayoutBuilder {
    /// Sets the [`Layout::compact`] field of the builder.
    pub fn compact(&mut self, compact: bool) -> &mut Self {
        self.compact = compact;
        self
    }

    /// Sets the [`Layout::version`] field of the builder.
    pub fn version(&mut self, version: Version) -> &mut Self {
        self.version = version;
        self
    }

    pub fn table<T: Into<TableBuilderIdentifier>>(&mut self, table: T) -> &mut Self {
        let table = table.into();
        match table {
            TableBuilderIdentifier::Character(mut table) => {
                table.resolve();
                self.character_table_builders.push(table);
            }
            TableBuilderIdentifier::Color(mut table) => {
                table.resolve();
                *table.index.0.borrow_mut() = self.color_table_builders.len() as u8;
                self.color_table_builders.push(table);
            }
            TableBuilderIdentifier::Pixmap(mut table) => {
                table.resolve();
                *table.index.0.borrow_mut() = self.pixmap_table_builders.len() as u8;
                self.pixmap_table_builders.push(table);
            }
        }
        self
    }

    pub fn build(&mut self) -> Layout {
        let mut layout = Layout {
            version: self.version.clone(),
            compact: self.compact,
            ..Default::default()
        };

        for character_table_builder in self.character_table_builders.iter_mut() {
            layout
                .character_tables
                .push(character_table_builder.build().into());
        }
        for color_table_builder in self.color_table_builders.iter_mut() {
            layout.color_tables.push(color_table_builder.build().into());
        }
        for pixmap_table_builder in self.pixmap_table_builders.iter_mut() {
            layout
                .pixmap_tables
                .push(pixmap_table_builder.build().into());
        }
        layout
    }
}
