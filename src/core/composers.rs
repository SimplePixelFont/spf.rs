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
    buffer.push(127);
    buffer.push(102);
    buffer.push(115);
    buffer.push(70);

    #[cfg(feature = "log")]
    info!("Signed font data.");

    buffer
}

pub(crate) fn push_version(buffer: &mut byte::ByteStorage, version: &Version) {
    buffer.push(match version {
        Version::FV0 => 0,
    });
}

pub(crate) fn push_header<'a>(buffer: &mut byte::ByteStorage, layout: &Layout) {
    let mut font_properties = 0b00000000;
    if layout.compact {
        font_properties |= 0b00000001;
    }
    buffer.push(font_properties);
}
