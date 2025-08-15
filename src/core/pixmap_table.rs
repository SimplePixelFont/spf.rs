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

use crate::core::{ParseError, Pixmap, PixmapTable, SerializeError, Table, TableIdentifier};
use crate::Vec;

#[cfg(feature = "log")]
pub(crate) use log::*;

impl Table for PixmapTable {
    fn deserialize(storage: &mut crate::core::byte::ByteStorage) -> Result<Self, ParseError> {
        let pixmap_table = PixmapTable::default();

        storage.index -= 1;
        storage.next(); // Skip modifieres
        let table_property_flags = storage.next();
        if crate::core::byte::get_bit(table_property_flags, 0) {
            pixmap_table.constant_width = Some(storage.next());
        }
        if crate::core::byte::get_bit(table_property_flags, 1) {
            pixmap_table.constant_height = Some(storage.next());
        }
        if crate::core::byte::get_bit(table_property_flags, 2) {
            pixmap_table.constant_bits_per_pixel = Some(storage.next());
        }

        let table_link_flags = storage.next(); // Skip table links
        if crate::core::byte::get_bit(table_link_flags, 0) {
            let color_table_indicies_length = storage.next();
            let mut color_table_indicies = vec![];
            for color_table_index in 0..color_table_indicies_length {
                color_table_indicies.push(storage.next());
            }
            pixmap_table.color_tables_indices = Some(color_table_indicies);
        }

        let pixmap_count = storage.get();
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

        Ok(pixmap_table)
    }
    fn serialize(&self, buffer: &mut crate::core::byte::ByteStorage) -> Result<(), SerializeError> {
        buffer.push(TableIdentifier::PixmapTable as u8);

        buffer.push(0b00000000); // Modifiers Byte

        let mut table_property_flags = 0b00000000;
        let mut table_property_values = Vec::new();

        // Configuration flags
        if self.constant_width.is_some() {
            table_property_flags |= 0b00000001;
            table_property_values.push(self.constant_width.unwrap());
        }
        if self.constant_height.is_some() {
            table_property_flags |= 0b00000010;
            table_property_values.push(self.constant_height.unwrap());
        }
        if self.constant_bits_per_pixel.is_some() {
            table_property_flags |= 0b00000100;
            table_property_values.push(self.constant_bits_per_pixel.unwrap());
        }

        buffer.push(table_property_flags);
        buffer.append(&table_property_values);

        // Table Links
        let mut table_link_flags = 0b00000000;
        let mut table_link_bytes = Vec::new();
        if self.color_tables_indices.is_some() {
            table_link_flags |= 0b00000001;
            let colors_tables_length = self.color_tables_indices.as_ref().unwrap().len();
            if colors_tables_length > 255 {
                return Err(SerializeError::StaticVectorTooLarge);
            }
            buffer.push(colors_tables_length as u8);
            for table_index in self.color_tables_indices.as_ref().unwrap() {
                table_link_bytes.push(*table_index);
            }
        }

        // Table relations
        buffer.push(table_link_flags);
        buffer.append(&table_link_bytes);

        if self.pixmaps.len() > 255 {
            return Err(SerializeError::StaticVectorTooLarge);
        }
        buffer.push(self.pixmaps.len() as u8);
        for pixmap in self.pixmaps.iter() {
            push_width(buffer, self.constant_width, pixmap.custom_width);
            push_height(buffer, self.constant_height, pixmap.custom_height);
            push_bits_per_pixel(
                buffer,
                self.constant_bits_per_pixel,
                pixmap.custom_bits_per_pixel,
            );
            push_pixmap(
                buffer,
                true,
                self.constant_width,
                self.constant_height,
                self.constant_bits_per_pixel,
                pixmap,
            );
        }

        Ok(())
    }
}

pub(crate) fn push_width<'a>(
    buffer: &mut byte::ByteStorage,
    constant_width: Option<u8>,
    custom_width: Option<u8>,
) {
    if constant_width.is_none() {
        let width = custom_width.unwrap();
        buffer.push(width);

        #[cfg(feature = "log")]
        {
            let width_bit_string = format!("{:08b}", width);
            info!(
                "Pushed character width '{}' with the following bits: {}",
                width, width_bit_string
            )
        }
    }
}

