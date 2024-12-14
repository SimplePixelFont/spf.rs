[![The-Nice-One - spf.rs](https://img.shields.io/static/v1?label=The-Nice-One&message=spf.rs&color=orange&logo=github)](https://github.com/The-Nice-One/spf.rs "Go to GitHub repo")
[![stars - spf.rs](https://img.shields.io/github/stars/The-Nice-One/spf.rs?style=social)](https://github.com/The-Nice-One/spf.rs)
[![forks - spf.rs](https://img.shields.io/github/forks/The-Nice-One/spf.rs?style=social)](https://github.com/The-Nice-One/spf.rs)
[![Rust](https://github.com/The-Nice-One/spf.rs/workflows/Rust/badge.svg)](https://github.com/The-Nice-One/spf.rs/actions?query=workflow:"Rust")
[![GitHub tag](https://img.shields.io/github/tag/The-Nice-One/spf.rs?include_prereleases=&sort=semver&color=orange)](https://github.com/The-Nice-One/spf.rs/releases/)
[![License](https://img.shields.io/badge/License-Unlicense-orange)](#license)
[![issues - spf.rs](https://img.shields.io/github/issues/The-Nice-One/spf.rs)](https://github.com/The-Nice-One/spf.rs/issues)

A very simple and concrete parser for `*.spf` ([SimplePixelFont](https://github.com/SimplePixelFont)) files for Rust. spf.rs provides
simple encoding and decoding for the `*.spf` binary representation through a `Vec<u8>`. And also
includes optional features to conveniently create a texture from a font rendering, which
can then be used in your favorite game engine or graphics framework.

### Example
Creates a new `SimplePixelFont` struct and adds the characters `o`, `w`, and `😊`.
```rs
use spf::core::*;

fn main() {
    let mut font = SimplePixelFont::new(
        ConfigurationFlags {
            0: ALIGNMENT_HEIGHT,
            ..Default::default()
        },
        ModifierFlags {
            ..Default::default()
        },
        4,
    );
    font.add_character(Character::inferred(
        'o',
        Bitmap::inferred(&[
            true, true, true, true,
            true, false, false, true,
            true, false, false, true,
            true, true, true, true,
        ]),
    ));
    font.add_character(Character::inferred(
        'w',
        Bitmap::inferred(&[
            true, false, true, false, true,
            true, false, true, false, true,
            true, false, true, false, true,
            true, true, true, true, true,
        ]),
    ));
    font.add_character(Character::inferred(
        '😊',
        Bitmap::inferred(&[
            false, true, true, false,
            false, false, false, false,
            true, false, false, true,
            false, true, true, false,
        ]),
    ));
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
| Compact | Modifier | ❌ | `Planned for v0.4` |

Key:
- `⚠️` = Work in progress
- `❌` = Not implemented
- `✔` = Stable
