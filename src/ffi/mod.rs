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

#![allow(clippy::missing_safety_doc)] // FFI will always be unsafe, no reason to document :)
#![allow(non_snake_case)]
//! A C compatible FFI layer for `spf.rs`.
//!
//! This module provides a thin wrapper around all the modules in `spf.rs` that allows it to be used
//! in a C compatible way exposed through a FFI. This allows `spf.rs` to be used as a library in C and
//! in any language that supports the platform-specific C-ABI through dynamic library loading, including
//! WebAssembly.
//!
//! To learn about how to use the `spf.rs` library in your language of choice, please refer to the
//! [`crate::articles::c_usage`] article. Also note that the [`self::converters`] module is not
//! part of the `spf.rs` library and only exposed in the Rust crate.
//!
//! # Conventions
//!
//! Function names are prefixed with `spf_` followed by the module name they are in. For example, the
//! function [`spf_core_layout_from_data`] is the C ABI compatible version of the [`layout_from_data`]
//! function in the [`crate::core`] module.
//!
//! All structs are prefixed with `SPF` followed by the struct name. For example, the struct
//! [`SPFLayout`] is the C ABI compatible version of the [`Layout`] struct in the [`crate::core`] module.
//!
//! All functions that return a [`Vec<u8>`] return a [`SPFData`] struct instead.

use crate::core::*;
use core::slice;

#[cfg(feature = "std")]
pub(crate) use std::ffi::*;

#[cfg(not(feature = "std"))]
pub(crate) use alloc::ffi::*;

pub mod converters;
pub mod defaults;
pub mod free;

#[macro_use]
pub(crate) mod macros;

#[doc(inline)]
pub use converters::*;

