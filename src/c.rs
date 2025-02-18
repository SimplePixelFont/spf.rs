use super::core::Layout;
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

pub(crate) fn to_c_layout(layout: Layout) -> CLayout {
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

#[no_mangle]
pub extern "C" fn c_core_layout_from_data(pointer: *const c_uchar, length: c_ulong) -> CLayout {
    let data = unsafe { slice::from_raw_parts(pointer, length as usize) };
    let layout = Layout::from_data(data.to_owned());
    let clayout = to_c_layout(layout);
    return clayout;
}
