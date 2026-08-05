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

/// Converts an `Option<Vec<T>>` into a `(pointer, length)` pair for the FFI boundary. A `None` or empty vec produces a null pointer and length `0`.
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

/// Converts a `Vec<T>` into a `(pointer, length)` pair for the FFI boundary. An empty vec produces a null pointer and length `0`.
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

/// Converts a `Vec<T>` of struct elements into a `(pointer, length)` pair for the FFI boundary, converting each element to `$item_type` via `TryInto` first.
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

/// Reconstructs a `Vec<T>` of struct elements from an FFI `(pointer, length)` pair, converting each element from its raw form via `TryInto`.
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

/// Converts an FFI `has_field`/`field` pair back into an `Option<T>`: `None` if `has_field` is `0`, otherwise `Some(field)`.
macro_rules! ffi_to_option {
    ($has_field:expr, $field:expr) => {{
        if $has_field == 0 {
            None
        } else {
            Some($field)
        }
    }};
}

pub(crate) use {
    ffi_to_option, option_vec_to_raw, vec_from_raw_with_conversion, vec_to_raw,
    vec_to_raw_with_conversion,
};
