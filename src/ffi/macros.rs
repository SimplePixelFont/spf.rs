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
