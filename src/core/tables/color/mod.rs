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
    Color, ColorTable, DeserializeError, Layout, SerializeError, Table, TableIdentifier,
};
use crate::Vec;

impl Table for ColorTable {
    fn deserialize(
        storage: &mut crate::core::byte::ByteStorage,
        _layout: &Layout,
    ) -> Result<Self, DeserializeError> {
        let mut color_table = ColorTable::default();

        storage.next(); // Skip modifires byte
        let configuration_flags = storage.next();
        if crate::core::byte::get_bit(configuration_flags, 0) {
            color_table.constant_alpha = Some(storage.next());
        }
        storage.next(); // Skip table links

        let color_count = storage.next();
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
        _layout: &Layout,
    ) -> Result<(), crate::core::SerializeError> {
        buffer.push(TableIdentifier::Color as u8);

        buffer.push(0b00000000); // Modifiers byte

        let mut configuration_flags = 0b00000000;
        let mut configuration_values = Vec::new();

        if self.constant_alpha.is_some() {
            configuration_flags |= 0b00000001;
            configuration_values.push(self.constant_alpha.unwrap());
        }
        buffer.push(configuration_flags); // Configuration flags byte
        buffer.append(&configuration_values); // Configuration values
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
