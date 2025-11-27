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

#[cfg(feature = "log")]
pub(crate) use log::*;

pub(crate) fn push_signature(engine: &mut SerializeEngine) {
    engine.bytes.push(127);
    engine.bytes.push(102);
    engine.bytes.push(115);
    engine.bytes.push(70);

    #[cfg(feature = "log")]
    info!("Signed font data.");
}

pub(crate) fn push_version(engine: &mut SerializeEngine) {
    engine.bytes.push(match engine.layout.version {
        Version::FV0 => 0,
    });
}

pub(crate) fn push_header(engine: &mut SerializeEngine) {
    let mut font_properties = 0b00000000;
    if engine.layout.compact {
        font_properties |= 0b00000001;
    }
    engine.bytes.push(font_properties);
}
