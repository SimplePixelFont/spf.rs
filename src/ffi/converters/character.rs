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

impl TryFrom<Character> for SPFCharacter {
    type Error = ConversionError;

    fn try_from(character: Character) -> Result<Self, Self::Error> {
        let code_points = CString::new(character.code_points.as_str())?;
        let code_points_ptr = code_points.into_raw();

        Ok(SPFCharacter {
            has_advance_x: character.advance_x.is_some() as c_uchar,
            advance_x: character.advance_x.unwrap_or(0) as c_uchar,
            has_pixmap_index: character.pixmap_index.is_some() as c_uchar,
            pixmap_index: character.pixmap_index.unwrap_or(0) as c_uchar,
            has_pixmap_table_index: character.pixmap_table_index.is_some() as c_uchar,
            pixmap_table_index: character.pixmap_table_index.unwrap_or(0) as c_uchar,
            code_points: code_points_ptr,
        })
    }
}

impl TryInto<Character> for &SPFCharacter {
    type Error = ConversionError;

    fn try_into(self) -> Result<Character, Self::Error> {
        unsafe {
            let code_points = CStr::from_ptr(self.code_points)
                .to_str()?
                .to_owned();
            let advance_x = ffi_to_option!(self.has_advance_x, self.advance_x);
            let pixmap_index = ffi_to_option!(self.has_pixmap_index, self.pixmap_index);
            let pixmap_table_index =
                ffi_to_option!(self.has_pixmap_table_index, self.pixmap_table_index);

            Ok(Character {
                advance_x,
                pixmap_index,
                pixmap_table_index,
                code_points,
            })
        }
    }
}
