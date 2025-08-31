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

pub(crate) fn next_signature(storage: &mut byte::ByteStorage) -> Result<(), DeserializeError> {
    if storage.index + 4 > storage.bytes.len() {
        return Err(DeserializeError::UnexpectedEndOfFile);
    }
    for byte in [127, 102, 115, 70].iter() {
        if storage.next() != *byte {
            return Err(DeserializeError::InvalidSignature);
        }
    }
    Ok(())
}

pub(crate) fn next_version(
    layout: &mut Layout,
    storage: &mut byte::ByteStorage,
) -> Result<(), DeserializeError> {
    let version = storage.next();
    let version = Version::try_from(version)?;
    layout.version = version;
    Ok(())
}

pub(crate) fn next_header(
    layout: &mut Layout,
    storage: &mut byte::ByteStorage,
) -> Result<(), DeserializeError> {
    let file_properties = storage.next();

    layout.compact = byte::get_bit(file_properties, 0);
    Ok(())
}
