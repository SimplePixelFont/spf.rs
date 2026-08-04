# Getting Started in Rust

### Synopsis
`spf.rs` brings the world of [`SimplePixelFont`](https://github.com/SimplePixelFont)(s) into the programming realm. Written in the Rust
programming language, `spf.rs` aims to be effective and simple to use, providing a native crate for
Rust and also an FFI compatible with C-like languages and C-interoperable languages. In particular, `spf.rs` provides a
low-level interface to the binary representation of [`SimplePixelFont`](https://github.com/SimplePixelFont) files via the [`crate::core`]
module.

### Resources
It's important that before beginning you have a general understanding of the Rust programming
language. Additionally, this guide will explain the Rust representation of a [`SimplePixelFont`](https://github.com/SimplePixelFont) which aims to closely reflect the binary file representation. However, you are encouraged to learn and use the official SPF specifications as an additional reference.

### The [`crate::core`] module
The [`crate::core`] module contains the lowest-level structures to
represent a [`SimplePixelFont`](https://github.com/SimplePixelFont) file. Included is the [`core::Layout`] struct which is
the binary representation of a [`SimplePixelFont`](https://github.com/SimplePixelFont) file as a Rust structure. A [`core::Layout`] holds a
list of tables — [`core::CharacterTable`], [`core::PixmapTable`], [`core::ColorTable`], and [`core::FontTable`] — each
holding its own records. Every one of these structs is `#[non_exhaustive]`, so you build them with
`::default()` and then assign the fields you need, rather than a full struct literal. Let's build a
minimal font with one character, `"w"`, backed by one pixmap:

```rs
use spf::core::*;

let mut pixmap = Pixmap::default();
pixmap.data = vec![0b10111111, 0b01010110]; // A 5x3, 1-bit-per-pixel glyph, packed LSB-first.

let mut pixmap_table = PixmapTable::default();
pixmap_table.configuration_flags = PixmapTableConfigurationFlags::ConstantWidth
    | PixmapTableConfigurationFlags::ConstantHeight
    | PixmapTableConfigurationFlags::ConstantBitsPerPixel;
pixmap_table.constant_width = Some(5);
pixmap_table.constant_height = Some(3);
pixmap_table.constant_bits_per_pixel = Some(1);
pixmap_table.pixmaps = vec![pixmap];

let mut character = Character::default();
character.code_points = String::from("w"); // May be made up of multiple utf8 characters, e.g. "😊".
character.pixmap_index = Some(0); // Which pixmap in the linked PixmapTable this character uses.

let mut character_table = CharacterTable::default();
character_table.modifier_flags = CharacterTableModifierFlags::UsePixmapIndex;
character_table.link_flags = CharacterTableLinkFlags::LinkPixmapTables;
character_table.pixmap_table_indexes = Some(vec![0]); // Links to pixmap_tables[0] below.
character_table.characters = vec![character];

let mut layout = Layout::default();
layout.compact = true; // Strips padding bits when converting the struct to data.
layout.pixmap_tables = vec![pixmap_table];
layout.character_tables = vec![character_table];
```

A few things worth calling out:
- [`CharacterTableModifierFlags::UsePixmapIndex`] tells the format that every [`Character`] record carries a `pixmap_index`. Without it, [`Character::pixmap_index`] is never read or written.
- [`CharacterTableLinkFlags::LinkPixmapTables`] plus [`CharacterTable::pixmap_table_indexes`] is how a `CharacterTable` declares which `PixmapTable`(s) its characters' pixmaps live in — `pixmap_index` is then an index into whichever one applies.
- [`PixmapTableConfigurationFlags::ConstantWidth`]/[`ConstantHeight`](PixmapTableConfigurationFlags::ConstantHeight)/[`ConstantBitsPerPixel`](PixmapTableConfigurationFlags::ConstantBitsPerPixel) mean every pixmap in this table shares the same dimensions, so individual [`Pixmap`] records only need to carry `data`, not their own `custom_width`/`custom_height`/`custom_bits_per_pixel`.

Side Note: To learn more about the different configuration flags and modifier flags, check out the
[SPF File Specifications](https://github.com/SimplePixelFont/Specification).

### But what is a character in SimplePixelFonts?
Before we discuss how to add a character to our font, we first need to learn what a character is in the
context of a [`SimplePixelFont`](https://github.com/SimplePixelFont) font.

In simple terms a character in [`SimplePixelFont`](https://github.com/SimplePixelFont) is simply a [`Character::code_points`] string, which may be made up of multiple utf8 characters such as `a`, `<`, `😊`, etc.
A character optionally carries an [`Character::advance_x`], and links to its glyph via [`Character::pixmap_index`] (and, if the table links to more than one [`PixmapTable`], [`Character::pixmap_table_index`]). The glyph itself lives in a [`Pixmap`]'s `data` — a packed vector of bits, one per pixel (at the moment). If a pixel's bit is set the character uses
the pixel, if it's unset the character does not.

Pixels are stored row-major, origin top-left, left to right then top to bottom, packed least-significant-bit-first within each byte — pixel 0 is the lowest bit of `data[0]`, and so on. Here is a diagram which maps
each pixel of a character to their index in the pixmap:

[image link](https://github.com/SimplePixelFont/spf.rs/blob/main/res/articles/res/wInNumberedFramex4.png)
![](https://github.com/SimplePixelFont/spf.rs/blob/main/res/articles/res/wInNumberedFramex4.png?raw=true)

And this will result in the following character:

[image link](https://github.com/SimplePixelFont/spf.rs/blob/main/res/articles/res/wWithoutNumberedFramex4.png)
![](https://github.com/SimplePixelFont/spf.rs/blob/main/res/articles/res/wWithoutNumberedFramex4.png?raw=true)

### Saving & Loading `spf.rs` fonts with [`std::fs`]

We can then encode the layout and use [`std::fs`] to write it to a file:

```rs
let data = layout_to_data(&layout).unwrap();

let mut file = std::fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open("./sampleToyFont.spf")
    .unwrap();
file.write_all(&data).unwrap();
```

Or we can load an existing `.spf` file using [`std::fs`] as well:

```rs
let mut file = std::fs::OpenOptions::new()
    .read(true)
    .open("./sampleToyFont.spf")
    .unwrap();
let mut buffer: Vec<u8> = vec![];
file.read_to_end(&mut buffer).unwrap();
let layout = layout_from_data(&buffer).unwrap();
```
