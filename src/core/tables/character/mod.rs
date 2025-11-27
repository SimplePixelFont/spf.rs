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

use crate::core::{
    byte, Character, CharacterTable, DeserializeEngine, DeserializeError, SerializeEngine,
    SerializeError, Table, TableIdentifier, TagWriter,
};
use crate::{vec, Vec};

pub(crate) mod deserialize;
pub(crate) use deserialize::*;
pub(crate) mod serialize;
pub(crate) use serialize::*;

impl Table for CharacterTable {
    fn deserialize<T: TagWriter>(
        engine: &mut DeserializeEngine<T>,
    ) -> Result<Self, DeserializeError> {
        let mut character_table = CharacterTable::default();

        let modifier_flags = engine.bytes.next();
        if byte::get_bit(modifier_flags, 0) {
            character_table.use_advance_x = true;
        }
        if byte::get_bit(modifier_flags, 1) {
            character_table.use_pixmap_index = true;
        }

        let configuration_flags = engine.bytes.next();
        if byte::get_bit(configuration_flags, 0) {
            character_table.constant_cluster_codepoints = Some(engine.bytes.next());
        }

        let links_flags = engine.bytes.next();
        if byte::get_bit(links_flags, 0) {
            let pixmap_table_indexes_length = engine.bytes.next();
            let mut pixmap_table_indexes = vec![];
            for _ in 0..pixmap_table_indexes_length {
                pixmap_table_indexes.push(engine.bytes.next());
            }
            character_table.pixmap_table_indexes = Some(pixmap_table_indexes);
        }

        let character_count = engine.bytes.next();
        for _ in 0..character_count {
            let mut character = Character::default();
            if character_table.use_advance_x {
                character.advance_x = Some(engine.bytes.next());
            }
            if character_table.use_pixmap_index {
                character.pixmap_index = Some(engine.bytes.next());
            }
            next_grapheme_cluster(
                engine,
                &mut character,
                character_table.constant_cluster_codepoints,
            );
            character_table.characters.push(character);
        }

        Ok(character_table)
    }

    fn serialize<T: TagWriter>(
        &self,
        engine: &mut SerializeEngine<T>,
    ) -> Result<(), crate::core::SerializeError> {
        engine.bytes.push(TableIdentifier::Character as u8);

        let mut modifier_flags = 0b00000000;
        if self.use_advance_x {
            modifier_flags |= 0b00000001;
        }
        if self.use_pixmap_index {
            modifier_flags |= 0b00000010;
        }
        engine.bytes.push(modifier_flags);

        let mut configuration_flags = 0b00000000;
        let mut configuration_values = Vec::new();

        if self.constant_cluster_codepoints.is_some() {
            configuration_flags |= 0b00000001;
            configuration_values.push(self.constant_cluster_codepoints.unwrap());
        }
        engine.bytes.push(configuration_flags); // Configuration flags byte
        engine.bytes.append(&configuration_values); // Configuration values

        // Table Links
        let mut link_flags = 0b00000000;
        let mut link_bytes = Vec::new();
        if self.pixmap_table_indexes.is_some() {
            link_flags |= 0b00000001;
            let pixmap_tables_length = self.pixmap_table_indexes.as_ref().unwrap().len();
            if pixmap_tables_length > 255 {
                return Err(SerializeError::StaticVectorTooLarge);
            }
            engine.bytes.push(pixmap_tables_length as u8);
            for pixmap_table_index in self.pixmap_table_indexes.as_ref().unwrap() {
                link_bytes.push(*pixmap_table_index);
            }
        }

        // Table relations
        engine.bytes.push(link_flags);
        engine.bytes.append(&link_bytes);

        if self.characters.len() > 255 {
            return Err(SerializeError::StaticVectorTooLarge);
        }
        engine.bytes.push(self.characters.len() as u8);
        for character in &self.characters {
            if self.use_advance_x {
                engine.bytes.push(character.advance_x.unwrap());
            }
            if self.use_pixmap_index {
                engine.bytes.push(character.pixmap_index.unwrap());
            }
            push_grapheme_cluster(
                engine,
                self.constant_cluster_codepoints,
                &character.grapheme_cluster,
            );
        }

        Ok(())
    }
}
