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
//! [`crate::articles::c_usage`] article. Also note that the [`crate::ffi::converters`] module is not
//! part of the `spf.rs` library and only exposed in the Rust crate.
//!
//! # Conventions
//!
//! Function names are prefixed with `spf_` followed by the module name they are in. For example, the
//! function [`crate::ffi::spf_core_layout_from_data`] is the C ABI compatible version of the
//! [`crate::core::layout_from_data`] function in the [`crate::core`] module.
//!
//! All structs are prefixed with `SPF` followed by the struct name. For example, the struct
//! [`crate::ffi::SPFLayout`] is the C ABI compatible version of the [`crate::core::Layout`] struct in
//! the [`crate::core`] module.
//!
//! All functions that return a [`Vec<u8>`] return a [`crate::ffi::SPFData`] struct instead.

use crate::core::*;
use core::slice;

#[cfg(feature = "std")]
pub(crate) use std::ffi::*;

#[cfg(not(feature = "std"))]
pub(crate) use alloc::ffi::*;

pub mod converters;
pub mod defaults;
pub mod free;

pub(crate) mod macros;

#[doc(inline)]
pub use converters::*;

#[doc(inline)]
pub use free::*;

#[doc = include_str!("../../res/snippets/pixmap_table/configurations/flag/use_constant_width.md")]
pub const SPF_PIXMAP_TABLE_CONFIGURATION_FLAGS_CONSTANT_WIDTH: u8 = 1 << 0;
#[doc = include_str!("../../res/snippets/pixmap_table/configurations/flag/use_constant_height.md")]
pub const SPF_PIXMAP_TABLE_CONFIGURATION_FLAGS_CONSTANT_HEIGHT: u8 = 1 << 1;
#[doc = include_str!("../../res/snippets/pixmap_table/configurations/flag/use_constant_bits_per_pixel.md")]
pub const SPF_PIXMAP_TABLE_CONFIGURATION_FLAGS_CONSTANT_BITS_PER_PIXEL: u8 = 1 << 2;
#[doc = include_str!("../../res/snippets/pixmap_table/links/flag/link_color_tables.md")]
pub const SPF_PIXMAP_TABLE_LINK_FLAGS_LINK_COLOR_TABLES: u8 = 1 << 0;

#[doc = include_str!("../../res/snippets/character_table/modifiers/brief/use_advance_x.md")]
pub const SPF_CHARACTER_TABLE_MODIFIER_FLAGS_USE_ADVANCE_X: u8 = 1 << 0;
#[doc = include_str!("../../res/snippets/character_table/modifiers/brief/use_pixmap_index.md")]
pub const SPF_CHARACTER_TABLE_MODIFIER_FLAGS_USE_PIXMAP_INDEX: u8 = 1 << 1;
#[doc = include_str!("../../res/snippets/character_table/modifiers/brief/use_pixmap_table_index.md")]
pub const SPF_CHARACTER_TABLE_MODIFIER_FLAGS_USE_PIXMAP_TABLE_INDEX: u8 = 1 << 2;

#[doc = include_str!("../../res/snippets/character_table/configurations/flag/use_constant_code_point_count.md")]
pub const SPF_CHARACTER_TABLE_CONFIGURATION_FLAGS_CONSTANT_CODE_POINT_COUNT: u8 = 1 << 0;

#[doc = include_str!("../../res/snippets/character_table/links/flag/link_pixmap_tables.md")]
pub const SPF_CHARACTER_TABLE_LINK_FLAGS_LINK_PIXMAP_TABLES: u8 = 1 << 0;

#[doc = include_str!("../../res/snippets/color_table/modifiers/brief/use_color_type.md")]
pub const SPF_COLOR_TABLE_MODIFIER_FLAGS_USE_COLOR_TYPE: u8 = 1 << 0;
#[doc = include_str!("../../res/snippets/color_table/configurations/flag/use_constant_alpha.md")]
pub const SPF_COLOR_TABLE_CONFIGURATION_FLAGS_CONSTANT_ALPHA: u8 = 1 << 0;

