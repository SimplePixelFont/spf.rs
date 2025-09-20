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
    fn build(&self) -> Box<impl Table>;
}

#[derive(Default, Debug, Clone)]
pub struct ColorTableIndex(Rc<RefCell<u8>>);

#[derive(Default)]
pub struct ColorTableBuilder {
    pub constant_alpha: Option<u8>,
    pub colors: Vec<ColorBuilder>,
    index: ColorTableIndex,
}

#[derive(Default)]
pub struct ColorBuilder {
    pub custom_alpha: Option<u8>,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorTableBuilder {
    pub fn constant_alpha(&mut self, constant_alpha: u8) -> &mut Self {
        self.constant_alpha = Some(constant_alpha);
        self
    }
    pub fn color<T: Into<ColorBuilder>>(&mut self, color: T) -> &mut Self {
        // if self.constant_alpha.is_some() {
        //     color.custom_alpha = None;
        // }
        // if self.constant_alpha.is_none() && color.custom_alpha.is_none() {
        //     panic!("Neither constant_alpha nor custom_alpha are set!");
        // }
        self.colors.push(color.into());
        self
    }
    pub fn link(&mut self) -> ColorTableIndex {
        self.index.clone()
    }
    // pub fn set_index(&mut self, index: u8) {
    //     *self.index.0.borrow_mut() = index;
    // }
}

impl TableBuilder for ColorTableBuilder {
    fn build(&self) -> Box<impl Table> {
        Box::new(ColorTable {
            constant_alpha: self.constant_alpha,
            colors: vec![],
        })
    }
}

impl From<&[u8]> for ColorBuilder {
    fn from(rgba: &[u8]) -> Self {
        ColorBuilder {
            custom_alpha: Some(rgba.get(3).unwrap_or(&0).to_owned()),
            r: rgba.get(0).unwrap_or(&0).to_owned(),
            g: rgba.get(1).unwrap_or(&0).to_owned(),
            b: rgba.get(2).unwrap_or(&0).to_owned(),
        }
    }
}

impl ColorBuilder {
    pub fn rgba(&mut self, r: u8, g: u8, b: u8, a: u8) -> &mut Self {
        self.custom_alpha = Some(a);
        self.r = r;
        self.g = g;
        self.b = b;
        self
    }
    pub fn rgb(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.custom_alpha = None;
        self.r = r;
        self.g = g;
        self.b = b;
        self
    }
    pub fn a(&mut self, a: u8) -> &mut Self {
        self.custom_alpha = Some(a);
        self
    }
    pub fn r(&mut self, r: u8) -> &mut Self {
        self.r = r;
        self
    }
    pub fn g(&mut self, g: u8) -> &mut Self {
        self.g = g;
        self
    }
    pub fn b(&mut self, b: u8) -> &mut Self {
        self.b = b;
        self
    }
    pub fn black() -> Self {
        ColorBuilder {
            custom_alpha: Some(255),
            r: 0,
            g: 0,
            b: 0,
        }
    }
    pub fn white() -> Self {
        ColorBuilder {
            custom_alpha: Some(255),
            r: 255,
            g: 255,
            b: 255,
        }
    }
    pub fn transparent() -> Self {
        ColorBuilder {
            custom_alpha: Some(0),
            r: 0,
            g: 0,
            b: 0,
        }
    }
}

#[derive(Default)]
pub struct CharacterBuilder {
    pub advance_x: Option<u8>,
    pub pixmap_index: Option<u8>,
    pub grapheme_cluster: Option<String>,
}

impl CharacterBuilder {
    pub fn advance_x(&mut self, advance_x: u8) -> &mut Self {
        self.advance_x = Some(advance_x);
        self
    }
    pub fn pixmap_index(&mut self, pixmap_index: u8) -> &mut Self {
        self.pixmap_index = Some(pixmap_index);
        self
    }
    pub fn grapheme_cluster(&mut self, grapheme_cluster: String) -> &mut Self {
        self.grapheme_cluster = Some(grapheme_cluster);
        self
    }
}

impl From<&str> for CharacterBuilder {
    fn from(grapheme_cluster: &str) -> Self {
        CharacterBuilder {
            advance_x: None,
            pixmap_index: None,
            grapheme_cluster: Some(grapheme_cluster.to_string()),
        }
    }
}

#[derive(Default)]
pub struct CharacterTableBuilder {
    pub use_advance_x: bool,
    pub use_pixmap_index: bool,

