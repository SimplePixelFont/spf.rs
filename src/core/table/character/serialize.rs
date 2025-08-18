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

use crate::core::{
    byte, Character, CharacterTable, DeserializeError, Layout, SerializeError, Table,
    TableIdentifier,
};
#[cfg(feature = "log")]
use log::*;

pub(crate) fn push_grapheme_cluster<'a>(
    buffer: &'a mut byte::ByteStorage,
    constant_cluster_codepoints: Option<u8>,
    string: &String,
) {
    info!("hmm {:?} with {:?}", constant_cluster_codepoints, string);
    let mut string_bit_string = String::new(); // part of log

    string.bytes().for_each(|byte| {
        buffer.push(byte);
        string_bit_string.push_str(&format!("{:08b} ", byte)); // part of log
    });

    if constant_cluster_codepoints.is_none() {
        buffer.push(0);
        string_bit_string.push_str(&format!("{:08b} ", 0)); // part of log
    }

    #[cfg(feature = "log")]
    info!(
        "Pushed grapheme cluster '{}' with the following bits: {}",
        string, string_bit_string
    );
}
