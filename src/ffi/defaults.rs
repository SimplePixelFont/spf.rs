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

#![doc(hidden)]

use super::*;

impl Default for SPFLayout {
    fn default() -> Self {
        SPFLayout {
            version: u8::default(),
            compact: u8::default(),
            character_tables: core::ptr::null_mut(),
            character_tables_length: 0,
            color_tables: core::ptr::null_mut(),
            color_tables_length: 0,
            pixmap_tables: core::ptr::null_mut(),
            pixmap_tables_length: 0,
        }
    }
}

impl Default for SPFCharacterTable {
    fn default() -> Self {
        SPFCharacterTable {
            use_advance_x: u8::default(),
            use_pixmap_index: u8::default(),
            use_pixmap_table_index: u8::default(),
            has_constant_cluster_codepoints: u8::default(),
            constant_cluster_codepoints: u8::default(),
            has_pixmap_table_indexes: u8::default(),
            pixmap_table_indexes: core::ptr::null_mut(),
            pixmap_table_indexes_length: 0,
            characters: core::ptr::null_mut(),
            characters_length: 0,
        }
    }
}

impl Default for SPFCharacter {
    fn default() -> Self {
        SPFCharacter {
            has_advance_x: u8::default(),
            advance_x: u8::default(),
            has_pixmap_index: u8::default(),
            pixmap_index: u8::default(),
            has_pixmap_table_index: u8::default(),
            pixmap_table_index: u8::default(),
            grapheme_cluster: core::ptr::null_mut(),
        }
    }
}

impl Default for SPFColorTable {
    fn default() -> Self {
        SPFColorTable {
            has_constant_alpha: u8::default(),
            constant_alpha: u8::default(),
            colors: core::ptr::null_mut(),
            colors_length: 0,
        }
    }
}

#[allow(clippy::derivable_impls)] // For consistency & future developments in Color
impl Default for SPFColor {
    fn default() -> Self {
        SPFColor {
            has_custom_alpha: u8::default(),
            custom_alpha: u8::default(),
            r: u8::default(),
            g: u8::default(),
            b: u8::default(),
        }
    }
}

impl Default for SPFPixmapTable {
    fn default() -> Self {
        SPFPixmapTable {
            has_constant_width: u8::default(),
            constant_width: u8::default(),
            has_constant_height: u8::default(),
            constant_height: u8::default(),
            has_constant_bits_per_pixel: u8::default(),
            constant_bits_per_pixel: u8::default(),
            has_color_table_indexes: u8::default(),
            color_table_indexes: core::ptr::null_mut(),
            color_table_indexes_length: 0,
            pixmaps: core::ptr::null_mut(),
            pixmaps_length: 0,
        }
    }
}

impl Default for SPFPixmap {
    fn default() -> Self {
        SPFPixmap {
            has_custom_width: u8::default(),
            custom_width: u8::default(),
            has_custom_height: u8::default(),
            custom_height: u8::default(),
            has_custom_bits_per_pixel: u8::default(),
            custom_bits_per_pixel: u8::default(),
            data: core::ptr::null_mut(),
            data_length: 0,
        }
    }
}

impl Default for SPFData {
    fn default() -> Self {
        SPFData {
            data: core::ptr::null_mut(),
            data_length: 0,
        }
    }
}
