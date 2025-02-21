use super::core::*;
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
    pub byte_map: *mut c_uchar,
    pub byte_map_length: c_ulong,
}

#[derive(Debug)]
#[repr(C)]
pub struct CBody {
    pub characters: *mut CCharacter,
    pub characters_length: c_ulong,
}

/// Converts a Rust native [`Layout`] struct into a C ABI compatible [`CLayout`] struct.
pub fn to_c_layout(layout: Layout) -> CLayout {
    let characters_len = layout.body.characters.len();
    let mut characters = Vec::with_capacity(characters_len);

    for character in layout.body.characters {
        let byte_map_len = character.byte_map.len();
        let byte_map_ptr = if byte_map_len == 0 {
            std::ptr::null_mut()
        } else {
            let mut byte_map_vec = character.byte_map.into_boxed_slice();
            let ptr = byte_map_vec.as_mut_ptr();
            std::mem::forget(byte_map_vec);
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
            byte_map: byte_map_ptr,
            byte_map_length: byte_map_len as c_ulong,
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
            let byte_map =
                slice::from_raw_parts(character.byte_map, character.byte_map_length as usize);

            characters.push(Character {
                utf8: utf8,
                custom_size: custom_size,
                byte_map: byte_map.to_vec(),
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

#[repr(C)]
/// Used to represent a [`Vec<u8>`] in the C ABI.
pub struct CData {
    pub data: *mut c_uchar,
    pub data_length: c_ulong,
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
