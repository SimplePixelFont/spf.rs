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
use crate::vec;

#[cfg(feature = "log")]
pub(crate) use log::*;

pub(crate) fn push_signature<T: TagWriter>(engine: &mut SerializeEngine<T>) {
    #[cfg(feature = "tagging")]
    let start = engine.bytes.byte_index();

    engine.bytes.push(127);
    engine.bytes.push(102);
    engine.bytes.push(115);
    engine.bytes.push(70);

    #[cfg(feature = "tagging")]
    engine.tags.tag_span(
        TagKind::Signature,
        Span::new(start, engine.bytes.byte_index()),
    );

    #[cfg(feature = "log")]
    info!("Signed font data");
}

pub(crate) fn push_version<T: TagWriter>(engine: &mut SerializeEngine<T>) {
    let version = engine.layout.version as u8;

    engine.bytes.push(version);
    #[cfg(feature = "tagging")]
    engine.tags.tag_byte(
        TagKind::Version {
            value: engine.layout.version,
        },
        engine.bytes.byte_index(),
    );

    #[cfg(feature = "log")]
    info!("Pushed version {}", engine.layout.version);
}

pub(crate) fn push_header<T: TagWriter>(engine: &mut SerializeEngine<T>) {
    let mut font_properties = 0b00000000;
    if engine.layout.compact {
        font_properties |= 0b00000001;
    }

    engine.bytes.push(font_properties);
    #[cfg(feature = "tagging")]
    engine.tags.tag_bitflag(
        TagKind::Header,
        vec![TagKind::CompactFlag {
            enabled: engine.layout.compact,
        }],
        engine.bytes.byte_index(),
    );

    #[cfg(feature = "log")]
    info!("Pushed header");
}