    pub constant_cluster_codepoints: Option<u8>,

    pub pixmap_table_indexes: Option<Vec<PixmapTableIndex>>,

    pub characters: Vec<CharacterBuilder>,
}

impl CharacterTableBuilder {
    pub fn advance_x(&mut self, use_advance_x: bool) -> &mut Self {
        self.use_advance_x = use_advance_x;
        self
    }
    pub fn pixmap_index(&mut self, use_pixmap_index: bool) -> &mut Self {
        self.use_pixmap_index = use_pixmap_index;
        self
    }
    pub fn constant_cluster_codepoints(&mut self, constant_cluster_codepoints: u8) -> &mut Self {
        self.constant_cluster_codepoints = Some(constant_cluster_codepoints);
        self
    }
    pub fn pixmap_table_indexes(&mut self, pixmap_table_indexes: &[PixmapTableIndex]) -> &mut Self {
        if self.pixmap_table_indexes.is_none() {
            self.pixmap_table_indexes = Some(Vec::new());
        }
        self.pixmap_table_indexes
            .as_mut()
            .unwrap()
            .append(pixmap_table_indexes.to_vec().as_mut());
        self
    }
    pub fn character<T: Into<CharacterBuilder>>(&mut self, character: T) -> &mut Self {
        self.characters.push(character.into());
        self
    }
}

// impl TableBuilder for CharacterTableBuilder {
//     fn build(&self) -> Box<impl Table> {
//         Box::new(CharacterTable {
//             use_advance_x: self.use_advance_x,
//             use_pixmap_index: self.use_pixmap_index,
//             constant_cluster_codepoints: self.constant_cluster_codepoints,
//             pixmap_table_indexes: self.pixmap_table_indexes.clone(),
//             characters: vec![],
//         })
//     }
// }

#[derive(Default)]
pub struct PixmapBuilder {
    pub custom_width: Option<u8>,
    pub custom_height: Option<u8>,
    pub custom_bits_per_pixel: Option<u8>,
    pub data: Vec<u8>,
}

impl From<&[u8]> for PixmapBuilder {
    fn from(data: &[u8]) -> Self {
        PixmapBuilder {
            custom_width: None,
            custom_height: None,
            custom_bits_per_pixel: None,
            data: data.to_vec(),
        }
    }
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

#[derive(Default, Debug, Clone)]
pub struct PixmapTableIndex(Rc<RefCell<u8>>);

impl PixmapTableBuilder {
    pub fn constant_width(&mut self, constant_width: u8) -> &mut Self {
        self.constant_width = Some(constant_width);
        self
    }
    pub fn constant_height(&mut self, constant_height: u8) -> &mut Self {
        self.constant_height = Some(constant_height);
        self
    }
    pub fn constant_bits_per_pixel(&mut self, constant_bits_per_pixel: u8) -> &mut Self {
        self.constant_bits_per_pixel = Some(constant_bits_per_pixel);
        self
    }
    pub fn color_table_indexes(&mut self, color_table_indexes: &[ColorTableIndex]) -> &mut Self {
        if self.color_table_indexes.is_none() {
            self.color_table_indexes = Some(Vec::new());
        }
        self.color_table_indexes
            .as_mut()
            .unwrap()
            .append(color_table_indexes.to_vec().as_mut());
        self
    }
    pub fn pixmap<T: Into<PixmapBuilder>>(&mut self, pixmap: T) -> &mut Self {
        self.pixmaps.push(pixmap.into());
        self
    }
    pub fn link(&mut self) -> PixmapTableIndex {
        self.index.clone()
    }
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
