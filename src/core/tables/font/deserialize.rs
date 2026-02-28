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
use crate::core::{
    byte, DeserializeEngine, DeserializeError, Font, FontTable, FontType, TagWriter,
};
use crate::{vec, String, Vec};

#[cfg(feature = "tagging")]
use crate::tagging::{Span, TagKind};

impl FontTable {
    pub(crate) fn next_modifer_flags<R: ByteReader, T: TagWriter>(
        &mut self,
        engine: &mut DeserializeEngine<R, T>,
    ) {
        engine.bytes.next();
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::FontTableModifierFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![],
            engine.bytes.byte_index(),
        );
    }
    pub(crate) fn next_configurations<R: ByteReader, T: TagWriter>(
        &mut self,
        engine: &mut DeserializeEngine<R, T>,
    ) {
        engine.bytes.next();
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::FontTableConfigurationFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![],
            engine.bytes.byte_index(),
        );
    }
    pub(crate) fn next_table_links<R: ByteReader, T: TagWriter>(
        &mut self,
        engine: &mut DeserializeEngine<R, T>,
    ) -> Result<(), DeserializeError> {
        #[cfg(feature = "tagging")]
        let links_start = engine.bytes.byte_index();

        let link_flags = engine.bytes.next();
        let link_character_tables = byte::get_bit(link_flags, 0);

        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::FontTableLinkFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![TagKind::FontTableLinkCharacterTables {
                table_index: engine.tagging_data.current_table_index,
                value: link_character_tables,
            }],
            engine.bytes.byte_index(),
        );

        if link_character_tables {
            #[cfg(feature = "tagging")]
            let character_tables_start = engine.bytes.byte_index();

            let character_tables_length = engine.bytes.next();
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::FontTableCharacterTableIndexesLength {
                    table_index: engine.tagging_data.current_table_index,
                    count: character_tables_length,
                },
                engine.bytes.byte_index(),
            );

            #[cfg(feature = "tagging")]
            let character_table_indexes_start = engine.bytes.byte_index();

            let mut character_table_indexes = Vec::new();
            for _ in 0..character_tables_length {
                let link_index = engine.bytes.next();
                character_table_indexes.push(link_index);
                #[cfg(feature = "tagging")]
                engine.tags.tag_byte(
                    TagKind::FontTableCharacterTableIndex {
                        table_index: engine.tagging_data.current_table_index,
                        index: link_index,
                    },
                    engine.bytes.byte_index(),
                );
            }

            self.character_table_indexes = Some(character_table_indexes);

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

pub(crate) fn next_string<R: ByteReader, T: TagWriter>(
    engine: &mut DeserializeEngine<R, T>,
) -> String {
    let mut string = String::new();
    let mut end_cluster = false;

    while !end_cluster {
        let utf81 = engine.bytes.next();
        let mut utf8_bytes: [u8; 4] = [0, 0, 0, 0];

        if utf81 >> 7 == 0b00000000 {
            utf8_bytes[0] = utf81;
        } else if utf81 >> 5 == 0b00000110 {
            utf8_bytes[0] = utf81;
            utf8_bytes[1] = engine.bytes.next();
        } else if utf81 >> 4 == 0b00001110 {
            utf8_bytes[0] = utf81;
            utf8_bytes[1] = engine.bytes.next();
            utf8_bytes[2] = engine.bytes.next();
        } else if utf81 >> 3 == 0b00011110 {
            utf8_bytes[0] = utf81;
            utf8_bytes[1] = engine.bytes.next();
            utf8_bytes[2] = engine.bytes.next();
            utf8_bytes[3] = engine.bytes.next();
        }

        string.push(
            String::from_utf8(utf8_bytes.to_vec())
                .unwrap()
                .chars()
                .next()
                .unwrap(),
        );

        if engine.bytes.get() == 0 {
            end_cluster = true;
            engine.bytes.next();
        }
    }

    string
}

pub(crate) fn next_name<R: ByteReader, T: TagWriter>(
    engine: &mut DeserializeEngine<R, T>,
    font: &mut Font,
) {
    #[cfg(feature = "tagging")]
    let start = engine.bytes.byte_index();

    font.name = next_string(engine);

    #[cfg(feature = "tagging")]
    engine.tags.tag_span(
        TagKind::FontName {
            table_index: engine.tagging_data.current_table_index,
            font_index: engine.tagging_data.current_record_index,
            value: font.name.clone(),
        },
        Span::new(start, engine.bytes.byte_index()),
    );
}

pub(crate) fn next_author<R: ByteReader, T: TagWriter>(
    engine: &mut DeserializeEngine<R, T>,
    font: &mut Font,
) {
    #[cfg(feature = "tagging")]
    let start = engine.bytes.byte_index();

    font.author = next_string(engine);

    #[cfg(feature = "tagging")]
    engine.tags.tag_span(
        TagKind::FontAuthor {
            table_index: engine.tagging_data.current_table_index,
            font_index: engine.tagging_data.current_record_index,
            value: font.author.clone(),
        },
        Span::new(start, engine.bytes.byte_index()),
    );
}

pub(crate) fn next_version<R: ByteReader, T: TagWriter>(
    engine: &mut DeserializeEngine<R, T>,
    font: &mut Font,
) {
    let version = engine.bytes.next();
    #[cfg(feature = "tagging")]
    engine.tags.tag_byte(
        TagKind::FontVersion {
            table_index: engine.tagging_data.current_table_index,
            font_index: engine.tagging_data.current_record_index,
            value: version,
        },
        engine.bytes.byte_index(),
    );
    font.version = version;
}

pub(crate) fn next_font_type<R: ByteReader, T: TagWriter>(
    engine: &mut DeserializeEngine<R, T>,
    font: &mut Font,
) -> Result<(), DeserializeError> {
    let font_type = engine.bytes.next();
    let font_type = FontType::try_from(font_type)?;
    #[cfg(feature = "tagging")]
    engine.tags.tag_byte(
        TagKind::FontFontType {
            table_index: engine.tagging_data.current_table_index,
            font_index: engine.tagging_data.current_record_index,
            value: font_type,
        },
        engine.bytes.byte_index(),
    );
    font.font_type = font_type;

    Ok(())
}

pub(crate) fn next_character_table_indexes<R: ByteReader, T: TagWriter>(
    engine: &mut DeserializeEngine<R, T>,
    font: &mut Font,
) {
    #[cfg(feature = "tagging")]
    let start = engine.bytes.byte_index();

    let character_table_indexes_length = engine.bytes.next();
    #[cfg(feature = "tagging")]
    engine.tags.tag_byte(
        TagKind::FontCharacterTableIndexesLength {
            table_index: engine.tagging_data.current_table_index,
            font_index: engine.tagging_data.current_record_index,
            count: character_table_indexes_length,
        },
        engine.bytes.byte_index(),
    );

    let mut character_table_indexes = Vec::new();
    for _ in 0..character_table_indexes_length {
        let character_table_index = engine.bytes.next();
        character_table_indexes.push(character_table_index);
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::FontCharacterTableIndexesIndex {
                table_index: engine.tagging_data.current_table_index,
                font_index: engine.tagging_data.current_record_index,
                index: character_table_index,
            },
            engine.bytes.byte_index(),
        );
    }

    font.character_table_indexes = character_table_indexes;

    #[cfg(feature = "tagging")]
    engine.tags.tag_span(
        TagKind::FontCharacterTableIndexes {
            table_index: engine.tagging_data.current_table_index,
            font_index: engine.tagging_data.current_record_index,
            value: font.character_table_indexes.clone(),
        },
        Span::new(start, engine.bytes.byte_index()),
    );
}
