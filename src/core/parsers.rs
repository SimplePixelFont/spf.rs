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

pub(crate) fn next_grapheme_cluster(
    body_buffer: &mut byte::ByteStorage,
    header: &Header,
    mut current_index: usize,
) -> (String, usize) {
    let mut grapheme_cluster = String::new();
    let mut end_cluster = false;
    let mut codepoint_count = 0;

    while !end_cluster {
        let utf81 = body_buffer.get(current_index);
        let mut utf8_bytes: [u8; 4] = [0, 0, 0, 0];

        if utf81.bits[7..] == [false] {
            utf8_bytes[0] = utf81.to_u8();
        } else if utf81.bits[5..] == [false, true, true, true] {
            utf8_bytes[0] = utf81.to_u8();
            current_index += 1;
            utf8_bytes[1] = body_buffer.get(current_index).to_u8();
        } else if utf81.bits[4..] == [false, true, true, true] {
            utf8_bytes[0] = utf81.to_u8();
            current_index += 1;
            utf8_bytes[1] = body_buffer.get(current_index).to_u8();
            current_index += 1;
            utf8_bytes[2] = body_buffer.get(current_index).to_u8();
        } else if utf81.bits[3..] == [false, true, true, true, true] {
            utf8_bytes[0] = utf81.to_u8();
            current_index += 1;
            utf8_bytes[1] = body_buffer.get(current_index).to_u8();
            current_index += 1;
            utf8_bytes[2] = body_buffer.get(current_index).to_u8();
            current_index += 1;
            utf8_bytes[3] = body_buffer.get(current_index).to_u8();
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
            current_index += 1;
            if body_buffer.get(current_index).to_u8() == 0 {
                end_cluster = true;
            }
        }
    }

    (grapheme_cluster, current_index)
}