#[doc(inline)]
pub use free::*;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SPFLayout {
    pub version: c_uchar,

    pub compact: c_uchar,

    pub character_tables: *mut SPFCharacterTable,
    pub character_tables_length: c_ulong,
    pub color_tables: *mut SPFColorTable,
    pub color_tables_length: c_ulong,
    pub pixmap_tables: *mut SPFPixmapTable,
    pub pixmap_tables_length: c_ulong,
    pub font_tables: *mut SPFFontTable,
    pub font_tables_length: c_ulong,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SPFPixmapTable {
    pub has_constant_width: c_uchar,
    pub constant_width: c_uchar,
    pub has_constant_height: c_uchar,
    pub constant_height: c_uchar,
    pub has_constant_bits_per_pixel: c_uchar,
    pub constant_bits_per_pixel: c_uchar,

    pub has_color_table_indexes: c_uchar,
    pub color_table_indexes: *mut c_uchar,
    pub color_table_indexes_length: c_ulong,

    pub pixmaps: *mut SPFPixmap,
    pub pixmaps_length: c_ulong,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SPFPixmap {
    pub has_custom_width: c_uchar,
    pub custom_width: c_uchar,
    pub has_custom_height: c_uchar,
    pub custom_height: c_uchar,
    pub has_custom_bits_per_pixel: c_uchar,
    pub custom_bits_per_pixel: c_uchar,
    pub data: *mut c_uchar,
    pub data_length: c_ulong,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SPFCharacterTable {
    pub use_advance_x: c_uchar,
    pub use_pixmap_index: c_uchar,
    pub use_pixmap_table_index: c_uchar,

    pub has_constant_cluster_codepoints: c_uchar,
    pub constant_cluster_codepoints: c_uchar,

    pub has_pixmap_table_indexes: c_uchar,
    pub pixmap_table_indexes: *mut c_uchar,
    pub pixmap_table_indexes_length: c_ulong,

    pub characters: *mut SPFCharacter,
    pub characters_length: c_ulong,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SPFCharacter {
    pub has_advance_x: c_uchar,
    pub advance_x: c_uchar,
    pub has_pixmap_index: c_uchar,
    pub pixmap_index: c_uchar,
    pub has_pixmap_table_index: c_uchar,
    pub pixmap_table_index: c_uchar,

    pub grapheme_cluster: *mut c_char,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SPFColorTable {
    pub use_color_type: c_uchar,

    pub has_constant_alpha: c_uchar,
    pub constant_alpha: c_uchar,

    pub colors: *mut SPFColor,
    pub colors_length: c_ulong,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SPFColor {
    pub has_color_type: c_uchar,
    pub color_type: c_uchar,

    pub has_custom_alpha: c_uchar,
    pub custom_alpha: c_uchar,

    pub r: c_uchar,
    pub g: c_uchar,
    pub b: c_uchar,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SPFFontTable {
    pub has_character_table_indexes: c_uchar,
    pub character_table_indexes: *mut c_uchar,
    pub character_table_indexes_length: c_ulong,

    pub fonts: *mut SPFFont,
    pub fonts_length: c_ulong,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SPFFont {
    pub name: *mut c_char,
    pub author: *mut c_char,
    pub version: c_uchar,
    pub font_type: c_uchar,
    pub character_table_indexes: *mut c_uchar,
    pub character_tables_indexes_length: c_ulong,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
/// Used to represent a [`Vec<u8>`] in the C ABI. This is simply a `u_char` array on the heap
/// which can be reconstructed with the pointer `data` and length `data_length`.
/// The caller is responsible for freeing this with [`free::spf_free_data`].
pub struct SPFData {
    pub data: *mut c_uchar,
    pub data_length: c_ulong,
}

/// Status codes returned by all exported FFI functions. `SPFStatus::Ok` (0) indicates success;
/// all other values indicate a specific failure. The C caller should check this before reading
/// any out-parameter.
#[repr(C)]
pub enum SPFStatus {
    Ok = 0,
    ErrUnexpectedEndOfFile = 1,
    ErrInvalidSignature = 2,
    ErrUnsupportedVersion = 3,
    ErrUnsupportedColorType = 4,
    ErrUnsupportedTableIdentifier = 5,
    ErrUnsupportedFontType = 6,
    ErrStaticVectorTooLarge = 10,
    ErrInvalidPixmapData = 11,
    ErrConversionNulError = 20,
    ErrConversionUtf8Error = 21,
}

impl From<DeserializeError> for SPFStatus {
    fn from(err: DeserializeError) -> Self {
        match err {
            DeserializeError::UnexpectedEndOfFile => SPFStatus::ErrUnexpectedEndOfFile,
            DeserializeError::InvalidSignature => SPFStatus::ErrInvalidSignature,
            DeserializeError::UnsupportedVersion => SPFStatus::ErrUnsupportedVersion,
            DeserializeError::UnsupportedColorType => SPFStatus::ErrUnsupportedColorType,
            DeserializeError::UnsupportedTableIdentifier => SPFStatus::ErrUnsupportedTableIdentifier,
            DeserializeError::UnsupportedFontType => SPFStatus::ErrUnsupportedFontType,
        }
    }
}

impl From<SerializeError> for SPFStatus {
    fn from(err: SerializeError) -> Self {
        match err {
            SerializeError::StaticVectorTooLarge => SPFStatus::ErrStaticVectorTooLarge,
            SerializeError::InvalidPixmapData => SPFStatus::ErrInvalidPixmapData,
        }
    }
}

impl From<converters::ConversionError> for SPFStatus {
    fn from(err: converters::ConversionError) -> Self {
        match err {
            converters::ConversionError::NulError(_) => SPFStatus::ErrConversionNulError,
            converters::ConversionError::Utf8Error(_) => SPFStatus::ErrConversionUtf8Error,
            converters::ConversionError::UnsupportedVersion => SPFStatus::ErrUnsupportedVersion,
            converters::ConversionError::UnsupportedColorType => SPFStatus::ErrUnsupportedColorType,
            converters::ConversionError::UnsupportedFontType => SPFStatus::ErrUnsupportedFontType,
        }
    }
}

/// Named constants for the `version` field of [`SPFLayout`].
#[repr(C)]
pub enum SPFVersion {
    FV0 = 0,
}

/// Named constants for the `color_type` field of [`SPFColor`].
#[repr(C)]
pub enum SPFColorType {
    Dynamic = 0,
    Absolute = 1,
}

/// Named constants for the `font_type` field of [`SPFFont`].
#[repr(C)]
pub enum SPFFontType {
    Regular = 0,
    Bold = 1,
    Italic = 2,
}

// ── Exported functions ────────────────────────────────────────────────────────

#[no_mangle]
/// Thin wrapper around [`layout_to_data`] compatible with the C ABI.
///
/// Reads the [`SPFLayout`] at `layout`, converts it to a Rust-native [`Layout`],
/// serializes it, and writes the result into `out` as an [`SPFData`].
/// Returns [`SPFStatus::Ok`] on success.
///
/// The input `layout` is not consumed and remains valid after the call. On failure
/// the out-parameter is not written and the returned status describes the error.
/// On success the caller is responsible for freeing `out` with [`free::spf_free_data`].
pub unsafe extern "C" fn spf_core_layout_to_data(
    layout: *const SPFLayout,
    out: *mut SPFData,
) -> SPFStatus {
    let rust_layout: Layout = match unsafe { (*layout).clone().try_into() } {
        Ok(l) => l,
        Err(e) => return SPFStatus::from(e),
    };
    let data = match layout_to_data(&rust_layout) {
        Ok(d) => d,
        Err(e) => return SPFStatus::from(e),
    };
    let mut boxed = data.into_boxed_slice();
    let data_length = boxed.len() as c_ulong;
    let data_ptr = boxed.as_mut_ptr();
    core::mem::forget(boxed);
    unsafe {
        *out = SPFData { data: data_ptr, data_length };
    }
    SPFStatus::Ok
}

#[no_mangle]
/// Thin wrapper around [`layout_from_data`] compatible with the C ABI.
///
/// Reads `length` bytes from `pointer`, deserializes a font layout, and writes the result into
/// `out` as an [`SPFLayout`]. Returns [`SPFStatus::Ok`] on success.
///
/// On failure the out-parameter is not written and the returned status describes the error.
/// On success the caller is responsible for freeing `out` with [`free::spf_free_layout`].
pub unsafe extern "C" fn spf_core_layout_from_data(
    pointer: *const c_uchar,
    length: c_ulong,
    out: *mut SPFLayout,
) -> SPFStatus {
    let data = unsafe { slice::from_raw_parts(pointer, length as usize) };
    let layout = match layout_from_data(data) {
        Ok(l) => l,
        Err(e) => return SPFStatus::from(e),
    };
    let spf_layout = match layout.try_into() {
        Ok(l) => l,
        Err(e) => return SPFStatus::from(e),
    };
    unsafe {
        *out = spf_layout;
    }
    SPFStatus::Ok
}