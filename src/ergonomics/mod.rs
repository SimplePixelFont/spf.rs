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
}

trait TableBuilder {
    fn build(&mut self) -> Box<impl Table>;
}

#[derive(Default, Debug, Clone)]
pub struct CharacterTableIndex(Rc<RefCell<u8>>);
#[derive(Default, Debug, Clone)]
pub struct ColorTableIndex(Rc<RefCell<u8>>);
#[derive(Default, Debug, Clone)]
pub struct PixmapTableIndex(Rc<RefCell<u8>>);

#[derive(Default, Debug, Clone)]
pub struct PixmapIndex(PixmapTableIndex, Rc<RefCell<u8>>);

#[derive(Default)]
pub struct ColorTableBuilder {
    pub constant_alpha: Option<u8>,
    pub colors: Vec<ColorBuilder>,
    index: ColorTableIndex,
}

#[derive(Default)]
pub struct CharacterTableBuilder {
    pub use_advance_x: bool,
    pub use_pixmap_index: bool,

    pub constant_cluster_codepoints: Option<u8>,

    pub pixmap_table_indexes: Option<Vec<PixmapTableIndex>>,

    pub characters: Vec<CharacterBuilder>,
}

#[derive(Default)]
pub struct CharacterBuilder {
    pub advance_x: Option<u8>,
    pub pixmap_index: Option<PixmapIndex>,
    pub grapheme_cluster: Option<String>,
}

#[derive(Default)]
pub struct PixmapTableBuilder {
    pub constant_width: Option<u8>,
    pub constant_height: Option<u8>,
    pub constant_bits_per_pixel: Option<u8>,
    pub color_table_indexes: Option<Vec<ColorTableIndex>>,
    pub pixmaps: Vec<PixmapBuilder>,
    index: PixmapTableIndex,
}

#[derive(Default)]
pub struct PixmapBuilder {
    pub custom_width: Option<u8>,
    pub custom_height: Option<u8>,
    pub custom_bits_per_pixel: Option<u8>,
    pub data: Vec<u8>,
    index: Option<PixmapIndex>,
}

#[derive(Default)]
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

    pub fn table(&mut self, table: Box<impl TableBuilder>) -> &mut Self {
        self
    }
}
