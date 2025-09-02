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

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(
    html_logo_url = "https://github.com/SimplePixelFont/spf.rs/blob/main/res/spf.rs.png?raw=true"
)]

#[cfg(not(feature = "std"))]
pub(crate) extern crate alloc;

// #[cfg(feature = "std")]
// pub(crate) use hashbrown::HashMap;
// #[cfg(feature = "std")]
// pub(crate) use std::borrow::ToOwned;
// #[cfg(feature = "std")]
// pub(crate) use std::format;
#[cfg(feature = "std")]
pub(crate) use std::string::String;
// #[cfg(feature = "std")]
// pub(crate) use std::string::ToString;
// #[cfg(feature = "std")]
// pub(crate) use std::vec;
#[cfg(feature = "std")]
pub(crate) use std::vec::Vec;

#[cfg(not(feature = "std"))]
pub(crate) use alloc::borrow::ToOwned;
#[cfg(not(feature = "std"))]
pub(crate) use alloc::format;
#[cfg(not(feature = "std"))]
pub(crate) use alloc::string::String;
#[cfg(not(feature = "std"))]
pub(crate) use alloc::string::ToString;
#[cfg(not(feature = "std"))]
pub(crate) use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
pub(crate) use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
pub(crate) use hashbrown::HashMap;

pub mod core;

// #[cfg_attr(docsrs, doc(cfg(feature = "cache")))]
// #[cfg(feature = "cache")]
// pub mod cache;

// #[cfg_attr(docsrs, doc(cfg(feature = "printer")))]
// #[cfg(feature = "printer")]
// pub mod printer;

#[cfg_attr(docsrs, doc(cfg(feature = "ergonomics")))]
#[cfg(feature = "ergonomics")]
pub mod ergonomics;

// #[cfg_attr(docsrs, doc(cfg(feature = "ffi")))]
// #[cfg(feature = "ffi")]
// pub mod ffi;

// #[cfg_attr(docsrs, doc(cfg(feature = "articles")))]
// #[cfg(feature = "articles")]
// pub mod articles;
