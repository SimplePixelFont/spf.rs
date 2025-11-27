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
    Color, ColorTable, DeserializeEngine, DeserializeError, SerializeEngine, SerializeError, Table,
    TableIdentifier, TagWriter,
};
use crate::Vec;

impl Table for ColorTable {
    fn deserialize<T: TagWriter>(
        engine: &mut DeserializeEngine<T>,
    ) -> Result<Self, DeserializeError> {
        let mut color_table = ColorTable::default();

        engine.bytes.next(); // Skip modifires byte
        let configuration_flags = engine.bytes.next();
        if crate::core::byte::get_bit(configuration_flags, 0) {
            color_table.constant_alpha = Some(engine.bytes.next());
        }
        engine.bytes.next(); // Skip table links

        let color_count = engine.bytes.next();
        for _ in 0..color_count {
            let mut color = Color::default();
            if color_table.constant_alpha.is_none() {
                color.custom_alpha = Some(engine.bytes.next());
            }
            color.r = engine.bytes.next();
            color.g = engine.bytes.next();
            color.b = engine.bytes.next();
            color_table.colors.push(color);
        }

        Ok(color_table)
    }

    fn serialize<T: TagWriter>(
        &self,
        engine: &mut SerializeEngine<T>,
    ) -> Result<(), crate::core::SerializeError> {
        engine.bytes.push(TableIdentifier::Color as u8);

        engine.bytes.push(0b00000000); // Modifiers byte

        let mut configuration_flags = 0b00000000;
        let mut configuration_values = Vec::new();

        if self.constant_alpha.is_some() {
            configuration_flags |= 0b00000001;
            configuration_values.push(self.constant_alpha.unwrap());
        }
        engine.bytes.push(configuration_flags); // Configuration flags byte
        engine.bytes.append(&configuration_values); // Configuration values
        engine.bytes.push(0b00000000); // Table relations byte

        if self.colors.len() > 255 {
            return Err(SerializeError::StaticVectorTooLarge);
        }
        engine.bytes.push(self.colors.len() as u8);
        for color in &self.colors {
            if self.constant_alpha.is_none() {
                engine.bytes.push(color.custom_alpha.unwrap());
            }
            engine.bytes.push(color.r);
            engine.bytes.push(color.g);
            engine.bytes.push(color.b);
        }

        Ok(())
    }
}
