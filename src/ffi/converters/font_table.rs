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

impl TryFrom<FontTable> for SPFFontTable {
    type Error = ConversionError;

    fn try_from(table: FontTable) -> Result<Self, Self::Error> {
        let (character_table_indexes_ptr, character_table_indexes_len) =
            option_vec_to_raw!(table.character_table_indexes);

        let (fonts_ptr, fonts_len) = vec_to_raw_with_conversion!(table.fonts, SPFFont);

        Ok(SPFFontTable {
            has_character_table_indexes: table.character_table_indexes.is_some() as c_uchar,
            character_table_indexes: character_table_indexes_ptr,
            character_table_indexes_length: character_table_indexes_len as c_ulong,
            fonts: fonts_ptr,
            fonts_length: fonts_len as c_ulong,
        })
    }
}

impl TryInto<FontTable> for &SPFFontTable {
    type Error = ConversionError;

    fn try_into(self) -> Result<FontTable, Self::Error> {
        unsafe {
            let character_table_indexes = slice::from_raw_parts(
                self.character_table_indexes,
                self.character_table_indexes_length as usize,
            )
            .to_vec();
            let character_table_indexes =
                ffi_to_option!(self.has_character_table_indexes, character_table_indexes);

            let fonts = vec_from_raw_with_conversion!(self.fonts, self.fonts_length);

            Ok(FontTable {
                character_table_indexes,
                fonts,
            })
        }
    }
}
