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
//! [`crate::articles::c_usage`] article. Also note that the [`self::converters`] module is  not
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

use core::ffi::*;
use core::slice;

pub mod converters;
pub mod defaults;

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

    pub grapheme_cluster: *mut c_char,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SPFColorTable {
    pub has_constant_alpha: c_uchar,
    pub constant_alpha: c_uchar,

    pub colors: *mut SPFColor,
    pub colors_length: c_ulong,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SPFColor {
    pub has_custom_alpha: c_uchar,
    pub custom_alpha: c_uchar,
    pub r: c_uchar,
    pub g: c_uchar,
    pub b: c_uchar,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
/// Used to represent a [`Vec<u8>`] in the C ABI. This is simply a `u_char` array on the heap which can be reconstructed with the pointer `data` and length `data_length`.
pub struct SPFData {
    pub data: *mut c_uchar,
    pub data_length: c_ulong,
}

#[no_mangle]
/// Thin wrapper around [`layout_to_data`] compatible with the C ABI.
///
/// This function takes a [`SPFLayout`] struct and converts it into a Rust native [`Layout`] struct.
/// The [`Layout`] struct is then parsed into a [`Vec<u8>`] with the [`layout_to_data`] function.
/// The [`Vec<u8>`] is then converted into a [`SPFData`] struct and returned.
pub extern "C" fn spf_core_layout_to_data(layout: SPFLayout) -> SPFData {
    let mut data = layout_to_data(layout.try_into().unwrap())
        .unwrap()
        .into_boxed_slice();
    let data_length = data.len() as c_ulong;
    let data_ptr = data.as_mut_ptr();
    core::mem::forget(data);
    SPFData {
        data: data_ptr,
        data_length,
    }
}

#[no_mangle]
/// Thin wrapper around [`layout_from_data`] compatible with the C ABI.
///
/// This function takes a pointer to a [`c_uchar`] array with a length of [`c_ulong`] and creates a
/// [`Vec<u8>`] from the data. This data is then passed to the [`layout_from_data`] function to
/// create a [`Layout`] struct. The [`Layout`] struct is then converted into a [`SPFLayout`] struct
/// and returned.
pub unsafe extern "C" fn spf_core_layout_from_data(
    pointer: *const c_uchar,
    length: c_ulong,
) -> SPFLayout {
    let data = unsafe { slice::from_raw_parts(pointer, length as usize) };
    let layout = layout_from_data(data).unwrap();
    layout.try_into().unwrap()
}
