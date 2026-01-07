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

impl<'a, T: TagWriter> DeserializeEngine<'a, ByteReaderImpl<'a>, T> {
    #[cfg(feature = "tagging")]
    pub fn from_data_and_tags(data: &'a [u8], tags: T) -> Self {
        Self {
            bytes: byte::ByteReaderImpl::from(data),
            layout: Layout::default(),
            #[cfg(feature = "tagging")]
            tags,
            #[cfg(feature = "tagging")]
            tagging_data: TaggingData::default(),
            _phantom: PhantomData,
            _phantom2: &PhantomData,
        }
    }
    #[cfg(feature = "tagging")]
    pub fn tagging_engine(&mut self, tags: T) {
        self.tags = tags;
    }
}

impl<'a> DeserializeEngine<'a, ByteReaderImpl<'a>> {
    pub fn from_data(data: &'a [u8]) -> Self {
        Self {
            bytes: byte::ByteReaderImpl::from(data),
            layout: Layout::default(),
            #[cfg(feature = "tagging")]
            tags: TagWriterNoOp,
            #[cfg(feature = "tagging")]
            tagging_data: TaggingData::default(),
            _phantom: PhantomData,
            _phantom2: &PhantomData,
        }
    }
}

impl<'a, R: ByteReader> DeserializeEngine<'a, R> {
    pub fn from_reader(reader: R) -> Self {
        Self {
            bytes: reader,
            layout: Layout::default(),
            #[cfg(feature = "tagging")]
            tags: TagWriterNoOp,
            #[cfg(feature = "tagging")]
            tagging_data: TaggingData::default(),
            _phantom: PhantomData,
            _phantom2: &PhantomData,
        }
    }
}

pub(crate) fn next_signature<R: ByteReader, T: TagWriter>(
    engine: &mut DeserializeEngine<R, T>,
) -> Result<(), DeserializeError> {
    if engine.bytes.index() + 4 > engine.bytes.len() {
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

pub(crate) fn next_version<R: ByteReader, T: TagWriter>(
    engine: &mut DeserializeEngine<R, T>,
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

pub(crate) fn next_header<R: ByteReader, T: TagWriter>(
    engine: &mut DeserializeEngine<R, T>,
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
