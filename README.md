[![The-Nice-One - spf.rs](https://img.shields.io/static/v1?label=The-Nice-One&message=spf.rs&color=orange&logo=github)](https://github.com/The-Nice-One/spf.rs "Go to GitHub repo")
[![stars - spf.rs](https://img.shields.io/github/stars/The-Nice-One/spf.rs?style=social)](https://github.com/The-Nice-One/spf.rs)
[![forks - spf.rs](https://img.shields.io/github/forks/The-Nice-One/spf.rs?style=social)](https://github.com/The-Nice-One/spf.rs)
[![Rust](https://github.com/The-Nice-One/spf.rs/workflows/Rust/badge.svg)](https://github.com/The-Nice-One/spf.rs/actions?query=workflow:"Rust")
[![GitHub tag](https://img.shields.io/github/tag/The-Nice-One/spf.rs?include_prereleases=&sort=semver&color=orange)](https://github.com/The-Nice-One/spf.rs/releases/)
[![License](https://img.shields.io/badge/License-Unlicense-orange)](#license)
[![issues - spf.rs](https://img.shields.io/github/issues/The-Nice-One/spf.rs)](https://github.com/The-Nice-One/spf.rs/issues)

A very simple and concrete parser library for the [SimplePixelFont](https://github.com/SimplePixelFont)
file specifications written in Rust. Initially written as a Rust crate, `spf.rs` is now also
a C/C++ ABI-compatible library which can be used in a variety of other programming languages. `spf.rs` is also shipped with additional modules to help integration be faster and easier for you next pixelated project.

### Installation

To install `spf.rs` as a rust crate run the following command in your cargo project:
```sh
cargo add spf
```

To install and use as a C/C++ ABI-compatible library you may want to download a pre-built version of `spf.rs` from the releases section. A corrosponding header file is also included if you are programming in C/C++. Please note that pre-built binaries are only avaiable for windows and linux-x86-64bit architectures. As a result you may want to compile `spf.rs` specifically if a pre-built binary is not availible for you.

### Example
Creates a new `Layout` struct with the characters `o`, `w`, and `😊` using the `LayoutBuilder`.
```rs
use spf::egronomics::*;

fn main() {
    let mut font = LayoutBuilder::new()
        .alignment(ALIGNMENT_HEIGHT)
        .size(4)
        .character('o', &[
            1, 1, 1, 1,
            1, 0, 0, 1,
            1, 0, 0, 1,
            1, 1, 1, 1,
        ])
        .character('w', &[
            1, 0, 1, 0, 1,
            1, 0, 1, 0, 1,
            1, 0, 1, 0, 1,
            1, 1, 1, 1, 1,
        ])
        .character('😊', &[
            0, 1, 1, 0,
            0, 0, 0, 0,
            1, 0, 0, 1,
            0, 1, 1, 0,
        ])
        .build();
}
```
We can then encode the struct and use `std::fs` to write to a file:
```rs
let mut file = std::fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open("./sampleToyFont.spf")
    .unwrap();
file.write_all(&font.to_vec_u8()).unwrap();
```
Or we can load an exsisting .spf file using `std::fs` aswell:
```rs
let mut file = std::fs::OpenOptions::new()
    .read(true)
    .open("./sampleToyFont.spf")
    .unwrap();
let mut buffer: Vec<u8> = vec![];
file.read_to_end(&mut buffer).unwrap();
let font = SimplePixelFont::from_vec_u8(buffer);
```
### Supported File Properties
| Flag | Type | Stability | Notes |
| --- | --- | --- | --- |
| Alignment | Configuration | ⚠️ | `Only height-aligned fonts are supported` |
| Compact | Modifier | ✔ | `Added in v0.4` |

Key:
- `⚠️` = Work in progress
- `❌` = Not implemented
- `✔` = Stable

