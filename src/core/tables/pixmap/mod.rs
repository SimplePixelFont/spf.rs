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

pub(crate) mod deserialize;
pub(crate) mod serialize;

use crate::core::{
    DeserializeEngine, Pixmap, PixmapTable, SerializeEngine, SerializeError, Table, TagWriter,
};

#[cfg(feature = "tagging")]
use crate::core::{ByteIndex, Span, TableType, TagKind};

pub(crate) use deserialize::*;
pub(crate) use serialize::*;

impl Table for PixmapTable {
    fn deserialize<T: TagWriter>(
        engine: &mut DeserializeEngine<T>,
    ) -> Result<Self, crate::core::DeserializeError> {
        #[cfg(feature = "tagging")]
        let table_start = engine.bytes.byte_index();
        #[cfg(feature = "tagging")]
        let table_start = ByteIndex::new(table_start.byte - 1, table_start.bit);
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::TableIdentifier {
                table_type: TableType::Pixmap,
            },
            engine.bytes.byte_index(),
        );

        let mut pixmap_table = PixmapTable::default();
        pixmap_table.next_modifier_flags(engine);
        pixmap_table.next_configurations(engine);
        pixmap_table.next_table_links(engine);

        let pixmap_count = engine.bytes.next();
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::PixmapTablePixmapCount {
                table_index: engine.tagging_data.current_table_index,
                count: pixmap_count,
            },
            engine.bytes.byte_index(),
        );

        for _ in 0..pixmap_count {
            #[cfg(feature = "tagging")]
            {
                engine.tagging_data.current_record_index = engine.bytes.index as u8;
            }
            #[cfg(feature = "tagging")]
            let pixmap_start = engine.bytes.byte_index();

            let mut pixmap = Pixmap::default();
            next_width(engine, &mut pixmap, pixmap_table.constant_width);
            next_height(engine, &mut pixmap, pixmap_table.constant_height);
            next_bits_per_pixel(engine, &mut pixmap, pixmap_table.constant_bits_per_pixel);
            next_pixmap(
                engine,
                &mut pixmap,
                pixmap_table.constant_width,
                pixmap_table.constant_height,
                pixmap_table.constant_bits_per_pixel,
            );
            pixmap_table.pixmaps.push(pixmap);

            #[cfg(feature = "tagging")]
            engine.tags.tag_span(
                TagKind::PixmapRecord {
                    table_index: engine.tagging_data.current_table_index,
                    pixmap_index: engine.tagging_data.current_record_index,
                },
                Span::new(pixmap_start, engine.bytes.byte_index()),
            );
        }

        #[cfg(feature = "tagging")]
        engine.tags.tag_span(
            TagKind::PixmapTable {
                index: engine.tagging_data.current_table_index,
            },
            Span::new(table_start, engine.bytes.byte_index()),
        );

        Ok(pixmap_table)
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

        if self.pixmaps.len() > 255 {
            return Err(SerializeError::StaticVectorTooLarge);
        }
        engine.bytes.push(self.pixmaps.len() as u8);
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::PixmapTablePixmapCount {
                table_index: engine.tagging_data.current_table_index,
                count: self.pixmaps.len() as u8,
            },
            engine.bytes.byte_index(),
        );
        for (index, pixmap) in self.pixmaps.iter().enumerate() {
            #[cfg(feature = "tagging")]
            {
                engine.tagging_data.current_record_index = index as u8;
            }
            #[cfg(feature = "tagging")]
            let pixmap_start = engine.bytes.byte_index();

            push_width(engine, self.constant_width, pixmap.custom_width);
            push_height(engine, self.constant_height, pixmap.custom_height);
            push_bits_per_pixel(
                engine,
                self.constant_bits_per_pixel,
                pixmap.custom_bits_per_pixel,
            );
            push_pixmap(
                engine,
                self.constant_width,
                self.constant_height,
                self.constant_bits_per_pixel,
                pixmap,
            )?;

            #[cfg(feature = "tagging")]
            engine.tags.tag_span(
                TagKind::PixmapRecord {
                    table_index: engine.tagging_data.current_table_index,
                    pixmap_index: engine.tagging_data.current_record_index,
                },
                Span::new(pixmap_start, engine.bytes.byte_index()),
            );
        }

        #[cfg(feature = "tagging")]
        engine.tags.tag_span(
            TagKind::PixmapTable {
                index: engine.tagging_data.current_table_index,
            },
            Span::new(table_start, engine.bytes.byte_index()),
        );

        Ok(())
    }
}
