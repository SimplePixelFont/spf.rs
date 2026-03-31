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

//! Opaque engine handles for stepped deserialization and serialization over the FFI.
//!
//! These types expose [`DeserializeEngine`] and [`SerializeEngine`] as heap-allocated,
//! pointer-sized handles that callers can hold and pass across call boundaries without
//! needing to know their internal layout.
//!
//! # Ownership and lifetimes
//!
//! **Deserialize:** [`spf_deserialize_engine_new`] copies the caller's byte buffer into the
//! handle, so the original buffer may be freed immediately after the call returns. The handle
//! is valid until [`spf_deserialize_engine_free`] is called.
//!
//! **Serialize:** [`spf_serialize_engine_new`] converts the caller's [`SPFLayout`] into an
//! owned Rust [`Layout`] stored inside the handle. The original [`SPFLayout`] is not consumed
//! and remains the caller's responsibility. The handle is valid until
//! [`spf_serialize_engine_free`] is called.
//!
//! # Typical usage
//!
//! ```c
//! // Deserialize
//! SPFDeserializeEngine *engine = NULL;
//! spf_deserialize_engine_new(data_ptr, data_len, &engine);
//! spf_deserialize_engine_run(engine);
//! SPFLayout layout;
//! spf_deserialize_engine_get_layout(engine, &layout);
//! spf_deserialize_engine_free(engine);
//! // ... use layout ...
//! spf_free_layout(layout);
//!
//! // Serialize
//! SPFSerializeEngine *engine = NULL;
//! spf_serialize_engine_new(&layout, &engine);
//! spf_serialize_engine_run(engine);
//! SPFData out;
//! spf_serialize_engine_get_data(engine, &out);
//! spf_serialize_engine_free(engine);
//! // ... use out.data ...
//! spf_free_data(out);
//! ```

use super::*;

/// Opaque handle for a concrete [`DeserializeEngine`].
///
/// Obtain via [`spf_deserialize_engine_new`]. Free via [`spf_deserialize_engine_free`].
pub struct SPFDeserializeEngine {
    /// Owned copy of the input byte buffer. Stored here so the caller does not need
    /// to keep it alive; the engine reads from this allocation during `run`.
    data: Box<[u8]>,
    /// The deserialized layout, populated after a successful call to
    /// [`spf_deserialize_engine_run`].
    result: Option<Layout>,
}

#[no_mangle]
/// Allocates a new deserialize engine handle and copies `len` bytes from `ptr` into it.
///
/// The original buffer at `ptr` may be freed after this call returns.
/// Writes the new handle into `*out` and returns [`SPFStatus::Ok`] on success.
/// On failure `*out` is not written.
pub unsafe extern "C" fn spf_deserialize_engine_new(
    ptr: *const c_uchar,
    len: c_ulong,
    out: *mut *mut SPFDeserializeEngine,
) -> SPFStatus {
    let data = unsafe { core::slice::from_raw_parts(ptr, len as usize) };
    let handle = Box::new(SPFDeserializeEngine {
        data: data.into(),
        result: None,
    });
    unsafe {
        *out = Box::into_raw(handle);
    }
    SPFStatus::Ok
}

#[no_mangle]
/// Runs the deserialize engine against the buffer that was copied during
/// [`spf_deserialize_engine_new`].
///
/// On success the result is stored in the handle and can be retrieved with
/// [`spf_deserialize_engine_get_layout`]. Calling this function more than once re-runs
/// deserialization and overwrites any previous result.
pub unsafe extern "C" fn spf_deserialize_engine_run(
    engine: *mut SPFDeserializeEngine,
) -> SPFStatus {
    let handle = unsafe { &mut *engine };

    // SAFETY: `handle.data` is a `Box<[u8]>` at a stable heap address. We extend its
    // lifetime to `'static` only for the duration of this function call. The engine is
    // stack-local and is fully dropped before we return, so the reference cannot escape.
    let data: &'static [u8] =
        unsafe { &*(handle.data.as_ref() as *const [u8]) };

    let mut de_engine = DeserializeEngine::from_data(data);
    match deserialize_with_engine(&mut de_engine) {
        Ok(()) => {
            handle.result = Some(de_engine.layout);
            SPFStatus::Ok
        }
        Err(e) => SPFStatus::from(e),
    }
}

#[no_mangle]
/// Converts the engine's stored [`Layout`] result into an [`SPFLayout`] and writes it into `*out`.
///
/// [`spf_deserialize_engine_run`] must have completed successfully before calling this.
/// If the engine has not been run, or the last run failed, returns
/// [`SPFStatus::ErrUnexpectedEndOfFile`].
///
/// Each call allocates a fresh [`SPFLayout`] with its own heap memory; the caller must
/// free every returned layout with [`free::spf_free_layout`].
pub unsafe extern "C" fn spf_deserialize_engine_get_layout(
    engine: *mut SPFDeserializeEngine,
    out: *mut SPFLayout,
) -> SPFStatus {
    let handle = unsafe { &*engine };
    match &handle.result {
        Some(layout) => match layout.clone().try_into() {
            Ok(spf_layout) => {
                unsafe { *out = spf_layout };
                SPFStatus::Ok
            }
            Err(e) => SPFStatus::from(e),
        },
        None => SPFStatus::ErrUnexpectedEndOfFile,
    }
}

