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

use crate::core::{byte, Pixmap};

#[cfg(feature = "log")]
use log::*;

pub(crate) fn next_width(
    storage: &mut byte::ByteStorage,
    pixmap: &mut Pixmap,
    constant_width: Option<u8>,
) {
    if constant_width.is_none() {
        pixmap.custom_width = Some(storage.next());

        #[cfg(feature = "log")]
        info!("Identified custom width: {:?}", pixmap.custom_width);
    }
}

pub(crate) fn next_height(
    storage: &mut byte::ByteStorage,
    pixmap: &mut Pixmap,
    constant_height: Option<u8>,
) {
    if constant_height.is_none() {
        pixmap.custom_height = Some(storage.next());

        #[cfg(feature = "log")]
        info!("Identified custom height: {:?}", pixmap.custom_height);
    }
}

pub(crate) fn next_bits_per_pixel(
    storage: &mut byte::ByteStorage,
    pixmap: &mut Pixmap,
    constant_bits_per_pixel: Option<u8>,
) {
    if constant_bits_per_pixel.is_none() {
        pixmap.custom_bits_per_pixel = Some(storage.next());

        #[cfg(feature = "log")]
        info!(
            "Identified custom bits per pixel: {:?}",
            pixmap.custom_bits_per_pixel
        );
    }
}

pub(crate) fn next_pixmap(
    storage: &mut byte::ByteStorage,
    pixmap: &mut Pixmap,
    compact: bool,
    constant_width: Option<u8>,
    constant_height: Option<u8>,
    constant_bits_per_pixel: Option<u8>,
) {
    let bits_per_pixel = constant_bits_per_pixel
        .or(pixmap.custom_bits_per_pixel)
        .unwrap();
    let width = constant_width.or(pixmap.custom_width).unwrap();
    let height = constant_height.or(pixmap.custom_height).unwrap();

    let pixels_used = width as u16 * height as u16;
    let complete_bytes_used = (pixels_used as f32 * bits_per_pixel as f32 / 8.0).floor() as usize;
    
    for _ in 0..complete_bytes_used {
        pixmap.data.push(storage.next());
    }

    let remainder_bits = ((width as u16 * height as u16 * bits_per_pixel as u16) % 8) as u8;
    if !compact && remainder_bits > 0 {
        pixmap.push(storage.next());
    } else if remainder_bits > 0 {
        let byte = storage.incomplete_get(remainder_bits);
        pixmap.push(byte);
        storage.pointer += remainder_bits;
        if storage.pointer >= 8 {
            storage.index += 1;
            storage.pointer -= 8;
        }
    }
    
    #[cfg(feature = "log")]
    info!("Identified pixmap: {:?}", pixmap.data);
}