[![SimplePixelFont - spf.rs](https://img.shields.io/static/v1?label=SimplePixelFont&message=spf.rs&color=orange&logo=github)](https://github.com/SimplePixelFont/spf.rs "Go to GitHub repo")
[![stars - spf.rs](https://img.shields.io/github/stars/SimplePixelFont/spf.rs?style=social)](https://github.com/SimplePixelFont/spf.rs)
[![forks - spf.rs](https://img.shields.io/github/forks/SimplePixelFont/spf.rs?style=social)](https://github.com/SimplePixelFont/spf.rs)
[![Rust](https://github.com/SimplePixelFont/spf.rs/workflows/Rust/badge.svg)](https://github.com/SimplePixelFont/spf.rs/actions?query=workflow:"rust")
[![GitHub tag](https://img.shields.io/github/tag/SimplePixelFont/spf.rs?include_prereleases=&sort=semver&color=orange)](https://github.com/SimplePixelFont/spf.rs/releases/)
[![License](https://img.shields.io/badge/License-Unlicense-orange)](#license)
[![issues - spf.rs](https://img.shields.io/github/issues/SimplePixelFont/spf.rs)](https://github.com/SimplePixelFont/spf.rs/issues)
[![Coverage](https://img.shields.io/codecov/c/gh/SimplePixelFont/spf.rs)](https://codecov.io/gh/SimplePixelFont/spf.rs)
[![Documentation](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/The-Nice-One/cfebb0fe555ac7e77ada109c469cdeb4/raw/documentation.json)](https://gist.github.com/The-Nice-One/cfebb0fe555ac7e77ada109c469cdeb4#file-documentation-md)
![Lint](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/The-Nice-One/cfebb0fe555ac7e77ada109c469cdeb4/raw/lint.json)
![Tests](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/The-Nice-One/cfebb0fe555ac7e77ada109c469cdeb4/raw/test.json)

A very simple and concrete parser library for the [SimplePixelFont file specifications](https://github.com/SimplePixelFont/Specification), written in Rust. `spf.rs` is both a native crate and also an FFI library which can be used  in a variety of other programming languages which support library loading. `spf.rs` is additionally shipped with features/modules to help integration be faster and easier for you next pixelated project.

### Installation

- To install `spf.rs` as a rust crate run the following command in your cargo project or [read more](https://docs.rs/spf/latest/spf/articles/installing/index.html#installing-with-cargo-and-rust):
```sh
cargo add spf
```

- To use `spf.rs` as an FFI library in your language of choice you must first download a pre-built library version of `spf.rs` from the [releases section](https://github.com/SimplePixelFont/spf.rs/releases) (a corrosponding header file is also included if you are programming in C/C++). Please note that pre-built binaries are only avaiable for Windows and Linux-x86-64bit architectures. As a result you may want to [compile `spf.rs` from source](https://docs.rs/spf/latest/spf/articles/installing/index.html#compiling-spfrs-from-source), specifically if a pre-built binary is not availible for you.

### Usage

Usage varies depending on the programming language you choose. For a guide using the native Rust interface check out the [Getting Started in Rust](https://docs.rs/spf/latest/spf/articles/getting_started/index.html) article. You can also check out the [Using the FFI in C](https://docs.rs/spf/latest/spf/articles/c_usage/index.html) article for usage with the `spf.rs` library.

### Supported SPF Header Properties
| Flag | Type | Stability | Notes |
| --- | --- | --- | --- |
| Constant Cluster Codepoints | Configuration | ✔ | `Added in v0.5` |
| Constant Width | Configuration | ✔ | `Added in v0.5` |
| Constant Height | Configuration | ✔ | `Added in v0.5` |
| Compact | Modifier | ✔ | `Added in v0.4` |

Key:
- `⚠️` = Work in progress
- `❌` = Not implemented
- `✔` = Stable
