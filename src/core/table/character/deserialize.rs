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

use crate::core::{byte, Character};
use crate::String;

#[cfg(feature = "log")]
use log::*;

pub(crate) fn next_grapheme_cluster(
    storage: &mut byte::ByteStorage,
    character: &mut Character,
    constant_cluster_codepoints: Option<u8>,
) {
    let mut grapheme_cluster = String::new();
    let mut end_cluster = false;
    let mut codepoint_count = 0;

    while !end_cluster {
        let utf81 = storage.next();
        let mut utf8_bytes: [u8; 4] = [0, 0, 0, 0];

        if utf81 >> 7 == 0b00000000 {
            utf8_bytes[0] = utf81;
        } else if utf81 >> 5 == 0b00000110 {
            utf8_bytes[0] = utf81;
            utf8_bytes[1] = storage.next();
        } else if utf81 >> 4 == 0b00001110 {
            utf8_bytes[0] = utf81;
            utf8_bytes[1] = storage.next();
            utf8_bytes[2] = storage.next();
        } else if utf81 >> 3 == 0b00011110 {
            utf8_bytes[0] = utf81;
            utf8_bytes[1] = storage.next();
            utf8_bytes[2] = storage.next();
            utf8_bytes[3] = storage.next();
        }

        grapheme_cluster.push(
            String::from_utf8(utf8_bytes.to_vec())
                .unwrap()
                .chars()
                .next()
                .unwrap(),
        );
        codepoint_count += 1;

        if let Some(constant_cluster_codepoints) = constant_cluster_codepoints {
            if codepoint_count == constant_cluster_codepoints {
                end_cluster = true;
            }
        } else if storage.peek() == 0 {
            end_cluster = true;
            storage.index += 1;
        }
    }

    #[cfg(feature = "log")]
    info!("Identified grapheme cluster: {:?}", grapheme_cluster);

    character.grapheme_cluster = grapheme_cluster;
}
