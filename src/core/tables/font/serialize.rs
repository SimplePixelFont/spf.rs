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
    FontTable, FontType, SerializeEngine, SerializeError, TableIdentifier, TagWriter,
};
use crate::vec;

#[cfg(feature = "tagging")]
use crate::tagging::{Span, TagKind};

impl FontTable {
    pub(crate) fn push_table_identifier<T: TagWriter>(&self, engine: &mut SerializeEngine<T>) {
        engine.bytes.push(TableIdentifier::Font as u8);
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::TableIdentifier {
                table_type: crate::core::TableType::Font,
            },
            engine.bytes.byte_index(),
        );
    }
    pub(crate) fn push_modifier_flags<T: TagWriter>(&self, engine: &mut SerializeEngine<T>) {
        engine.bytes.push(0b00000000);
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::FontTableModifierFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![],
            engine.bytes.byte_index(),
        );
    }
    pub(crate) fn push_configurations<T: TagWriter>(&self, engine: &mut SerializeEngine<T>) {
        engine.bytes.push(0b00000000);
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::FontTableConfigurationFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![],
            engine.bytes.byte_index(),
        );
    }

    pub(crate) fn push_table_links<T: TagWriter>(
        &self,
        engine: &mut SerializeEngine<T>,
    ) -> Result<(), SerializeError> {
        #[cfg(feature = "tagging")]
        let links_start = engine.bytes.byte_index();

        // Table Links
        let mut link_flags = 0b00000000;
        if self.character_table_indexes.is_some() {
            link_flags |= 0b00000001;
        }

        // Table relations
        engine.bytes.push(link_flags);
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::FontTableLinkFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![TagKind::FontTableLinkCharacterTables {
                table_index: engine.tagging_data.current_table_index,
                value: self.character_table_indexes.is_some(),
            }],
            engine.bytes.byte_index(),
        );

        if let Some(character_table_indexes) = &self.character_table_indexes {
            #[cfg(feature = "tagging")]
            let character_tables_start = engine.bytes.byte_index();

            let character_tables_length = character_table_indexes.len();
            if character_tables_length > 255 {
                return Err(SerializeError::StaticVectorTooLarge);
            }

            engine.bytes.push(character_tables_length as u8);
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::FontTableCharacterTableIndexesLength {
                    table_index: engine.tagging_data.current_table_index,
                    count: character_tables_length as u8,
                },
                engine.bytes.byte_index(),
            );

            #[cfg(feature = "tagging")]
            let character_table_indexes_start = engine.bytes.byte_index();

            for character_table_index in character_table_indexes {
                engine.bytes.push(*character_table_index);
                #[cfg(feature = "tagging")]
                engine.tags.tag_byte(
                    TagKind::FontTableCharacterTableIndex {
                        table_index: engine.tagging_data.current_table_index,
                        index: *character_table_index,
                    },
                    engine.bytes.byte_index(),
                );
            }

            #[cfg(feature = "tagging")]
            engine.tags.tag_span(
                TagKind::FontTableCharacterTableIndexes {
                    table_index: engine.tagging_data.current_table_index,
                    indexes: self.character_table_indexes.as_ref().unwrap().clone(),
                },
                Span::new(character_table_indexes_start, engine.bytes.byte_index()),
            );

            #[cfg(feature = "tagging")]
            engine.tags.tag_span(
                TagKind::FontTableCharacterTableLinks {
                    table_index: engine.tagging_data.current_table_index,
                },
                Span::new(character_tables_start, engine.bytes.byte_index()),
            );
        }

        #[cfg(feature = "tagging")]
        engine.tags.tag_span(
            TagKind::FontTableLinks {
                table_index: engine.tagging_data.current_table_index,
            },
            Span::new(links_start, engine.bytes.byte_index()),
        );
        Ok(())
    }
}

pub(crate) fn push_string<T: TagWriter>(engine: &mut SerializeEngine<T>, string: &str) {
    string.bytes().for_each(|byte| {
        engine.bytes.push(byte);
    });
    engine.bytes.push(0);
}

pub(crate) fn push_name<T: TagWriter>(engine: &mut SerializeEngine<T>, string: &str) {
    #[cfg(feature = "tagging")]
    let string_start = engine.bytes.byte_index();

    push_string(engine, string);

    #[cfg(feature = "tagging")]
    engine.tags.tag_span(
        TagKind::FontName {
            table_index: engine.tagging_data.current_table_index,
            font_index: engine.tagging_data.current_record_index,
            value: string.to_owned(),
        },
        Span::new(string_start, engine.bytes.byte_index()),
    );
}

pub(crate) fn push_author<T: TagWriter>(engine: &mut SerializeEngine<T>, string: &str) {
    #[cfg(feature = "tagging")]
    let string_start = engine.bytes.byte_index();

    push_string(engine, string);

    #[cfg(feature = "tagging")]
    engine.tags.tag_span(
        TagKind::FontAuthor {
            table_index: engine.tagging_data.current_table_index,
            font_index: engine.tagging_data.current_record_index,
            value: string.to_owned(),
        },
        Span::new(string_start, engine.bytes.byte_index()),
    );
}

pub(crate) fn push_version<T: TagWriter>(engine: &mut SerializeEngine<T>, version: u8) {
    engine.bytes.push(version);
    #[cfg(feature = "tagging")]
    engine.tags.tag_byte(
        TagKind::FontVersion {
            table_index: engine.tagging_data.current_table_index,
            font_index: engine.tagging_data.current_record_index,
            value: version,
        },
        engine.bytes.byte_index(),
    );
}

pub(crate) fn push_font_type<T: TagWriter>(engine: &mut SerializeEngine<T>, font_type: FontType) {
    engine.bytes.push(font_type as u8);
    #[cfg(feature = "tagging")]
    engine.tags.tag_byte(
        TagKind::FontFontType {
            table_index: engine.tagging_data.current_table_index,
            font_index: engine.tagging_data.current_record_index,
            value: font_type,
        },
        engine.bytes.byte_index(),
    );
}

pub(crate) fn push_character_table_indexes<T: TagWriter>(
    engine: &mut SerializeEngine<T>,
    character_table_indexes: &Vec<u8>,
) -> Result<(), SerializeError> {
    #[cfg(feature = "tagging")]
    let start = engine.bytes.byte_index();

    let character_table_indexes_length = character_table_indexes.len();
    if character_table_indexes_length > 255 {
        return Err(SerializeError::StaticVectorTooLarge);
    }

    engine.bytes.push(character_table_indexes_length as u8);
    #[cfg(feature = "tagging")]
    engine.tags.tag_byte(
        TagKind::FontCharacterTableIndexesLength {
            table_index: engine.tagging_data.current_table_index,
            font_index: engine.tagging_data.current_record_index,
            count: character_table_indexes_length as u8,
        },
        engine.bytes.byte_index(),
    );

    for character_table_index in character_table_indexes {
        engine.bytes.push(*character_table_index);
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::FontCharacterTableIndexesIndex {
                table_index: engine.tagging_data.current_table_index,
                font_index: engine.tagging_data.current_record_index,
                index: *character_table_index,
            },
            engine.bytes.byte_index(),
        );
    }

    #[cfg(feature = "tagging")]
    engine.tags.tag_span(
        TagKind::FontCharacterTableIndexes {
            table_index: engine.tagging_data.current_table_index,
            font_index: engine.tagging_data.current_record_index,
            value: character_table_indexes.clone(),
        },
        Span::new(start, engine.bytes.byte_index()),
    );

    Ok(())
}
