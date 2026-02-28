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

use crate::core::{byte::ByteReader, Font};
#[cfg(feature = "tagging")]
use crate::core::{ByteIndex, Span, TableType, TagKind};
use crate::core::{
    DeserializeEngine, DeserializeError, FontTable, SerializeEngine, SerializeError, Table,
    TagWriter,
};

pub(crate) mod deserialize;
pub(crate) use deserialize::*;
pub(crate) mod serialize;
pub(crate) use serialize::*;

impl Table for FontTable {
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
                table_type: TableType::Font,
            },
            engine.bytes.byte_index(),
        );

        let mut font_table = FontTable::default();
        font_table.next_modifer_flags(engine);
        font_table.next_configurations(engine);
        font_table.next_table_links(engine)?;

        let font_count = engine.bytes.next();
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::FontTableFontCount {
                table_index: engine.tagging_data.current_table_index,
                count: font_count,
            },
            engine.bytes.byte_index(),
        );

        for index in 0..font_count {
            #[cfg(feature = "tagging")]
            {
                engine.tagging_data.current_record_index = index;
            }
            #[cfg(feature = "tagging")]
            let font_start = engine.bytes.byte_index();

            let mut font = Font::default();

            next_name(engine, &mut font);
            next_author(engine, &mut font);
            next_version(engine, &mut font);
            next_font_type(engine, &mut font)?;
            next_character_table_indexes(engine, &mut font);
            font_table.fonts.push(font);

            #[cfg(feature = "tagging")]
            engine.tags.tag_span(
                TagKind::FontRecord {
                    table_index: engine.tagging_data.current_table_index,
                    font_index: engine.tagging_data.current_record_index,
                },
                Span::new(font_start, engine.bytes.byte_index()),
            );
        }

        #[cfg(feature = "tagging")]
        engine.tags.tag_span(
            TagKind::FontTable {
                index: engine.tagging_data.current_table_index,
            },
            Span::new(table_start, engine.bytes.byte_index()),
        );

        Ok(font_table)
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
        let font_count = self.fonts.len();
        if font_count > 255 {
            return Err(SerializeError::StaticVectorTooLarge);
        }
        engine.bytes.push(font_count as u8);
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::FontTableFontCount {
                table_index: engine.tagging_data.current_table_index,
                count: font_count as u8,
            },
            engine.bytes.byte_index(),
        );

        // records
        for (index, font) in self.fonts.iter().enumerate() {
            #[cfg(feature = "tagging")]
            {
                engine.tagging_data.current_record_index = index as u8;
            }
            #[cfg(feature = "tagging")]
            let font_start = engine.bytes.byte_index();

            push_name(engine, &font.name);
            push_author(engine, &font.author);
            push_version(engine, font.version);
            push_font_type(engine, font.font_type);
            push_character_table_indexes(engine, &font.character_table_indexes)?;

            #[cfg(feature = "tagging")]
            engine.tags.tag_span(
                TagKind::FontRecord {
                    table_index: engine.tagging_data.current_table_index,
                    font_index: engine.tagging_data.current_record_index,
                },
                Span::new(font_start, engine.bytes.byte_index()),
            );
        }

        #[cfg(feature = "tagging")]
        engine.tags.tag_span(
            TagKind::FontTable {
                index: engine.tagging_data.current_table_index,
            },
            Span::new(table_start, engine.bytes.byte_index()),
        );
        Ok(())
    }
}
