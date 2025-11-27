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

use crate::core::{Pixmap, SerializeEngine, SerializeError};
use crate::{format, String};

#[cfg(feature = "log")]
pub(crate) use log::*;

pub(crate) fn push_width(
    engine: &mut SerializeEngine,
    constant_width: Option<u8>,
    custom_width: Option<u8>,
) {
    if constant_width.is_none() {
        let width = custom_width.unwrap();
        engine.bytes.push(width);

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
    engine: &mut SerializeEngine,
    constant_height: Option<u8>,
    custom_height: Option<u8>,
) {
    if constant_height.is_none() {
        let height = custom_height.unwrap();
        engine.bytes.push(height);

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
    engine: &mut SerializeEngine,
    constant_bits_per_pixel: Option<u8>,
    custom_bits_per_pixel: Option<u8>,
) {
    if constant_bits_per_pixel.is_none() {
        let bits_per_pixel = custom_bits_per_pixel.unwrap();
        engine.bytes.push(bits_per_pixel);

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
    engine: &mut SerializeEngine,
    constant_width: Option<u8>,
    constant_height: Option<u8>,
    constant_bits_per_pixel: Option<u8>,
    pixmap: &Pixmap,
) -> Result<(), SerializeError> {
    let mut pixmap_bit_string = String::new();
    let mut bits_used: u64 = 0;

    let bits_per_pixel = constant_bits_per_pixel
        .or(pixmap.custom_bits_per_pixel)
        .unwrap();
    let width = constant_width.or(pixmap.custom_width).unwrap();
    let height = constant_height.or(pixmap.custom_height).unwrap();

    if pixmap.data.len() > width as usize * height as usize {
        return Err(SerializeError::StaticVectorTooLarge);
    }

    for pixel in pixmap.data.iter() {
        pixmap_bit_string.push_str(&format!(
            "{:0bits_per_pixel$b} ",
            pixel,
            bits_per_pixel = bits_per_pixel as usize
        ));
        engine.bytes.incomplete_push(*pixel, bits_per_pixel);
        bits_used += bits_per_pixel as u64;
    }

    if !engine.layout.compact && engine.bytes.pointer != 0 {
        engine.bytes.incomplete_push(0, 8 - (bits_used % 8) as u8);
    }

    #[cfg(feature = "log")]
    info!(
        "Pushed pixmap with the following bits: {}",
        pixmap_bit_string
    );
    Ok(())
}
