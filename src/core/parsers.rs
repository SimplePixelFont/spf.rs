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

pub(crate) fn next_signature(storage: &mut byte::ByteStorage) {
    if storage.index + 3 > storage.bytes.len() {
        panic!("Unexpected end of file");
    }

    for byte in [102, 115, 70].iter() {
        if storage.get() != *byte {
            panic!("File is not signed");
        }
        storage.index += 1;
    }
}

pub(crate) fn next_header(layout: &mut Layout, storage: &mut byte::ByteStorage) {
    let file_properties = storage.get();

    let configuration_flag_booleans = [
        (file_properties & 0b10000000) >> 7 == 1,
        (file_properties & 0b01000000) >> 6 == 1,
        (file_properties & 0b00100000) >> 5 == 1,
        (file_properties & 0b00010000) >> 4 == 1,
    ];

    layout.header.modifier_flags.compact = (file_properties & 0b00001000) >> 3 == 1;
    storage.index += 1;

    let mut configuration_flag_values = [None; 4];

    for (current_configuration_flag_index, configuration_flag) in
        configuration_flag_booleans.iter().enumerate()
    {
        if *configuration_flag {
            configuration_flag_values[current_configuration_flag_index] = Some(storage.get());
            storage.index += 1;
        }
    }

    layout
        .header
        .configuration_flags
        .constant_cluster_codepoints = configuration_flag_booleans[0];
    layout.header.configuration_flags.constant_width = configuration_flag_booleans[1];
    layout.header.configuration_flags.constant_height = configuration_flag_booleans[2];
    layout.header.configuration_flags.custom_bits_per_pixel = configuration_flag_booleans[3];

    layout
        .header
        .configuration_values
        .constant_cluster_codepoints = configuration_flag_values[0];
    layout.header.configuration_values.constant_width = configuration_flag_values[1];
    layout.header.configuration_values.constant_height = configuration_flag_values[2];
    layout.header.configuration_values.custom_bits_per_pixel = configuration_flag_values[3];
}

pub(crate) fn next_grapheme_cluster(
    storage: &mut byte::ByteStorage,
    header: &Header,
    character: &mut Character,
) {
    let mut grapheme_cluster = String::new();
    let mut end_cluster = false;
    let mut codepoint_count = 0;

    while !end_cluster {
        let utf81 = storage.get();
        let mut utf8_bytes: [u8; 4] = [0, 0, 0, 0];

        if utf81 >> 7 == 0b00000000 {
            utf8_bytes[0] = utf81;
        } else if utf81 >> 5 == 0b00000110 {
            utf8_bytes[0] = utf81;
            storage.index += 1;
            utf8_bytes[1] = storage.get();
        } else if utf81 >> 4 == 0b00001110 {
            utf8_bytes[0] = utf81;
            storage.index += 1;
            utf8_bytes[1] = storage.get();
            storage.index += 1;
            utf8_bytes[2] = storage.get();
        } else if utf81 >> 3 == 0b00011110 {
            utf8_bytes[0] = utf81;
            storage.index += 1;
            utf8_bytes[1] = storage.get();
            storage.index += 1;
            utf8_bytes[2] = storage.get();
            storage.index += 1;
            utf8_bytes[3] = storage.get();
        }

        grapheme_cluster.push(
            String::from_utf8(utf8_bytes.to_vec())
                .unwrap()
                .chars()
                .next()
                .unwrap(),
        );
        codepoint_count += 1;

        if header.configuration_flags.constant_cluster_codepoints {
            if codepoint_count
                == header
                    .configuration_values
                    .constant_cluster_codepoints
                    .unwrap()
            {
                end_cluster = true;
            }
        } else {
            storage.index += 1;
            if storage.get() == 0 {
                end_cluster = true;
            }
        }
    }

    #[cfg(feature = "log")]
    info!("Identified grapheme cluster: {:?}", grapheme_cluster);

    character.grapheme_cluster = grapheme_cluster;
}
