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

#[derive(Default)]
pub struct ColorTableBuilder {
    pub constant_alpha: Option<u8>,
    pub colors: Vec<Color>,
}

impl ColorTableBuilder {
    pub fn constant_alpha(&mut self, constant_alpha: u8) -> &mut Self {
        self.constant_alpha = Some(constant_alpha);
        self
    }
    pub fn rgb(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        if self.constant_alpha.is_none() {
            panic!("constant_alpha must be set before adding colors without alpha channel, use self.rgba() instead.");
        }
        self.colors.push(Color {
            custom_alpha: None,
            r,
            g,
            b,
        });
        self
    }
    pub fn rgba(&mut self, r: u8, g: u8, b: u8, a: u8) -> &mut Self {
        if self.constant_alpha.is_some() {
            panic!("constant_alpha is set, use self.rgb() instead.");
        }
        self.colors.push(Color {
            custom_alpha: Some(a),
            r,
            g,
            b,
        });
        self
    }
}

impl TableBuilder for ColorTableBuilder {
    fn build(&self) -> Box<impl Table> {
        Box::new(ColorTable {
            constant_alpha: self.constant_alpha,
            colors: self.colors.clone(),
        })
    }
}

#[derive(Default)]
pub struct CharacterBuilder {
    pub advance_x: Option<u8>,
    pub pixmap_index: Option<u8>,
    pub cluster_codepoints: Option<String>,
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
    pub fn cluster_codepoints(&mut self, cluster_codepoints: String) -> &mut Self {
        self.cluster_codepoints = Some(cluster_codepoints);
        self
    }
}

impl From<&str> for CharacterBuilder {
    fn from(cluster_codepoints: &str) -> Self {
        CharacterBuilder {
            advance_x: None,
            pixmap_index: None,
            cluster_codepoints: Some(cluster_codepoints.to_string()),
        }
    }
}

#[derive(Default)]
pub struct CharacterTableBuilder {
    pub use_advance_x: bool,
    pub use_pixmap_index: bool,

    pub constant_cluster_codepoints: Option<u8>,

    pub pixmap_table_indexes: Option<Vec<u8>>,

    pub characters: Vec<Character>,
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
    pub fn pixmap_table_indexes(&mut self, pixmap_table_indexes: &[u8]) -> &mut Self {
        if self.pixmap_table_indexes.is_none() {
            self.pixmap_table_indexes = Some(Vec::new());
        }
        self.pixmap_table_indexes
            .as_mut()
            .unwrap()
            .append(pixmap_table_indexes.to_vec().as_mut());
        self
    }
}

impl TableBuilder for CharacterTableBuilder {
    fn build(&self) -> Box<impl Table> {
        Box::new(CharacterTable {
            use_advance_x: self.use_advance_x,
            use_pixmap_index: self.use_pixmap_index,
            constant_cluster_codepoints: self.constant_cluster_codepoints,
            pixmap_table_indexes: self.pixmap_table_indexes.clone(),
            characters: self.characters.clone(),
        })
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