#[doc = include_str!("../../res/snippets/font_table/links/flag/link_character_tables.md")]
pub const SPF_FONT_TABLE_LINK_FLAGS_LINK_CHARACTER_TABLES: u8 = 1 << 0;

#[derive(Debug, Clone)]
#[repr(C)]
/// C ABI mirror of [`Layout`].
pub struct SPFLayout {
    /// See [`Layout::version`].
    pub version: c_uchar,

    /// See [`Layout::compact`].
    pub compact: c_uchar,

    /// Pointer to the first of `character_tables_length` [`SPFCharacterTable`]s. See [`Layout::character_tables`].
    pub character_tables: *mut SPFCharacterTable,
    /// Number of elements at `character_tables`.
    pub character_tables_length: c_ulong,
    /// Pointer to the first of `color_tables_length` [`SPFColorTable`]s. See [`Layout::color_tables`].
    pub color_tables: *mut SPFColorTable,
    /// Number of elements at `color_tables`.
    pub color_tables_length: c_ulong,
    /// Pointer to the first of `pixmap_tables_length` [`SPFPixmapTable`]s. See [`Layout::pixmap_tables`].
    pub pixmap_tables: *mut SPFPixmapTable,
    /// Number of elements at `pixmap_tables`.
    pub pixmap_tables_length: c_ulong,
    /// Pointer to the first of `font_tables_length` [`SPFFontTable`]s. See [`Layout::font_tables`].
    pub font_tables: *mut SPFFontTable,
    /// Number of elements at `font_tables`.
    pub font_tables_length: c_ulong,
}

#[derive(Debug, Clone)]
#[repr(C)]
/// C ABI mirror of [`PixmapTable`].
pub struct SPFPixmapTable {
    /// See [`PixmapTable::configuration_flags`].
    pub configuration_flags: c_uchar,
    #[doc = include_str!("../../res/snippets/data_types/has_field.md")]
    pub has_constant_width: c_uchar,
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/condition/constant_width.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/brief/constant_width.md")]
    pub constant_width: c_uchar,
    #[doc = include_str!("../../res/snippets/data_types/has_field.md")]
    pub has_constant_height: c_uchar,
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/condition/constant_height.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/brief/constant_height.md")]
    pub constant_height: c_uchar,
    #[doc = include_str!("../../res/snippets/data_types/has_field.md")]
    pub has_constant_bits_per_pixel: c_uchar,
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/condition/constant_bits_per_pixel.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/configurations/brief/constant_bits_per_pixel.md")]
    pub constant_bits_per_pixel: c_uchar,

    /// See [`PixmapTable::link_flags`].
    pub link_flags: c_uchar,
    #[doc = include_str!("../../res/snippets/data_types/has_field.md")]
    pub has_color_table_indexes: c_uchar,
    #[doc = include_str!("../../res/snippets/pixmap_table/links/condition/color_tables.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/links/brief/color_tables.md")]
    pub color_table_indexes: *mut c_uchar,
    /// Number of elements at `color_table_indexes`.
    pub color_table_indexes_length: c_ulong,

    /// Pointer to the first of `pixmaps_length` [`SPFPixmap`]s. See [`PixmapTable::pixmaps`].
    pub pixmaps: *mut SPFPixmap,
    /// Number of elements at `pixmaps`.
    pub pixmaps_length: c_ulong,
}

#[derive(Debug, Clone)]
#[repr(C)]
/// C ABI mirror of [`Pixmap`].
pub struct SPFPixmap {
    #[doc = include_str!("../../res/snippets/data_types/has_field.md")]
    pub has_custom_width: c_uchar,
    #[doc = include_str!("../../res/snippets/pixmap_table/records/condition/custom_width.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/records/brief/custom_width.md")]
    pub custom_width: c_uchar,
    #[doc = include_str!("../../res/snippets/data_types/has_field.md")]
    pub has_custom_height: c_uchar,
    #[doc = include_str!("../../res/snippets/pixmap_table/records/condition/custom_height.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/records/brief/custom_height.md")]
    pub custom_height: c_uchar,
    #[doc = include_str!("../../res/snippets/data_types/has_field.md")]
    pub has_custom_bits_per_pixel: c_uchar,
    #[doc = include_str!("../../res/snippets/pixmap_table/records/condition/custom_bits_per_pixel.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/records/brief/custom_bits_per_pixel.md")]
    pub custom_bits_per_pixel: c_uchar,
    #[doc = include_str!("../../res/snippets/pixmap_table/records/condition/data.md")]
    #[doc = include_str!("../../res/snippets/pixmap_table/records/brief/data.md")]
    pub data: *mut c_uchar,
    /// Number of elements at `data`.
    pub data_length: c_ulong,
}

