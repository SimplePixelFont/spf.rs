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

#[cfg(feature = "std")]
pub(crate) use std::ffi::*;

#[cfg(not(feature = "std"))]
pub(crate) use alloc::ffi::*;

#[cfg(feature = "std")]
pub(crate) use std::str::Utf8Error;

#[cfg(not(feature = "std"))]
pub(crate) use core::str::Utf8Error;

pub(crate) use super::*;
pub(crate) use crate::{
    ffi_to_option, option_vec_to_raw, vec_from_raw_with_conversion, vec_to_raw,
    vec_to_raw_with_conversion, ToOwned, Vec,
};

pub(crate) mod character;
pub(crate) mod character_table;
pub(crate) mod color;
pub(crate) mod color_table;
pub(crate) mod font;
pub(crate) mod font_table;
pub(crate) mod pixmap;
pub(crate) mod pixmap_table;

#[derive(Debug, Clone)]
pub enum ConversionError {
    NulError(NulError),
    Utf8Error(Utf8Error),
}

impl From<NulError> for ConversionError {
    fn from(err: NulError) -> Self {
        ConversionError::NulError(err)
    }
}

impl From<Utf8Error> for ConversionError {
    fn from(err: Utf8Error) -> Self {
        ConversionError::Utf8Error(err)
    }
}

impl TryFrom<Layout> for SPFLayout {
    type Error = ConversionError;

    fn try_from(layout: Layout) -> Result<Self, Self::Error> {
        let (character_tables_ptr, character_tables_len) =
            vec_to_raw_with_conversion!(layout.character_tables, SPFCharacterTable);
        let (color_tables_ptr, color_tables_len) =
            vec_to_raw_with_conversion!(layout.color_tables, SPFColorTable);
        let (pixmap_tables_ptr, pixmap_tables_len) =
            vec_to_raw_with_conversion!(layout.pixmap_tables, SPFPixmapTable);
        let (font_tables_ptr, font_tables_len) =
            vec_to_raw_with_conversion!(layout.font_tables, SPFFontTable);

        Ok(SPFLayout {
            version: layout.version as c_uchar,
            compact: layout.compact as c_uchar,
            character_tables: character_tables_ptr,
            character_tables_length: character_tables_len as c_ulong,
            color_tables: color_tables_ptr,
            color_tables_length: color_tables_len as c_ulong,
            pixmap_tables: pixmap_tables_ptr,
            pixmap_tables_length: pixmap_tables_len as c_ulong,
            font_tables: font_tables_ptr,
            font_tables_length: font_tables_len as c_ulong,
        })
    }
}

impl TryInto<Layout> for SPFLayout {
    type Error = ConversionError;

    fn try_into(self) -> Result<Layout, Self::Error> {
        unsafe {
            let character_tables =
                vec_from_raw_with_conversion!(self.character_tables, self.character_tables_length);
            let color_tables =
                vec_from_raw_with_conversion!(self.color_tables, self.color_tables_length);
            let pixmap_tables =
                vec_from_raw_with_conversion!(self.pixmap_tables, self.pixmap_tables_length);
            let font_tables =
                vec_from_raw_with_conversion!(self.font_tables, self.font_tables_length);

            Ok(Layout {
                version: Version::try_from(self.version).unwrap(),
                compact: self.compact != 0,
                character_tables,
                color_tables,
                pixmap_tables,
                font_tables,
            })
        }
    }
}