pub(crate) fn push_height(
    buffer: &mut byte::ByteStorage,
    constant_height: Option<u8>,
    custom_height: Option<u8>,
) {
    if constant_height.is_none() {
        let height = custom_height.unwrap();
        buffer.push(height);

        #[cfg(feature = "log")]
        {
            let height_bit_string = format!("{:08b}", height);
            info!(
                "Pushed character height '{}' with the following bits: {}",
                height, height_bit_string
            )
        }
    }
}

pub(crate) fn push_bits_per_pixel(
    buffer: &mut byte::ByteStorage,
    constant_bits_per_pixel: Option<u8>,
    custom_bits_per_pixel: Option<u8>,
) {
    if constant_bits_per_pixel.is_none() {
        let bits_per_pixel = custom_bits_per_pixel.unwrap();
        buffer.push(bits_per_pixel);

        #[cfg(feature = "log")]
        {
            let bits_per_pixel_bit_string = format!("{:08b}", bits_per_pixel);
            info!(
                "Pushed character bits_per_pixel '{}' with the following bits: {}",
                bits_per_pixel, bits_per_pixel_bit_string
            )
        }
    }
}

pub(crate) fn push_pixmap(
    buffer: &mut byte::ByteStorage,
    compact: bool,
    constant_width: Option<u8>,
    constant_height: Option<u8>,
    constant_bits_per_pixel: Option<u8>,
    pixmap: &Pixmap,
) -> Result<(), SerializeError> {
    let mut pixmap_bit_string = String::new();
    let mut bits_used = 0;

    let bits_per_pixel = constant_bits_per_pixel.unwrap_or(pixmap.custom_bits_per_pixel.unwrap());
    let width = constant_width.unwrap_or(pixmap.custom_width.unwrap());
    let height = constant_height.unwrap_or(pixmap.custom_height.unwrap());

    if pixmap.data.len() > width as usize * height as usize {
        return Err(SerializeError::StaticVectorTooLarge);
    }

    for pixel in pixmap.data.iter() {
        pixmap_bit_string.push_str(&format!(
            "{:0bits_per_pixel$b} ",
            pixel,
            bits_per_pixel = bits_per_pixel as usize
        ));
        buffer.incomplete_push(*pixel, bits_per_pixel);
        bits_used += bits_per_pixel;
    }

    if !compact {
        buffer.incomplete_push(0, 8 - (bits_used % 8));
    }

    #[cfg(feature = "log")]
    info!(
        "Pushed pixmap with the following bits: {}",
        pixmap_bit_string
    );
    Ok(())
}

pub(crate) fn next_width(
    storage: &mut byte::ByteStorage,
    header: &Header,
    character: &mut Character,
) -> u8 {
    let current_character_width;
    if !header.configuration_flags.constant_width {
        character.custom_width = Some(storage.get());
        current_character_width = character.custom_width.unwrap();
        storage.index += 1;

        #[cfg(feature = "log")]
        info!("Identified custom width: {:?}", current_character_width);
    } else {
        current_character_width = header.configuration_values.constant_width.unwrap();
    }

    current_character_width
}

pub(crate) fn next_height(
    storage: &mut byte::ByteStorage,
    header: &Header,
    character: &mut Character,
) -> u8 {
    let current_character_height;
    if !header.configuration_flags.constant_height {
        character.custom_height = Some(storage.get());
        current_character_height = character.custom_height.unwrap();
        storage.index += 1;

        #[cfg(feature = "log")]
        info!("Identified custom height: {:?}", current_character_height);
    } else {
        current_character_height = header.configuration_values.constant_height.unwrap();
    }

    current_character_height
}

pub(crate) fn next_pixmap(
    storage: &mut byte::ByteStorage,
    header: &Header,
    character: &mut Character,
    width: u8,
    height: u8,
    bits_per_pixel: u8,
) -> usize {
    let pixels_used = width * height;
    let mut current_bit = storage.pointer;
    for _ in 0..pixels_used {
        let pixel = storage.incomplete_get(bits_per_pixel);
        character.pixmap.push(pixel);
        current_bit += bits_per_pixel;
        if current_bit >= 8 {
            storage.index += 1;
            current_bit = 0;
        }
        storage.pointer = current_bit;
    }

    if !header.modifier_flags.compact && (width * height * bits_per_pixel) % 8 != 0 {
        storage.index += 1;
        storage.pointer = 0;
    }

    #[cfg(feature = "log")]
    info!("Identified pixmap: {:?}", character.pixmap);
}
