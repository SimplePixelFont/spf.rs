# Welcome to the spf.rs wiki!

"The Rust parser for simple yet expressive pixel fonts!"

# Synoposis

spf.rs is a parser library built in Rust for the [SimplePixelFont file specifications](https://github.com/SimplePixelFont/Specification). It is both a native crate (spf) and also an FFI library (libspf). As a result you can use the parser in a variety of programming languages which support library loading and the platform specific C ABI. This includes the following languages and more; C/C++, Julia, Python, JS, WASM (no official builds hosted yet). spf.rs is also the official and maintained parser for the SPF specifications and the project will always attempt to parallel developments within the specifications.

# Guides

The following lists documents which you can read to learn about certain topics in regards to spf.rs.

[Installing spf.rs](https://github.com/SimplePixelFont/spf.rs/wiki/InstallingSPFRS) - Guide for installing `spf.rs` crate and dynamic FFI library.  
[Getting Started in Rust](https://github.com/SimplePixelFont/spf.rs/wiki/GettingStartedInRust) - Guide for `spf.rs` basics and understanding the file format via Rust.  
[C Usage](https://github.com/SimplePixelFont/spf.rs/wiki/UsingSPFRSWithFFI) - Guide for using `spf.rs` as an FFI library in C.  
[Tagging with spf.rs](https://github.com/SimplePixelFont/spf.rs/wiki/TaggingWithSPFRS) - Guide for using the `tagging` feature to inspect a `.spf` file's byte-level structure.
