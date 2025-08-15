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

use crate::core::{Color, ColorTable, SerializeError, Table, TableIdentifier};

impl Table for ColorTable {
    fn deserialize(
        storage: &mut crate::core::byte::ByteStorage,
    ) -> Result<Self, crate::core::ParseError> {
        let mut color_table = ColorTable::default();

        storage.next(); // Skip modifieres
        let table_property_flags = storage.next();
        if crate::core::byte::get_bit(table_property_flags, 0) {
            color_table.constant_alpha = Some(storage.next());
        }
        storage.next(); // Skip table links

        let color_count = storage.get();
        for _ in 0..color_count {
            let mut color = Color::default();
            if color_table.constant_alpha.is_none() {
                color.custom_alpha = Some(storage.next());
            }
            color.r = storage.next();
            color.g = storage.next();
            color.b = storage.next();
            color_table.colors.push(color);
        }

        Ok(color_table)
    }

    fn serialize(
        &self,
        buffer: &mut crate::core::byte::ByteStorage,
    ) -> Result<(), crate::core::SerializeError> {
        buffer.push(TableIdentifier::ColorTable as u8);

        let mut table_property_flags = 0b00000000;
        let mut table_property_values = Vec::new();

        if self.constant_alpha.is_some() {
            table_property_flags |= 0b00000001;
            table_property_values.push(self.constant_alpha.unwrap());
        }

        buffer.push(0b00000000); // Modifiers byte
        buffer.push(table_property_flags); // Configuration flags byte
        buffer.append(&table_property_values); // Configuration values
        buffer.push(0b00000000); // Table relations byte

        if self.colors.len() > 255 {
            return Err(SerializeError::StaticVectorTooLarge);
        }
        buffer.push(self.colors.len() as u8);
        for color in &self.colors {
            if self.constant_alpha.is_none() {
                buffer.push(color.custom_alpha.unwrap());
            }
            buffer.push(color.r);
            buffer.push(color.g);
            buffer.push(color.b);
        }

        Ok(())
    }
}
