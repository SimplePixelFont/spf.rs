#![allow(unused_imports)] // Otherwise we get warnings about unused imports even though they are used in the docs.
//! Helpful guides and learning resources to integrate spf.rs in your next project.

pub(crate) use crate::core::*;
pub(crate) use crate::ergonomics::*;
pub(crate) use crate::printer::*;
pub(crate) use crate::*;

/// Guide for `spf.rs` printer module and creating text renderings.
pub mod printer_guide {
    #![doc = include_str!("../res/articles/PrinterModuleAndTextRenderings.md")]
}

/// Guide for spf.rs basics and understanding the file format via Rust.
pub mod getting_started {
    #![doc = include_str!("../res/articles/GettingStartedInRust.md")]
}

/// Guide for installing `spf.rs` crate and dynamic FFI library.
pub mod installing {
    #![doc = include_str!("../res/articles/InstallingSPFRS.md")]
}

/// Guide for using `spf.rs` as an FFI library in C.
pub mod c_usage {
    #![doc = include_str!("../res/articles/UsingSPFRSWithFFI.md")]
}
