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

//! Rust-only module to abstract, and make writing `spf.rs` code easier.

pub(crate) use crate::core::*;

// remove ToString trait
use crate::Vec;

/// Magic bytes of `SimplePixelFont` files
///
/// The magic bytes can be used to determine if a file is a `SimplePixelFont` regardless of
/// the file extension. That being said the magic bytes as u8 are are follows: `102, 115, 70`.
/// In utf8 encoding this spells out `fsF` with a .
pub const MAGIC_BYTES: [u8; 4] = [127, 102, 115, 70];

#[derive(Default, Debug, Clone)]
#[non_exhaustive]
/// [`LayoutBuilder`] lets you create [`Layout`]'s without all the nested structs.
pub struct LayoutBuilder {
    pub version: Version,
    pub compact: bool,
}

impl LayoutBuilder {
    /// Sets the [`Layout::compact`] field of the builder.
    pub fn compact(&mut self, compact: bool) -> &mut Self {
        self.compact = compact;
        self
    }

    /// Sets the [`Layout::version`] field of the builder.
    pub fn version(&mut self, version: Version) -> &mut Self {
        self.version = version;
        self
    }

    pub fn table(&mut self, table: Box<impl Table>) -> &mut Self {
        self
    }
}
