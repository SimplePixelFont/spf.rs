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

pub(crate) use super::*;
use crate::{format, String, Vec};

#[cfg(feature = "log")]
pub(crate) use log::*;

pub(crate) fn push_signature(buffer: &mut byte::ByteStorage) -> &mut byte::ByteStorage {
    buffer.push(102);
    buffer.push(115);
    buffer.push(70);

    #[cfg(feature = "log")]
    info!("Signed font data.");

    buffer
}

pub(crate) fn push_header<'a>(
    buffer: &'a mut byte::ByteStorage,
    header: &Header,
) -> &'a mut byte::ByteStorage {
    let mut font_properties = 0b00000000;
    let font_properties_index = buffer.bytes.len();
    buffer.push(font_properties);

    if header.configuration_flags.constant_cluster_codepoints {
        font_properties |= 0b10000000;
        buffer.push(
            header
                .configuration_values
                .constant_cluster_codepoints
                .unwrap(),
        );
    }

    if header.configuration_flags.constant_width {
        font_properties |= 0b01000000;
        buffer.push(header.configuration_values.constant_width.unwrap());
    }

    if header.configuration_flags.constant_height {
        font_properties |= 0b00100000;
        buffer.push(header.configuration_values.constant_height.unwrap());
    }

    if header.configuration_flags.custom_bits_per_pixel {
        font_properties |= 0b00010000;
        buffer.push(header.configuration_values.custom_bits_per_pixel.unwrap());
    }

    if header.modifier_flags.compact {
        font_properties |= 0b00001000;
    }

    buffer.bytes[font_properties_index] = font_properties;
    buffer
}

pub(crate) fn push_grapheme_cluster<'a>(
    buffer: &'a mut byte::ByteStorage,
    header: &Header,
    string: &String,
) -> &'a mut byte::ByteStorage {
    let mut string_bit_string = String::new(); // part of log

    string.bytes().for_each(|byte| {
        buffer.push(byte);
        string_bit_string.push_str(&format!("{:08b} ", byte)); // part of log
    });

    if !header.configuration_flags.constant_cluster_codepoints {
        buffer.push(0);
        string_bit_string.push_str(&format!("{:08b} ", 0)); // part of log
    }

    #[cfg(feature = "log")]
    info!(
        "Pushed grapheme cluster '{}' with the following bits: {}",
        string, string_bit_string
    );

    buffer
}

pub(crate) fn push_width<'a>(
    buffer: &'a mut byte::ByteStorage,
    header: &Header,
    custom_width: Option<u8>,
) -> &'a mut byte::ByteStorage {
    if !header.configuration_flags.constant_width {
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

    buffer
}

pub(crate) fn push_height<'a>(
    buffer: &'a mut byte::ByteStorage,
    header: &Header,
    custom_height: Option<u8>,
) -> &'a mut byte::ByteStorage {
    if !header.configuration_flags.constant_height {
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

    buffer
}

pub(crate) fn push_pixmap(buffer: &mut byte::ByteStorage, header: &Header, pixmap: &Vec<u8>) {
    let mut pixmap_bit_string = String::new();
    let mut bits_per_pixel = 1;
    let mut bits_used = 0;

    if header.configuration_flags.custom_bits_per_pixel {
        bits_per_pixel = header.configuration_values.custom_bits_per_pixel.unwrap();
    }

    for pixel in pixmap {
        pixmap_bit_string.push_str(&format!(
            "{:0bits_per_pixel$b} ",
            pixel,
            bits_per_pixel = bits_per_pixel as usize
        ));
        buffer.incomplete_push(*pixel, bits_per_pixel);
        bits_used += bits_per_pixel;
    }

    if !header.modifier_flags.compact {
        buffer.incomplete_push(0, 8 - (bits_used % 8));
    }

    #[cfg(feature = "log")]
    info!(
        "Pushed pixmap with the following bits: {}",
        pixmap_bit_string
    );
}
