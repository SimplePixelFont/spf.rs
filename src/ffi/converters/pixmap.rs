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

impl TryFrom<Pixmap> for SPFPixmap {
    type Error = ConversionError;

    fn try_from(pixmap: Pixmap) -> Result<Self, Self::Error> {
        let (pixmap_ptr, pixmap_len) = vec_to_raw!(pixmap.data);
        Ok(SPFPixmap {
            has_custom_width: pixmap.custom_width.is_some() as c_uchar,
            custom_width: pixmap.custom_width.unwrap_or(0) as c_uchar,
            has_custom_height: pixmap.custom_height.is_some() as c_uchar,
            custom_height: pixmap.custom_height.unwrap_or(0) as c_uchar,
            has_custom_bits_per_pixel: pixmap.custom_bits_per_pixel.is_some() as c_uchar,
            custom_bits_per_pixel: pixmap.custom_bits_per_pixel.unwrap_or(0) as c_uchar,
            data: pixmap_ptr,
            data_length: pixmap_len as c_ulong,
        })
    }
}

impl TryInto<Pixmap> for &SPFPixmap {
    type Error = ConversionError;

    fn try_into(self) -> Result<Pixmap, Self::Error> {
        unsafe {
            let data = slice::from_raw_parts(self.data, self.data_length as usize).to_vec();
            let custom_width = ffi_to_option!(self.has_custom_width, self.custom_width);
            let custom_height = ffi_to_option!(self.has_custom_height, self.custom_height);
            let custom_bits_per_pixel =
                ffi_to_option!(self.has_custom_bits_per_pixel, self.custom_bits_per_pixel);

            Ok(Pixmap {
                custom_width,
                custom_height,
                custom_bits_per_pixel,
                data,
            })
        }
    }
}
