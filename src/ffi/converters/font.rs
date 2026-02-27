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

impl TryFrom<Font> for SPFFont {
    type Error = ConversionError;

    fn try_from(font: Font) -> Result<Self, Self::Error> {
        let name = CString::new(font.name.as_str())?;
        let name_ptr = name.into_raw();
        let author = CString::new(font.author.as_str())?;
        let author_ptr = author.into_raw();
        let (character_table_indexes_ptr, character_table_indexes_len) =
            vec_to_raw!(font.character_table_indexes);

        Ok(SPFFont {
            name: name_ptr,
            author: author_ptr,
            version: font.version as c_uchar,
            font_type: font.font_type as c_uchar,
            character_table_indexes: character_table_indexes_ptr,
            character_tables_indexes_length: character_table_indexes_len as c_ulong,
        })
    }
}

impl TryInto<Font> for &SPFFont {
    type Error = ConversionError;

    fn try_into(self) -> Result<Font, Self::Error> {
        unsafe {
            let name = CString::from_raw(self.name).to_str()?.to_owned();
            let author = CString::from_raw(self.author).to_str()?.to_owned();

            let character_table_indexes = slice::from_raw_parts(
                self.character_table_indexes,
                self.character_tables_indexes_length as usize,
            )
            .to_vec();

            Ok(Font {
                name,
                author,
                version: self.version,
                font_type: FontType::try_from(self.font_type).unwrap_or_default(),
                character_table_indexes,
            })
        }
    }
}
