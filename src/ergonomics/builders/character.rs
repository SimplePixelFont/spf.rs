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

use std::cell::RefCell;
use std::rc::Rc;

pub(crate) use crate::core::*;
pub(crate) use crate::ergonomics::*;

// remove ToString trait
use crate::Vec;

impl CharacterBuilder {
    pub fn advance_x(&mut self, advance_x: u8) -> &mut Self {
        self.advance_x = Some(advance_x);
        self
    }
    pub fn pixmap_index(&mut self, pixmap_index: PixmapIndex) -> &mut Self {
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
