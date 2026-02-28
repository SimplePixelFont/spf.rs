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

impl TryFrom<CharacterTable> for SPFCharacterTable {
    type Error = ConversionError;

    fn try_from(table: CharacterTable) -> Result<Self, Self::Error> {
        let (pixmap_table_indexes_ptr, pixmap_table_indexes_len) =
            option_vec_to_raw!(table.pixmap_table_indexes);

        let (characters_ptr, characters_len) =
            vec_to_raw_with_conversion!(table.characters, SPFCharacter);

        Ok(SPFCharacterTable {
            use_advance_x: table.use_advance_x as c_uchar,
            use_pixmap_index: table.use_pixmap_index as c_uchar,
            use_pixmap_table_index: table.use_pixmap_table_index as c_uchar,
            has_constant_cluster_codepoints: table.constant_cluster_codepoints.is_some() as c_uchar,
            constant_cluster_codepoints: table.constant_cluster_codepoints.unwrap_or(0) as c_uchar,
            has_pixmap_table_indexes: table.pixmap_table_indexes.is_some() as c_uchar,
            pixmap_table_indexes: pixmap_table_indexes_ptr,
            pixmap_table_indexes_length: pixmap_table_indexes_len as c_ulong,
            characters: characters_ptr,
            characters_length: characters_len as c_ulong,
        })
    }
}

impl TryInto<CharacterTable> for &SPFCharacterTable {
    type Error = ConversionError;

    fn try_into(self) -> Result<CharacterTable, Self::Error> {
        unsafe {
            let pixmap_table_indexes = slice::from_raw_parts(
                self.pixmap_table_indexes,
                self.pixmap_table_indexes_length as usize,
            )
            .to_vec();

            let characters = vec_from_raw_with_conversion!(self.characters, self.characters_length);

            let constant_cluster_codepoints = ffi_to_option!(
                self.has_constant_cluster_codepoints,
                self.constant_cluster_codepoints
            );
            let pixmap_table_indexes =
                ffi_to_option!(self.has_pixmap_table_indexes, pixmap_table_indexes);

            Ok(CharacterTable {
                use_advance_x: self.use_advance_x != 0,
                use_pixmap_index: self.use_pixmap_index != 0,
                use_pixmap_table_index: self.use_pixmap_table_index != 0,
                constant_cluster_codepoints,
                pixmap_table_indexes,
                characters,
            })
        }
    }
}