#[derive(Debug, Clone)]
#[repr(C)]
/// C ABI mirror of [`CharacterTable`].
pub struct SPFCharacterTable {
    /// See [`CharacterTable::modifier_flags`].
    pub modifier_flags: c_uchar,

    /// See [`CharacterTable::configuration_flags`].
    pub configuration_flags: c_uchar,
    #[doc = include_str!("../../res/snippets/data_types/has_field.md")]
    pub has_constant_code_point_count: c_uchar,
    #[doc = include_str!("../../res/snippets/character_table/configurations/condition/constant_code_point_count.md")]
    #[doc = include_str!("../../res/snippets/character_table/configurations/brief/constant_code_point_count.md")]
    pub constant_code_point_count: c_uchar,

    /// See [`CharacterTable::link_flags`].
    pub link_flags: c_uchar,
    #[doc = include_str!("../../res/snippets/data_types/has_field.md")]
    pub has_pixmap_table_indexes: c_uchar,
    #[doc = include_str!("../../res/snippets/character_table/links/condition/pixmap_tables.md")]
    #[doc = include_str!("../../res/snippets/character_table/links/brief/pixmap_tables.md")]
    pub pixmap_table_indexes: *mut c_uchar,
    /// Number of elements at `pixmap_table_indexes`.
    pub pixmap_table_indexes_length: c_ulong,

    /// Pointer to the first of `characters_length` [`SPFCharacter`]s. See [`CharacterTable::characters`].
    pub characters: *mut SPFCharacter,
    /// Number of elements at `characters`.
    pub characters_length: c_ulong,
}

#[derive(Debug, Clone)]
#[repr(C)]
/// C ABI mirror of [`Character`].
pub struct SPFCharacter {
    #[doc = include_str!("../../res/snippets/data_types/has_field.md")]
    pub has_advance_x: c_uchar,
    #[doc = include_str!("../../res/snippets/character_table/records/condition/advance_x.md")]
    #[doc = include_str!("../../res/snippets/character_table/records/brief/advance_x.md")]
    pub advance_x: c_uchar,
    #[doc = include_str!("../../res/snippets/data_types/has_field.md")]
    pub has_pixmap_index: c_uchar,
    #[doc = include_str!("../../res/snippets/character_table/records/condition/pixmap_index.md")]
    #[doc = include_str!("../../res/snippets/character_table/records/brief/pixmap_index.md")]
    pub pixmap_index: c_uchar,
    #[doc = include_str!("../../res/snippets/data_types/has_field.md")]
    pub has_pixmap_table_index: c_uchar,
    #[doc = include_str!("../../res/snippets/character_table/records/condition/pixmap_table_index.md")]
    #[doc = include_str!("../../res/snippets/character_table/records/brief/pixmap_table_index.md")]
    pub pixmap_table_index: c_uchar,

    #[doc = include_str!("../../res/snippets/character_table/records/condition/code_points.md")]
    #[doc = include_str!("../../res/snippets/character_table/records/brief/code_points.md")]
    pub code_points: *mut c_char,
}

#[derive(Debug, Clone)]
#[repr(C)]
/// C ABI mirror of [`ColorTable`].
pub struct SPFColorTable {
    /// See [`ColorTable::modifier_flags`].
    pub modifier_flags: c_uchar,

