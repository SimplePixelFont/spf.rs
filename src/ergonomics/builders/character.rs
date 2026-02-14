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

pub(crate) use crate::ergonomics::*;

use crate::Vec;

impl CharacterBuilder {
    pub fn advance_x(&mut self, advance_x: u8) -> &mut Self {
        self.advance_x = Some(advance_x);
        self
    }
    pub fn pixmap_index(&mut self, pixmap_index: &PixmapIndex) -> &mut Self {
        self.pixmap_index = Some(pixmap_index.clone());
        self
    }
    pub fn grapheme_cluster(&mut self, grapheme_cluster: String) -> &mut Self {
        self.grapheme_cluster = grapheme_cluster;
        self
    }
}

impl From<&str> for CharacterBuilder {
    fn from(grapheme_cluster: &str) -> Self {
        CharacterBuilder {
            advance_x: None,
            pixmap_index: None,
            grapheme_cluster: grapheme_cluster.to_string(),
        }
    }
}

impl From<&mut CharacterBuilder> for CharacterBuilder {
    fn from(character: &mut CharacterBuilder) -> Self {
        CharacterBuilder {
            advance_x: character.advance_x,
            pixmap_index: character.pixmap_index.clone(),
            grapheme_cluster: character.grapheme_cluster.clone(),
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
    pub fn character_process<
        T: Into<CharacterBuilder>,
        F: Fn(&mut CharacterBuilder) -> &mut CharacterBuilder,
    >(
        &mut self,
        character: T,
        process: F,
    ) -> &mut Self {
        self.character(process(&mut character.into()));
        self
    }
}

impl From<CharacterTableBuilder> for TableBuilderIdentifier {
    fn from(character_table_builder: CharacterTableBuilder) -> Self {
        TableBuilderIdentifier::Character(character_table_builder)
    }
}

impl TableBuilder for CharacterTableBuilder {
    fn resolve(&mut self) {
        for character_builder in self.characters.iter_mut() {
            if !self.use_advance_x {
                character_builder.advance_x = None;
            }
            if !self.use_pixmap_index {
                character_builder.pixmap_index = None;
            }
            if self.use_advance_x && character_builder.advance_x.is_none() {
                panic!("use_advance_x is set to true but no advance_x is set!");
            }
            if self.use_pixmap_index && character_builder.pixmap_index.is_none() {
                panic!("use_pixmap_index is set to true but no pixmap_index is set!");
            }
        }
    }
    fn build(&mut self) -> TableBuilderResult {
        let mut character_table = CharacterTable {
            use_advance_x: self.use_advance_x,
            use_pixmap_index: self.use_pixmap_index,
            constant_cluster_codepoints: self.constant_cluster_codepoints,
            ..Default::default()
        };

        let pixmap_table_indexes = self
            .pixmap_table_indexes
            .as_ref()
            .map(|pixmap_table_indexes| {
                pixmap_table_indexes
                    .iter()
                    .map(|pixmap_table_index| *pixmap_table_index.0.borrow())
                    .collect()
            });

        character_table.pixmap_table_indexes = pixmap_table_indexes;

        let mut characters = vec![];
        for character_builder in self.characters.iter_mut() {
            characters.push(Character {
                advance_x: character_builder.advance_x,
                pixmap_index: character_builder
                    .pixmap_index
                    .as_ref()
                    .map(|pixmap_index| *pixmap_index.1.borrow()),
                pixmap_table_index: None,
                grapheme_cluster: character_builder.grapheme_cluster.clone(),
            });
        }
        character_table.characters = characters;

        TableBuilderResult::Character(character_table)
    }
}
