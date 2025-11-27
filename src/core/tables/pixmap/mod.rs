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
    DeserializeEngine, Pixmap, PixmapTable, SerializeEngine, SerializeError, Table, TableIdentifier,
};
use crate::{vec, Vec};
pub(crate) use deserialize::*;
pub(crate) use serialize::*;

impl Table for PixmapTable {
    fn deserialize(engine: &mut DeserializeEngine) -> Result<Self, crate::core::DeserializeError> {
        let mut pixmap_table = PixmapTable::default();

        engine.bytes.next(); // Skip modifieres
        let configuration_flags = engine.bytes.next();
        if crate::core::byte::get_bit(configuration_flags, 0) {
            pixmap_table.constant_width = Some(engine.bytes.next());
        }
        if crate::core::byte::get_bit(configuration_flags, 1) {
            pixmap_table.constant_height = Some(engine.bytes.next());
        }
        if crate::core::byte::get_bit(configuration_flags, 2) {
            pixmap_table.constant_bits_per_pixel = Some(engine.bytes.next());
        }

        let links_flags = engine.bytes.next();
        if crate::core::byte::get_bit(links_flags, 0) {
            let color_table_indexes_length = engine.bytes.next();
            let mut color_table_indexes = vec![];
            for _ in 0..color_table_indexes_length {
                color_table_indexes.push(engine.bytes.next());
            }
            pixmap_table.color_table_indexes = Some(color_table_indexes);
        }

        let pixmap_count = engine.bytes.next();
        for _ in 0..pixmap_count {
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
        }

        Ok(pixmap_table)
    }
    fn serialize(&self, engine: &mut SerializeEngine) -> Result<(), crate::core::SerializeError> {
        engine.bytes.push(TableIdentifier::Pixmap as u8);

        engine.bytes.push(0b00000000); // Modifiers Byte

        let mut configuration_flags = 0b00000000;
        let mut configuration_values = Vec::new();

        // Configuration flags
        if self.constant_width.is_some() {
            configuration_flags |= 0b00000001;
            configuration_values.push(self.constant_width.unwrap());
        }
        if self.constant_height.is_some() {
            configuration_flags |= 0b00000010;
            configuration_values.push(self.constant_height.unwrap());
        }
        if self.constant_bits_per_pixel.is_some() {
            configuration_flags |= 0b00000100;
            configuration_values.push(self.constant_bits_per_pixel.unwrap());
        }

        engine.bytes.push(configuration_flags);
        engine.bytes.append(&configuration_values);

        // Table Links
        let mut link_flags = 0b00000000;
        let mut link_bytes = Vec::new();
        if self.color_table_indexes.is_some() {
            link_flags |= 0b00000001;
            let colors_tables_length = self.color_table_indexes.as_ref().unwrap().len();
            if colors_tables_length > 255 {
                return Err(SerializeError::StaticVectorTooLarge);
            }
            engine.bytes.push(colors_tables_length as u8);
            for color_table_index in self.color_table_indexes.as_ref().unwrap() {
                link_bytes.push(*color_table_index);
            }
        }

        // Table relations
        engine.bytes.push(link_flags);
        engine.bytes.append(&link_bytes);

        if self.pixmaps.len() > 255 {
            return Err(SerializeError::StaticVectorTooLarge);
        }
        engine.bytes.push(self.pixmaps.len() as u8);
        for pixmap in self.pixmaps.iter() {
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
        }

        Ok(())
    }
}