    /// See [`ColorTable::configuration_flags`].
    pub configuration_flags: c_uchar,
    #[doc = include_str!("../../res/snippets/data_types/has_field.md")]
    pub has_constant_alpha: c_uchar,
    #[doc = include_str!("../../res/snippets/color_table/configurations/condition/constant_alpha.md")]
    #[doc = include_str!("../../res/snippets/color_table/configurations/brief/constant_alpha.md")]
    pub constant_alpha: c_uchar,

    /// Pointer to the first of `colors_length` [`SPFColor`]s. See [`ColorTable::colors`].
    pub colors: *mut SPFColor,
    /// Number of elements at `colors`.
    pub colors_length: c_ulong,
}

#[derive(Debug, Clone)]
#[repr(C)]
/// C ABI mirror of [`Color`].
pub struct SPFColor {
    #[doc = include_str!("../../res/snippets/data_types/has_field.md")]
    pub has_color_type: c_uchar,
    #[doc = include_str!("../../res/snippets/color_table/records/condition/color_type.md")]
    #[doc = include_str!("../../res/snippets/color_table/records/brief/color_type.md")]
    pub color_type: c_uchar,

    #[doc = include_str!("../../res/snippets/data_types/has_field.md")]
    pub has_custom_alpha: c_uchar,
    #[doc = include_str!("../../res/snippets/color_table/records/condition/custom_alpha.md")]
    #[doc = include_str!("../../res/snippets/color_table/records/brief/custom_alpha.md")]
    pub custom_alpha: c_uchar,

    #[doc = include_str!("../../res/snippets/color_table/records/condition/red.md")]
    #[doc = include_str!("../../res/snippets/color_table/records/brief/red.md")]
    pub red: c_uchar,
    #[doc = include_str!("../../res/snippets/color_table/records/condition/green.md")]
    #[doc = include_str!("../../res/snippets/color_table/records/brief/green.md")]
    pub green: c_uchar,
    #[doc = include_str!("../../res/snippets/color_table/records/condition/blue.md")]
    #[doc = include_str!("../../res/snippets/color_table/records/brief/blue.md")]
    pub blue: c_uchar,
}

#[derive(Debug, Clone)]
#[repr(C)]
/// C ABI mirror of [`FontTable`].
pub struct SPFFontTable {
    /// See [`FontTable::link_flags`].
    pub link_flags: c_uchar,
    #[doc = include_str!("../../res/snippets/data_types/has_field.md")]
    pub has_character_table_indexes: c_uchar,
    #[doc = include_str!("../../res/snippets/font_table/links/condition/character_tables.md")]
    #[doc = include_str!("../../res/snippets/font_table/links/brief/character_tables.md")]
    pub character_table_indexes: *mut c_uchar,
    /// Number of elements at `character_table_indexes`.
    pub character_table_indexes_length: c_ulong,

    /// Pointer to the first of `fonts_length` [`SPFFont`]s. See [`FontTable::fonts`].
    pub fonts: *mut SPFFont,
    /// Number of elements at `fonts`.
    pub fonts_length: c_ulong,
}

