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

// Macro to convert an Option<Vec> into a raw pointer and length
#[macro_export]
macro_rules! option_vec_to_raw {
    ($vec:expr) => {{
        let len = if let Some(vec) = &$vec { vec.len() } else { 0 };
        let ptr = if len == 0 {
            core::ptr::null_mut()
        } else {
            let mut boxed = $vec.clone().unwrap().into_boxed_slice();
            let ptr = boxed.as_mut_ptr();
            core::mem::forget(boxed);
            ptr
        };
        (ptr, len)
    }};
}

// Macro to convert a Vec into a raw pointer and length
#[macro_export]
macro_rules! vec_to_raw {
    ($vec:expr) => {{
        let len = $vec.len();
        let ptr = if len == 0 {
            core::ptr::null_mut()
        } else {
            let mut boxed = $vec.into_boxed_slice();
            let ptr = boxed.as_mut_ptr();
            core::mem::forget(boxed);
            ptr
        };
        (ptr, len)
    }};
}

#[macro_export]
// Macro to convert a Vec with element conversion into a raw pointer and length.
// Used for vectors with elements of structs.
macro_rules! vec_to_raw_with_conversion {
    ($vec:expr, $item_type:ty) => {{
        let len = $vec.len();
        let mut converted: Vec<$item_type> = Vec::with_capacity(len);
        for item in $vec {
            converted.push(item.try_into()?);
        }
        vec_to_raw!(converted)
    }};
}

#[macro_export]
// Macro to reconstruct a Vec from raw pointer and length, given the vector has struct elements.
macro_rules! vec_from_raw_with_conversion {
    ($ptr:expr, $len:expr) => {{
        let len = $len as usize;
        let mut vec = Vec::with_capacity(len);
        for index in 0..len {
            let item = &*$ptr.add(index);
            vec.push(item.try_into()?);
        }
        vec
    }};
}

#[macro_export]
// Macro for FFI to Option<T> conversion
macro_rules! ffi_to_option {
    ($has_field:expr, $field:expr) => {{
        if $has_field == 0 {
            None
        } else {
            Some($field)
        }
    }};
}
