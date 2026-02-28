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

impl TryFrom<ColorTable> for SPFColorTable {
    type Error = ConversionError;

    fn try_from(table: ColorTable) -> Result<Self, Self::Error> {
        let (colors_ptr, colors_len) = vec_to_raw_with_conversion!(table.colors, SPFColor);

        Ok(SPFColorTable {
            use_color_type: table.use_color_type as c_uchar,
            has_constant_alpha: table.constant_alpha.is_some() as c_uchar,
            constant_alpha: table.constant_alpha.unwrap_or(0) as c_uchar,
            colors: colors_ptr,
            colors_length: colors_len as c_ulong,
        })
    }
}

impl TryInto<ColorTable> for &SPFColorTable {
    type Error = ConversionError;

    fn try_into(self) -> Result<ColorTable, Self::Error> {
        unsafe {
            let colors = vec_from_raw_with_conversion!(self.colors, self.colors_length);
            let constant_alpha = ffi_to_option!(self.has_constant_alpha, self.constant_alpha);

            Ok(ColorTable {
                use_color_type: self.use_color_type != 0,
                constant_alpha,
                colors,
            })
        }
    }
}