#[derive(Debug, Clone)]
#[repr(C)]
/// C ABI mirror of [`Font`].
pub struct SPFFont {
    #[doc = include_str!("../../res/snippets/font_table/records/condition/name.md")]
    #[doc = include_str!("../../res/snippets/font_table/records/brief/name.md")]
    pub name: *mut c_char,
    #[doc = include_str!("../../res/snippets/font_table/records/condition/author.md")]
    #[doc = include_str!("../../res/snippets/font_table/records/brief/author.md")]
    pub author: *mut c_char,
    #[doc = include_str!("../../res/snippets/font_table/records/condition/version.md")]
    #[doc = include_str!("../../res/snippets/font_table/records/brief/version.md")]
    pub version: c_uchar,
    #[doc = include_str!("../../res/snippets/font_table/records/condition/font_type.md")]
    #[doc = include_str!("../../res/snippets/font_table/records/brief/font_type.md")]
    pub font_type: c_uchar,
    #[doc = include_str!("../../res/snippets/font_table/records/condition/linked_character_table_indexes.md")]
    #[doc = include_str!("../../res/snippets/font_table/records/brief/linked_character_table_indexes.md")]
    pub linked_character_table_indexes: *mut c_uchar,
    /// Number of elements at `linked_character_table_indexes`.
    pub linked_character_table_indexes_length: c_ulong,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
/// Used to represent a [`Vec<u8>`] in the C ABI. This is simply a `u_char` array on the heap
/// which can be reconstructed with the pointer `data` and length `data_length`.
/// The caller is responsible for freeing this with [`free::spf_free_data`].
pub struct SPFData {
    /// Pointer to the first of `data_length` bytes.
    pub data: *mut c_uchar,
    /// Number of bytes at `data`.
    pub data_length: c_ulong,
}

/// Status codes returned by all exported FFI functions. `SPFStatus::Ok` (0) indicates success;
/// all other values indicate a specific failure. The C caller should check this before reading
/// any out-parameter.
#[repr(C)]
pub enum SPFStatus {
    #[doc = include_str!("../../res/snippets/errors/ok.md")]
    Ok = 0,
    #[doc = include_str!("../../res/snippets/errors/unexpected_end_of_file.md")]
    ErrUnexpectedEndOfFile = 1,
    #[doc = include_str!("../../res/snippets/errors/invalid_signature.md")]
    ErrInvalidSignature = 2,
    #[doc = include_str!("../../res/snippets/errors/unsupported_version.md")]
    ErrUnsupportedVersion = 3,
    #[doc = include_str!("../../res/snippets/errors/unsupported_color_type.md")]
    ErrUnsupportedColorType = 4,
    #[doc = include_str!("../../res/snippets/errors/unsupported_table_identifier.md")]
    ErrUnsupportedTableIdentifier = 5,
    #[doc = include_str!("../../res/snippets/errors/unsupported_font_type.md")]
    ErrUnsupportedFontType = 6,
    #[doc = include_str!("../../res/snippets/errors/static_vector_too_large.md")]
    ErrStaticVectorTooLarge = 10,
    #[doc = include_str!("../../res/snippets/errors/invalid_pixmap_data.md")]
    ErrInvalidPixmapData = 11,
    #[doc = include_str!("../../res/snippets/errors/conversion_null_error.md")]
    ErrConversionNulError = 20,
    #[doc = include_str!("../../res/snippets/errors/conversion_utf8_error.md")]
    ErrConversionUtf8Error = 21,
}

impl From<DeserializeError> for SPFStatus {
    fn from(err: DeserializeError) -> Self {
        match err {
            DeserializeError::UnexpectedEndOfFile => SPFStatus::ErrUnexpectedEndOfFile,
            DeserializeError::InvalidSignature => SPFStatus::ErrInvalidSignature,
            DeserializeError::UnsupportedVersion => SPFStatus::ErrUnsupportedVersion,
            DeserializeError::UnsupportedColorType => SPFStatus::ErrUnsupportedColorType,
            DeserializeError::UnsupportedTableIdentifier => {
                SPFStatus::ErrUnsupportedTableIdentifier
            }
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
    #[doc = include_str!("../../res/snippets/data_types/Version/FV0.md")]
    FV0 = 0,
}

/// Named constants for the `color_type` field of [`SPFColor`].
#[repr(C)]
pub enum SPFColorType {
    #[doc = include_str!("../../res/snippets/data_types/ColorType/Dynamic.md")]
    Dynamic = 0,
    #[doc = include_str!("../../res/snippets/data_types/ColorType/Absolute.md")]
    Absolute = 1,
}

/// Named constants for the `font_type` field of [`SPFFont`].
#[repr(C)]
pub enum SPFFontType {
    #[doc = include_str!("../../res/snippets/data_types/FontType/Regular.md")]
    Regular = 0,
    #[doc = include_str!("../../res/snippets/data_types/FontType/Bold.md")]
    Bold = 1,
    #[doc = include_str!("../../res/snippets/data_types/FontType/Italic.md")]
    Italic = 2,
}

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
        *out = SPFData {
            data: data_ptr,
            data_length,
        };
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
