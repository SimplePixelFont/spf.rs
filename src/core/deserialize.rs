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

pub(crate) fn next_signature<T: TagWriter>(
    engine: &mut DeserializeEngine<T>,
) -> Result<(), DeserializeError> {
    if engine.bytes.index + 4 > engine.bytes.len() {
        return Err(DeserializeError::UnexpectedEndOfFile);
    }
    #[cfg(feature = "tagging")]
    let start = engine.bytes.byte_index();

    for byte in [127, 102, 115, 70].iter() {
        if engine.bytes.next() != *byte {
            return Err(DeserializeError::InvalidSignature);
        }
    }

    #[cfg(feature = "tagging")]
    engine.tags.tag_span(
        TagKind::Signature,
        Span::new(start, engine.bytes.byte_index()),
    );

    Ok(())
}

pub(crate) fn next_version<T: TagWriter>(
    engine: &mut DeserializeEngine<T>,
) -> Result<(), DeserializeError> {
    let version = engine.bytes.next();
    let version = Version::try_from(version)?;
    #[cfg(feature = "tagging")]
    engine.tags.tag_byte(
        TagKind::Version { value: version },
        engine.bytes.byte_index(),
    );

    engine.layout.version = version;
    Ok(())
}

pub(crate) fn next_header<T: TagWriter>(
    engine: &mut DeserializeEngine<T>,
) -> Result<(), DeserializeError> {
    let file_properties = engine.bytes.next();

    engine.layout.compact = byte::get_bit(file_properties, 0);

    #[cfg(feature = "tagging")]
    engine.tags.tag_bitflag(
        TagKind::Header,
        vec![TagKind::CompactFlag {
            enabled: engine.layout.compact,
        }],
        engine.bytes.byte_index(),
    );

    Ok(())
}
