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

use crate::core::byte::ByteReader;
#[cfg(feature = "tagging")]
use crate::core::{ByteIndex, Span, TableType, TagKind};
use crate::core::{
    Character, CharacterTable, DeserializeEngine, DeserializeError, SerializeEngine,
    SerializeError, Table, TagWriter,
};

pub(crate) mod deserialize;
pub(crate) use deserialize::*;
pub(crate) mod serialize;
pub(crate) use serialize::*;

impl Table for CharacterTable {
    fn deserialize<R: ByteReader, T: TagWriter>(
        engine: &mut DeserializeEngine<R, T>,
    ) -> Result<Self, DeserializeError> {
        #[cfg(feature = "tagging")]
        let table_start = engine.bytes.byte_index();
        #[cfg(feature = "tagging")]
        let table_start = ByteIndex::new(table_start.byte - 1, table_start.bit);
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::TableIdentifier {
                table_type: TableType::Character,
            },
            engine.bytes.byte_index(),
        );

        let mut character_table = CharacterTable::default();
        character_table.next_modifer_flags(engine);
        character_table.next_configurations(engine);
        character_table.next_table_links(engine)?;

        let character_count = engine.bytes.next();
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::CharacterTableCharacterCount {
                table_index: engine.tagging_data.current_table_index,
                count: character_count,
            },
            engine.bytes.byte_index(),
        );

        for index in 0..character_count {
            #[cfg(feature = "tagging")]
            {
                engine.tagging_data.current_record_index = index;
            }
            #[cfg(feature = "tagging")]
            let character_start = engine.bytes.byte_index();

            let mut character = Character::default();
            if character_table.use_advance_x {
                character.advance_x = Some(engine.bytes.next());
                #[cfg(feature = "tagging")]
                engine.tags.tag_byte(
                    TagKind::CharacterAdvanceX {
                        table_index: engine.tagging_data.current_table_index,
                        char_index: engine.tagging_data.current_record_index,
                        value: character.advance_x.unwrap(),
                    },
                    engine.bytes.byte_index(),
                );
            }
            if character_table.use_pixmap_index {
                character.pixmap_index = Some(engine.bytes.next());
                #[cfg(feature = "tagging")]
                engine.tags.tag_byte(
                    TagKind::CharacterPixmapIndex {
                        table_index: engine.tagging_data.current_table_index,
                        char_index: engine.tagging_data.current_record_index,
                        value: character.pixmap_index.unwrap(),
                    },
                    engine.bytes.byte_index(),
                );
            }
            if character_table.use_pixmap_table_index {
                character.pixmap_table_index = Some(engine.bytes.next());
                #[cfg(feature = "tagging")]
                engine.tags.tag_byte(
                    TagKind::CharacterPixmapTableIndex {
                        table_index: engine.tagging_data.current_table_index,
                        char_index: engine.tagging_data.current_record_index,
                        value: character.pixmap_table_index.unwrap(),
                    },
                    engine.bytes.byte_index(),
                );
            }

            next_grapheme_cluster(
                engine,
                &mut character,
                character_table.constant_cluster_codepoints,
            );
            character_table.characters.push(character);

            #[cfg(feature = "tagging")]
            engine.tags.tag_span(
                TagKind::CharacterRecord {
                    table_index: engine.tagging_data.current_table_index,
                    char_index: engine.tagging_data.current_record_index,
                },
                Span::new(character_start, engine.bytes.byte_index()),
            );
        }

        #[cfg(feature = "tagging")]
        engine.tags.tag_span(
            TagKind::CharacterTable {
                index: engine.tagging_data.current_table_index,
            },
            Span::new(table_start, engine.bytes.byte_index()),
        );

        Ok(character_table)
    }

    fn serialize<T: TagWriter>(
        &self,
        engine: &mut SerializeEngine<T>,
    ) -> Result<(), crate::core::SerializeError> {
        #[cfg(feature = "tagging")]
        let table_start = engine.bytes.byte_index();

        self.push_table_identifier(engine);
        self.push_modifier_flags(engine);
        self.push_configurations(engine);
        self.push_table_links(engine)?;

        // record length
        let character_count = self.characters.len();
        if character_count > 255 {
            return Err(SerializeError::StaticVectorTooLarge);
        }
        engine.bytes.push(character_count as u8);
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::CharacterTableCharacterCount {
                table_index: engine.tagging_data.current_table_index,
                count: character_count as u8,
            },
            engine.bytes.byte_index(),
        );

        // records
        for (index, character) in self.characters.iter().enumerate() {
            #[cfg(feature = "tagging")]
            {
                engine.tagging_data.current_record_index = index as u8;
            }
            #[cfg(feature = "tagging")]
            let character_start = engine.bytes.byte_index();

            if self.use_advance_x {
                engine.bytes.push(character.advance_x.unwrap());
                #[cfg(feature = "tagging")]
                engine.tags.tag_byte(
                    TagKind::CharacterAdvanceX {
                        table_index: engine.tagging_data.current_table_index,
                        char_index: engine.tagging_data.current_record_index,
                        value: character.advance_x.unwrap(),
                    },
                    engine.bytes.byte_index(),
                );
            }
            if self.use_pixmap_index {
                engine.bytes.push(character.pixmap_index.unwrap());
                #[cfg(feature = "tagging")]
                engine.tags.tag_byte(
                    TagKind::CharacterPixmapIndex {
                        table_index: engine.tagging_data.current_table_index,
                        char_index: engine.tagging_data.current_record_index,
                        value: character.pixmap_index.unwrap(),
                    },
                    engine.bytes.byte_index(),
                );
            }
            if self.use_pixmap_table_index {
                engine.bytes.push(character.pixmap_table_index.unwrap());
                #[cfg(feature = "tagging")]
                engine.tags.tag_byte(
                    TagKind::CharacterPixmapTableIndex {
                        table_index: engine.tagging_data.current_table_index,
                        char_index: engine.tagging_data.current_record_index,
                        value: character.pixmap_table_index.unwrap(),
                    },
                    engine.bytes.byte_index(),
                );
            }

            push_grapheme_cluster(
                engine,
                self.constant_cluster_codepoints,
                &character.grapheme_cluster,
            );

            #[cfg(feature = "tagging")]
            engine.tags.tag_span(
                TagKind::CharacterRecord {
                    table_index: engine.tagging_data.current_table_index,
                    char_index: engine.tagging_data.current_record_index,
                },
                Span::new(character_start, engine.bytes.byte_index()),
            );
        }

        #[cfg(feature = "tagging")]
        engine.tags.tag_span(
            TagKind::CharacterTable {
                index: engine.tagging_data.current_table_index,
            },
            Span::new(table_start, engine.bytes.byte_index()),
        );
        Ok(())
    }
}