#[no_mangle]
/// Frees the deserialize engine handle.
///
/// After this call the pointer is invalid. Any [`SPFLayout`] previously retrieved via
/// [`spf_deserialize_engine_get_layout`] is unaffected and remains the caller's responsibility.
/// A null pointer is safe to pass.
pub unsafe extern "C" fn spf_deserialize_engine_free(engine: *mut SPFDeserializeEngine) {
    if !engine.is_null() {
        unsafe {
            drop(Box::from_raw(engine));
        }
    }
}

/// Opaque handle for a concrete [`SerializeEngine`].
///
/// Obtain via [`spf_serialize_engine_new`]. Free via [`spf_serialize_engine_free`].
pub struct SPFSerializeEngine {
    /// Owned Rust representation of the layout to serialize. Converted from the caller's
    /// [`SPFLayout`] during [`spf_serialize_engine_new`]; independent of the original.
    layout: Box<Layout>,
    /// The serialized byte buffer, populated after a successful call to
    /// [`spf_serialize_engine_run`]. Taken out by [`spf_serialize_engine_get_data`].
    result: Option<Box<[u8]>>,
}

#[no_mangle]
/// Allocates a new serialize engine handle by converting `*layout` into an owned Rust
/// [`Layout`]. The original [`SPFLayout`] is not consumed and remains the caller's
/// responsibility.
///
/// Writes the new handle into `*out` and returns [`SPFStatus::Ok`] on success.
/// On failure `*out` is not written.
pub unsafe extern "C" fn spf_serialize_engine_new(
    layout: *const SPFLayout,
    out: *mut *mut SPFSerializeEngine,
) -> SPFStatus {
    let spf_layout = unsafe { core::ptr::read(layout) };
    let rust_layout: Layout = match spf_layout.try_into() {
        Ok(l) => l,
        Err(e) => return SPFStatus::from(e),
    };
    let handle = Box::new(SPFSerializeEngine {
        layout: Box::new(rust_layout),
        result: None,
    });
    unsafe {
        *out = Box::into_raw(handle);
    }
    SPFStatus::Ok
}

#[no_mangle]
/// Runs the serialize engine against the [`Layout`] stored in the handle.
///
/// On success the output is buffered inside the handle and can be retrieved with
/// [`spf_serialize_engine_get_data`]. Calling this function more than once re-runs
/// serialization and overwrites any previous (unretrieved) result.
pub unsafe extern "C" fn spf_serialize_engine_run(engine: *mut SPFSerializeEngine) -> SPFStatus {
    let handle = unsafe { &mut *engine };

    // SAFETY: `handle.layout` is a `Box<Layout>` at a stable heap address. We extend its
    // lifetime to `'static` only for the duration of this function call. The engine is
    // stack-local and is fully dropped before we return, so the reference cannot escape.
    let layout: &'static Layout =
        unsafe { &*(handle.layout.as_ref() as *const Layout) };

    let mut ser_engine = SerializeEngine::from_layout(layout);
    match serialize_with_engine(&mut ser_engine) {
        Ok(()) => {
            handle.result = Some(ser_engine.data_owned().into_boxed_slice());
            SPFStatus::Ok
        }
        Err(e) => SPFStatus::from(e),
    }
}

#[no_mangle]
/// Moves the serialized byte buffer out of the handle and into `*out` as an [`SPFData`].
///
/// [`spf_serialize_engine_run`] must have completed successfully before calling this.
/// The result is consumed on retrieval — subsequent calls without another `run` will
/// return [`SPFStatus::ErrUnexpectedEndOfFile`].
///
/// The caller is responsible for freeing the returned buffer with [`free::spf_free_data`].
pub unsafe extern "C" fn spf_serialize_engine_get_data(
    engine: *mut SPFSerializeEngine,
    out: *mut SPFData,
) -> SPFStatus {
    let handle = unsafe { &mut *engine };
    match handle.result.take() {
        Some(mut boxed) => {
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
        None => SPFStatus::ErrUnexpectedEndOfFile,
    }
}

#[no_mangle]
/// Frees the serialize engine handle.
///
/// After this call the pointer is invalid. Any [`SPFData`] previously retrieved via
/// [`spf_serialize_engine_get_data`] is unaffected and remains the caller's responsibility.
/// A null pointer is safe to pass.
pub unsafe extern "C" fn spf_serialize_engine_free(engine: *mut SPFSerializeEngine) {
    if !engine.is_null() {
        unsafe {
            drop(Box::from_raw(engine));
        }
    }
}