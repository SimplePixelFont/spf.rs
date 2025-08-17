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

pub(crate) use crate::core::Character;

#[cfg(feature = "log")]
pub(crate) use log::*;

pub(crate) fn next_width(storage: &mut byte::ByteStorage, constant_width: Option<u8>) -> u8 {
    if constant_width.is_none() {
        character.custom_width = Some(storage.get());
    }

    let current_character_width;
    if !header.configuration_flags.constant_width {
        character.custom_width = Some(storage.get());
        current_character_width = character.custom_width.unwrap();
        storage.index += 1;

        #[cfg(feature = "log")]
        info!("Identified custom width: {:?}", current_character_width);
    } else {
        current_character_width = header.configuration_values.constant_width.unwrap();
    }

    current_character_width
}

pub(crate) fn next_height(
    storage: &mut byte::ByteStorage,
    header: &Header,
    character: &mut Character,
) -> u8 {
    let current_character_height;
    if !header.configuration_flags.constant_height {
        character.custom_height = Some(storage.get());
        current_character_height = character.custom_height.unwrap();
        storage.index += 1;

        #[cfg(feature = "log")]
        info!("Identified custom height: {:?}", current_character_height);
    } else {
        current_character_height = header.configuration_values.constant_height.unwrap();
    }

    current_character_height
}

pub(crate) fn next_pixmap(
    storage: &mut byte::ByteStorage,
    header: &Header,
    character: &mut Character,
    width: u8,
    height: u8,
    bits_per_pixel: u8,
) {
    let pixels_used = width * height;
    let mut current_bit = storage.pointer;
    for _ in 0..pixels_used {
        let pixel = storage.incomplete_get(bits_per_pixel);
        character.pixmap.push(pixel);
        current_bit += bits_per_pixel;
        if current_bit >= 8 {
            storage.index += 1;
            current_bit = 0;
        }
        storage.pointer = current_bit;
    }

    if !header.modifier_flags.compact && (width * height * bits_per_pixel) % 8 != 0 {
        storage.index += 1;
        storage.pointer = 0;
    }

    #[cfg(feature = "log")]
    info!("Identified pixmap: {:?}", character.pixmap);
}
