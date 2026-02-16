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

impl TryFrom<Color> for SPFColor {
    type Error = ConversionError;

    fn try_from(color: Color) -> Result<Self, Self::Error> {
        Ok(SPFColor {
            has_color_type: color.color_type.is_some() as c_uchar,
            color_type: color.color_type.unwrap_or(ColorType::Dynamic) as c_uchar,
            has_custom_alpha: color.custom_alpha.is_some() as c_uchar,
            custom_alpha: color.custom_alpha.unwrap_or(0) as c_uchar,
            r: color.r as c_uchar,
            g: color.g as c_uchar,
            b: color.b as c_uchar,
        })
    }
}

impl TryInto<Color> for &SPFColor {
    type Error = ConversionError;

    fn try_into(self) -> Result<Color, Self::Error> {
        let color_type = ffi_to_option!(
            self.has_color_type,
            // original C value can be lost!
            ColorType::try_from(self.color_type).unwrap_or_default()
        );
        let custom_alpha = ffi_to_option!(self.has_custom_alpha, self.custom_alpha);
        Ok(Color {
            color_type,
            custom_alpha,
            r: self.r,
            g: self.g,
            b: self.b,
        })
    }
}
