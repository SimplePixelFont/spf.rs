[![The-Nice-One - spf.rs](https://img.shields.io/static/v1?label=The-Nice-One&message=spf.rs&color=orange&logo=github)](https://github.com/The-Nice-One/spf.rs "Go to GitHub repo")
[![stars - spf.rs](https://img.shields.io/github/stars/The-Nice-One/spf.rs?style=social)](https://github.com/The-Nice-One/spf.rs)
[![forks - spf.rs](https://img.shields.io/github/forks/The-Nice-One/spf.rs?style=social)](https://github.com/The-Nice-One/spf.rs)
[![Rust](https://github.com/The-Nice-One/spf.rs/workflows/Rust/badge.svg)](https://github.com/The-Nice-One/spf.rs/actions?query=workflow:"Rust")
[![GitHub tag](https://img.shields.io/github/tag/The-Nice-One/spf.rs?include_prereleases=&sort=semver&color=orange)](https://github.com/The-Nice-One/spf.rs/releases/)
[![License](https://img.shields.io/badge/License-Unlicense-orange)](#license)
[![issues - spf.rs](https://img.shields.io/github/issues/The-Nice-One/spf.rs)](https://github.com/The-Nice-One/spf.rs/issues)
[![codecov](https://codecov.io/gh/The-Nice-One/spf.rs/graph/badge.svg?token=MPXNW4AUJD)](https://codecov.io/gh/The-Nice-One/spf.rs)

A very simple and concrete parser library for the [SimplePixelFont file specifications](https://github.com/SimplePixelFont/Specification), written in Rust. `spf.rs` is both a native crate and also an FFI library which can be used  in a variety of other programming languages which support library loading. `spf.rs` is additionally shipped with features/modules to help integration be faster and easier for you next pixelated project.

### Installation

- To install `spf.rs` as a rust crate run the following command in your cargo project or [read more](https://docs.rs/spf/0.4.0/spf/articles/installing/index.html#installing-with-cargo-and-rust):
```sh
cargo add spf
```

- To use `spf.rs` as an FFI library in your language of choice you must first download a pre-built library version of `spf.rs` from the [releases section](https://github.com/The-Nice-One/spf.rs/releases) (a corrosponding header file is also included if you are programming in C/C++). Please note that pre-built binaries are only avaiable for Windows and Linux-x86-64bit architectures. As a result you may want to [compile `spf.rs` from source](https://docs.rs/spf/0.4.0/spf/articles/installing/index.html#compiling-spfrs-from-source), specifically if a pre-built binary is not availible for you.

### Usage

Usage varies depending on the programming language you choose. For a guide using the native Rust interface check out the [Getting Started in Rust](https://docs.rs/spf/0.4.0/spf/articles/getting_started/index.html) article. You can also check out the [Using the FFI in C](https://docs.rs/spf/0.4.0/spf/articles/c_usage/index.html) article for usage with the `spf.rs` library.


### Supported File Properties
| Flag | Type | Stability | Notes |
| --- | --- | --- | --- |
| Alignment | Configuration | ⚠️ | `Only height-aligned fonts are supported` |
| Compact | Modifier | ✔ | `Added in v0.4` |

Key:
- `⚠️` = Work in progress
- `❌` = Not implemented
- `✔` = Stable
