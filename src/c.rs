//! A C compatible FFI layer for `spf.rs`.
//!
//! This module provides a thin wrapper around all the modules in `spf.rs` that allows it to be used
//! in a C compatible way exposed through a FFI. This allows `spf.rs` to be used as a library in C and
//! in any language that supports the platform-specific C-ABI through dynamic library loading, including
//! WebAssembly.
//!
//! To learn about how to use the `spf.rs` library in your language of choice, please refer to the
//! [`crate::articles::c_usage`] article. Also note that the ['to_c_layout`] and ['from_c_layout`]
//! functions are not part of the `spf.rs` library and only exposed in the Rust crate.
//!
//! # Conventions
//!
//! Function names are prefixed with `c_` followed by the module name they are in. For example, the
//! function [`c_core_layout_from_data`] is the C ABI compatible version of the [`layout_from_data`]
//! function in the [`crate::core`] module.
//!
//! All structs are prefixed with `C` followed by the struct name. For example, the struct
//! [`CLayout`] is the C ABI compatible version of the [`Layout`] struct in the [`crate::core`] module.
//!
//! All functions that return a [`Vec<u8>`] return a [`CData`] struct instead.

use crate::core::*;

#[cfg(feature = "log")]
use crate::log::*;

use std::ffi::*;
use std::slice;

#[derive(Debug)]
#[repr(C)]
pub struct CLayout {
    pub header: CHeader,
    pub body: CBody,
}

#[derive(Debug)]
#[repr(C)]
pub struct CHeader {
    pub configuration_flags: CConfigurationFlags,
    pub modifier_flags: CModifierFlags,
    pub required_values: CRequiredValues,
}

#[derive(Debug)]
#[repr(C)]
pub struct CConfigurationFlags {
    pub alignment: c_uchar,
}

#[derive(Debug)]
#[repr(C)]
pub struct CModifierFlags {
    pub compact: c_uchar,
}

#[derive(Debug)]
#[repr(C)]
pub struct CRequiredValues {
    pub constant_size: c_uchar,
}

#[derive(Debug)]
#[repr(C)]
pub struct CCharacter {
    pub utf8: *const c_char,
    pub custom_size: c_uchar,
    pub pixmap: *mut c_uchar,
    pub pixmap_length: c_ulong,
}

#[derive(Debug)]
#[repr(C)]
pub struct CBody {
    pub characters: *mut CCharacter,
    pub characters_length: c_ulong,
}

#[repr(C)]
/// Used to represent a [`Vec<u8>`] in the C ABI. This is simply a `u_char` array on the heap which can be reconstructed with the pointer `data` and length `data_length`.
pub struct CData {
    pub data: *mut c_uchar,
    pub data_length: c_ulong,
}

/// Converts a Rust native [`Layout`] struct into a C ABI compatible [`CLayout`] struct.
pub fn to_c_layout(layout: Layout) -> CLayout {
    let characters_len = layout.body.characters.len();
    let mut characters = Vec::with_capacity(characters_len);

    for character in layout.body.characters {
        let pixmap_len = character.pixmap.len();
        let pixmap_ptr = if pixmap_len == 0 {
            std::ptr::null_mut()
        } else {
            let mut pixmap_vec = character.pixmap.into_boxed_slice();
            let ptr = pixmap_vec.as_mut_ptr();
            std::mem::forget(pixmap_vec);
            ptr
        };

        let utf8 = CString::new(
            character
                .utf8
                .to_string()
                .as_bytes()
                .to_vec()
                .into_boxed_slice(),
        )
        .unwrap();
        let utf8_ptr = utf8.as_ptr();
        std::mem::forget(utf8);

        characters.push(CCharacter {
            utf8: utf8_ptr,
            custom_size: character.custom_size,
            pixmap: pixmap_ptr,
            pixmap_length: pixmap_len as c_ulong,
        })
    }

    let characters_ptr = if characters_len == 0 {
        std::ptr::null_mut()
    } else {
        let mut characters_raw = characters.into_boxed_slice();
        let ptr = characters_raw.as_mut_ptr();
        std::mem::forget(characters_raw);
        ptr
    };

    CLayout {
        header: CHeader {
            configuration_flags: CConfigurationFlags {
                alignment: layout.header.configuration_flags.alignment as u8,
            },
            modifier_flags: CModifierFlags {
                compact: layout.header.modifier_flags.compact as u8,
            },
            required_values: CRequiredValues {
                constant_size: layout.header.required_values.constant_size,
            },
        },
        body: CBody {
            characters: characters_ptr,
            characters_length: characters_len as c_ulong,
        },
    }
}

/// Converts a C ABI compatible [`CLayout`] struct into a Rust native [`Layout`] struct.
pub fn from_c_layout(layout: CLayout) -> Layout {
    let characters_len = layout.body.characters_length as usize;
    let mut characters = Vec::with_capacity(characters_len);
    unsafe {
        for index in 0..characters_len {
            let character = &*layout.body.characters.add(index);
            let utf8 = CStr::from_ptr(character.utf8)
                .to_str()
                .unwrap()
                .chars()
                .next()
                .unwrap();
            let custom_size = character.custom_size;
            let pixmap = slice::from_raw_parts(character.pixmap, character.pixmap_length as usize);

            characters.push(Character {
                utf8: utf8,
                custom_size: custom_size,
                pixmap: pixmap.to_vec(),
            });
        }
    }

    Layout {
        header: Header {
            configuration_flags: ConfigurationFlags {
                alignment: layout.header.configuration_flags.alignment != 0,
            },
            modifier_flags: ModifierFlags {
                compact: layout.header.modifier_flags.compact != 0,
            },
            required_values: RequiredValues {
                constant_size: layout.header.required_values.constant_size,
            },
        },
        body: Body {
            characters: characters,
        },
    }
}

#[no_mangle]
/// Thin wrapper around [`layout_from_data`] compatible with the C ABI.
///
/// This function takes a pointer to a [`c_uchar`] array with a length of [`c_ulong`] and creates a
/// [`Vec<u8>`] from the data. This data is then passed to the [`layout_from_data`] function to
/// create a [`Layout`] struct. The [`Layout`] struct is then converted into a [`CLayout`] struct
/// and returned.
pub extern "C" fn c_core_layout_from_data(pointer: *const c_uchar, length: c_ulong) -> CLayout {
    let data = unsafe { slice::from_raw_parts(pointer, length as usize) };
    let layout = layout_from_data(data.to_owned());
    let clayout = to_c_layout(layout);
    return clayout;
}

#[no_mangle]
/// Thin wrapper around [`layout_to_data`] compatible with the C ABI.
///
/// This function takes a [`CLayout`] struct and converts it into a Rust native [`Layout`] struct.
/// The [`Layout`] struct is then parsed into a [`Vec<u8>`] with the [`layout_to_data`] function.
/// The [`Vec<u8>`] is then converted into a [`CData`] struct and returned.
pub extern "C" fn c_core_layout_to_data(layout: CLayout) -> CData {
    let layout = from_c_layout(layout);
    let mut data = layout_to_data(&layout).into_boxed_slice();
    let data_length = data.len() as c_ulong;
    let data_ptr = data.as_mut_ptr();
    std::mem::forget(data);
    return CData {
        data: data_ptr,
        data_length: data_length,
    };
}

#[no_mangle]
pub extern "C" fn c_log_LOGGER_set_log_level(log_level: c_uchar) {
    let log_level = match log_level {
        0 => LogLevel::None,
        1 => LogLevel::Info,
        2 => LogLevel::Debug,
        _ => panic!("Invalid log level."),
    };
    LOGGER_set_log_level(log_level);
}
