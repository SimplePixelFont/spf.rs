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

#[cfg(feature = "log")]
use crate::log::*;

use std::ffi::*;
use std::slice;

pub mod converters;

#[derive(Debug)]
#[repr(C)]
pub struct SPFLayout {
    pub header: SPFHeader,
    pub body: SPFBody,
}

#[derive(Debug)]
#[repr(C)]
pub struct SPFHeader {
    pub configuration_flags: SPFConfigurationFlags,
    pub modifier_flags: SPFModifierFlags,
    pub configuration_values: SPFConfigurationValues,
}

#[derive(Debug)]
#[repr(C)]
pub struct SPFConfigurationFlags {
    pub constant_cluster_codepoints: c_uchar,
    pub constant_width: c_uchar,
    pub constant_height: c_uchar,
}

#[derive(Debug)]
#[repr(C)]
pub struct SPFModifierFlags {
    pub compact: c_uchar,
}

#[derive(Debug)]
#[repr(C)]
pub struct SPFConfigurationValues {
    pub constant_cluster_codepoints: c_uchar,
    pub constant_width: c_uchar,
    pub constant_height: c_uchar,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct SPFCharacter {
    pub grapheme_cluster: *const c_char,
    pub custom_width: c_uchar,
    pub custom_height: c_uchar,
    pub pixmap: *mut c_uchar,
    pub pixmap_length: c_ulong,
}

#[derive(Debug)]
#[repr(C)]
pub struct SPFBody {
    pub characters: *mut SPFCharacter,
    pub characters_length: c_ulong,
}

#[repr(C)]
/// Used to represent a [`Vec<u8>`] in the C ABI. This is simply a `u_char` array on the heap which can be reconstructed with the pointer `data` and length `data_length`.
pub struct SPFData {
    pub data: *mut c_uchar,
    pub data_length: c_ulong,
}

#[repr(C)]
pub struct SPFCharacterCache {
    pub mappings_keys: *mut *const c_char,
    pub mappings_values: *mut c_ulong,
    pub mappings_length: c_ulong,
}

#[repr(C)]
pub struct SPFPrinter {
    pub font: SPFLayout,
    pub character_cache: SPFCharacterCache,
    pub letter_spacing: c_ulong,
}

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
    std::mem::forget(data);
    return SPFData {
        data: data_ptr,
        data_length: data_length,
    };
}

#[no_mangle]
/// Thin wrapper around [`layout_from_data`] compatible with the C ABI.
///
/// This function takes a pointer to a [`c_uchar`] array with a length of [`c_ulong`] and creates a
/// [`Vec<u8>`] from the data. This data is then passed to the [`layout_from_data`] function to
/// create a [`Layout`] struct. The [`Layout`] struct is then converted into a [`SPFLayout`] struct
/// and returned.
pub extern "C" fn spf_core_layout_from_data(pointer: *const c_uchar, length: c_ulong) -> SPFLayout {
    let data = unsafe { slice::from_raw_parts(pointer, length as usize) };
    let layout = layout_from_data(data.to_owned()).unwrap();
    return layout.try_into().unwrap();
}

#[no_mangle]
pub extern "C" fn spf_log_LOGGER_set_log_level(log_level: c_uchar) {
    let log_level = match log_level {
        0 => LogLevel::None,
        1 => LogLevel::Info,
        2 => LogLevel::Debug,
        _ => panic!("Invalid log level."),
    };
    LOGGER_set_log_level(log_level);
}

#[no_mangle]
pub extern "C" fn spf_cache_SPFCharacterCache_empty() -> SPFCharacterCache {
    CharacterCache::empty().try_into().unwrap()
}

#[no_mangle]
pub extern "C" fn spf_cache_SPFCharacterCache_from_characters(
    characters: *mut SPFCharacter,
    characters_length: c_ulong,
) -> SPFCharacterCache {
    let characters = unsafe {
        slice::from_raw_parts(characters, characters_length as usize)
            .into_iter()
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
pub extern "C" fn spf_printer_SPFPrinter_print(
    printer: SPFPrinter,
    text: *const c_char,
) -> SPFSurface {
    let printer: Printer = printer.try_into().unwrap();
    let text = unsafe { CStr::from_ptr(text).to_str().unwrap() };
    printer.print(text.to_string()).try_into().unwrap()
}
