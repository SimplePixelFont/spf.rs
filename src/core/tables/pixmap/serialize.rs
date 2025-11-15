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

use crate::core::{byte, Pixmap, SerializeError};
use crate::{format, String};

#[cfg(feature = "log")]
pub(crate) use log::*;

pub(crate) fn push_width(
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

    let bits_per_pixel = constant_bits_per_pixel
        .or(pixmap.custom_bits_per_pixel)
        .unwrap();
    let width = constant_width.or(pixmap.custom_width).unwrap();
    let height = constant_height.or(pixmap.custom_height).unwrap();

    let bytes_used = (width as f32 * height as f32 * bits_per_pixel as f32 / 8.0).ceil() as usize;
    let complete_bytes_used = (pixels_used as f32 * bits_per_pixel as f32 / 8.0).floor() as usize;

    if pixmap.data.len() > bytes_used {
        return Err(SerializeError::StaticVectorTooLarge);
    }

    for index in 0..complete_bytes_used {
        buffer.push(pixmap.data[index]);
        pixmap_bit_string.push_str(&format!(
            "{:08b} ",
            pixmap.data[index],
        ));
    }

    let remainder_bits = ((width as u16 * height as u16 * bits_per_pixel as u16) % 8) as u8;
    if !compact && remainder_bits > 0 {
        buffer.push(pixmap.data[complete_bytes_used]);
        pixmap_bit_string.push_str(&format!(
            "{:08b} ",
            pixmap.data[complete_bytes_used],
        ));
    } else if remainder_bits > 0 {
        buffer.incomplete_push(pixmap.data[complete_bytes_used], remainder_bits);
        pixmap_bit_string.push_str(&format!(
            "{:08b} ",
            pixmap.data[complete_bytes_used],
        ));
    }

    #[cfg(feature = "log")]
    info!(
        "Pushed pixmap with the following bits: {}",
        pixmap_bit_string
    );
    Ok(())
}
