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

use super::*;

impl TryFrom<PixmapTable> for SPFPixmapTable {
    type Error = ConversionError;

    fn try_from(table: PixmapTable) -> Result<Self, Self::Error> {
        let (color_table_indexes_ptr, color_table_indexes_len) =
            option_vec_to_raw!(table.color_table_indexes);

        let (pixmaps_ptr, pixmaps_len) = vec_to_raw_with_conversion!(table.pixmaps, SPFPixmap);

        Ok(SPFPixmapTable {
            has_constant_width: table.constant_width.is_some() as c_uchar,
            constant_width: table.constant_width.unwrap_or(0) as c_uchar,
            has_constant_height: table.constant_height.is_some() as c_uchar,
            constant_height: table.constant_height.unwrap_or(0) as c_uchar,
            has_constant_bits_per_pixel: table.constant_bits_per_pixel.is_some() as c_uchar,
            constant_bits_per_pixel: table.constant_bits_per_pixel.unwrap_or(0) as c_uchar,
            has_color_table_indexes: table.color_table_indexes.is_some() as c_uchar,
            color_table_indexes: color_table_indexes_ptr,
            color_table_indexes_length: color_table_indexes_len as c_ulong,
            pixmaps: pixmaps_ptr,
            pixmaps_length: pixmaps_len as c_ulong,
        })
    }
}

impl TryInto<PixmapTable> for &SPFPixmapTable {
    type Error = ConversionError;

    fn try_into(self) -> Result<PixmapTable, Self::Error> {
        unsafe {
            let color_table_indexes = slice::from_raw_parts(
                self.color_table_indexes,
                self.color_table_indexes_length as usize,
            )
            .to_vec();
            let color_table_indexes =
                ffi_to_option!(self.has_color_table_indexes, color_table_indexes);

            let pixmaps = vec_from_raw_with_conversion!(self.pixmaps, self.pixmaps_length);
            let constant_width = ffi_to_option!(self.has_constant_width, self.constant_width);
            let constant_height = ffi_to_option!(self.has_constant_height, self.constant_height);
            let constant_bits_per_pixel = ffi_to_option!(
                self.has_constant_bits_per_pixel,
                self.constant_bits_per_pixel
            );

            Ok(PixmapTable {
                constant_width,
                constant_height,
                constant_bits_per_pixel,
                color_table_indexes,
                pixmaps,
            })
        }
    }
}
