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

use crate::cache::*;
use crate::core::*;
use crate::printer::*;

use crate::{ToOwned, ToString, Vec};

use core::ffi::*;
use core::slice;

pub mod converters;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SPFLayout {
    pub header: SPFHeader,
    pub body: SPFBody,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SPFHeader {
    pub configuration_flags: SPFConfigurationFlags,
    pub modifier_flags: SPFModifierFlags,
    pub configuration_values: SPFConfigurationValues,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SPFConfigurationFlags {
    pub constant_cluster_codepoints: c_uchar,
    pub constant_width: c_uchar,
    pub constant_height: c_uchar,
    pub custom_bits_per_pixel: c_uchar,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SPFModifierFlags {
    pub compact: c_uchar,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SPFConfigurationValues {
    pub constant_cluster_codepoints: c_uchar,
    pub constant_width: c_uchar,
    pub constant_height: c_uchar,
    pub custom_bits_per_pixel: c_uchar,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SPFCharacter {
    pub grapheme_cluster: *const c_char,
    pub custom_width: c_uchar,
    pub custom_height: c_uchar,
    pub pixmap: *mut c_uchar,
    pub pixmap_length: c_ulong,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SPFBody {
    pub characters: *mut SPFCharacter,
    pub characters_length: c_ulong,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
/// Used to represent a [`Vec<u8>`] in the C ABI. This is simply a `u_char` array on the heap which can be reconstructed with the pointer `data` and length `data_length`.
pub struct SPFData {
    pub data: *mut c_uchar,
    pub data_length: c_ulong,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SPFCharacterCache {
    pub mappings_keys: *mut *const c_char,
    pub mappings_values: *mut c_ulong,
    pub mappings_length: c_ulong,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SPFPrinter {
    pub font: SPFLayout,
    pub character_cache: SPFCharacterCache,
    pub letter_spacing: c_ulong,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SPFSurface {
    pub width: c_ulong,
    pub height: c_ulong,
    pub data: *mut c_ulong,
    pub data_length: c_ulong,
}

#[no_mangle]
/// Thin wrapper around [`layout_to_data`] compatible with the C ABI.
///
/// This function takes a [`SPFLayout`] struct and converts it into a Rust native [`Layout`] struct.
/// The [`Layout`] struct is then parsed into a [`Vec<u8>`] with the [`layout_to_data`] function.
/// The [`Vec<u8>`] is then converted into a [`SPFData`] struct and returned.
pub extern "C" fn spf_core_layout_to_data(layout: SPFLayout) -> SPFData {
    let mut data = layout_to_data(&layout.try_into().unwrap()).into_boxed_slice();
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
    let layout = layout_from_data(data.to_owned()).unwrap();
    layout.try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn spf_cache_SPFCharacterCache_empty() -> SPFCharacterCache {
    CharacterCache::empty().try_into().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn spf_cache_SPFCharacterCache_from_characters(
    characters: *mut SPFCharacter,
    characters_length: c_ulong,
) -> SPFCharacterCache {
    let characters: Vec<Character> = unsafe {
        slice::from_raw_parts(characters, characters_length as usize)
            .iter()
            .map(|c| c.try_into().unwrap())
            .collect()
    };
    CharacterCache::from_characters(&characters)
        .try_into()
        .unwrap()
}

#[no_mangle]
pub extern "C" fn spf_printer_SPFPrinter_from_font(font: SPFLayout) -> SPFPrinter {
    let printer = Printer::from_font(font.try_into().unwrap());
    printer.try_into().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn spf_printer_SPFPrinter_print(
    printer: SPFPrinter,
    text: *const c_char,
) -> SPFSurface {
    let printer: Printer = printer.try_into().unwrap();
    let text = unsafe { CStr::from_ptr(text).to_str().unwrap() };
    printer.print(text.to_string()).try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn spf_printer_SPFSurface_blank(width: c_ulong, height: c_ulong) -> SPFSurface {
    Surface::blank(width as usize, height as usize)
        .try_into()
        .unwrap()
}

#[no_mangle]
pub extern "C" fn spf_printer_SPFSurface_get_pixel(
    surface: SPFSurface,
    x: c_ulong,
    y: c_ulong,
) -> c_ulong {
    let surface: Surface = surface.try_into().unwrap();
    surface.get_pixel(x as usize, y as usize).unwrap() as c_ulong
}

#[no_mangle]
pub extern "C" fn spf_printer_SPFSurface_blit(
    surface: SPFSurface,
    surface2: SPFSurface,
    x: c_ulong,
    y: c_ulong,
) {
    let mut surface: Surface = surface.try_into().unwrap();
    let surface2: Surface = surface2.try_into().unwrap();
    surface.blit(&surface2, x as usize, y as usize);
}

#[no_mangle]
pub extern "C" fn spf_printer_SPFSurface_flip_vertical(surface: SPFSurface) -> SPFSurface {
    let surface: Surface = surface.try_into().unwrap();
    surface.flip_vertical().try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn spf_printer_SPFSurface_flip_horizontal(surface: SPFSurface) -> SPFSurface {
    let surface: Surface = surface.try_into().unwrap();
    surface.flip_horizontal().try_into().unwrap()
}
