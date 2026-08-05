# Getting Started in Rust

### Synopsis
`spf.rs` brings the world of [`SimplePixelFont`](https://github.com/SimplePixelFont)s into the programming realm. Written in the Rust
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
list of tables such as; [`core::CharacterTable`], [`core::PixmapTable`], [`core::ColorTable`], and [`core::FontTable`]. Each
holding its own records. Every one of these structs is `#[non_exhaustive]`, so you build them with
`::default()` and then assign the fields you need. Let's build a
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
pixmap_table.link_flags = PixmapTableLinkFlags::LinkColorTables;
pixmap_table.color_table_indexes = Some(vec![0]); // Links to color_tables[0] below.
pixmap_table.pixmaps = vec![pixmap];

let mut character = Character::default();
character.code_points = String::from("w"); // May be made up of multiple utf8 characters, like "😊".
character.pixmap_index = Some(0); // Which pixmap in the linked PixmapTable this character uses.

let mut character_table = CharacterTable::default();
character_table.modifier_flags = CharacterTableModifierFlags::UsePixmapIndex;
character_table.link_flags = CharacterTableLinkFlags::LinkPixmapTables;
character_table.pixmap_table_indexes = Some(vec![0]); // Links to pixmap_tables[0] below.
character_table.characters = vec![character];

let mut layout = Layout::default();
layout.compact = true; // Strips padding bits in certain fields when converting the struct to data, saving space.
layout.pixmap_tables = vec![pixmap_table];
layout.character_tables = vec![character_table];
```

A few things worth calling out:
- [`CharacterTableModifierFlags::UsePixmapIndex`] tells the format that every [`Character`] record carries a `pixmap_index`. Without it, [`Character::pixmap_index`] is never read or written.
- [`CharacterTableLinkFlags::LinkPixmapTables`] plus [`CharacterTable::pixmap_table_indexes`] is how a `CharacterTable` declares which `PixmapTable`(s) its characters' pixmaps live in, `pixmap_index` is then an index into whichever one applies.
- We set `pixmap_index` explicitly above, but strictly speaking we didn't have to: per [`CharacterTableModifierFlags::UsePixmapIndex`]'s docs, when that flag is *not* enabled `pixmap_index` defaults to the character's own record index instead which creates a one-to-one mapping. Our only character is record `0`, so it map to pixmap `0` either way. Here we explicitly set the field for clarity and to see a modifier flag in action.
- [`PixmapTableConfigurationFlags::ConstantWidth`]/[`ConstantHeight`](PixmapTableConfigurationFlags::ConstantHeight)/[`ConstantBitsPerPixel`](PixmapTableConfigurationFlags::ConstantBitsPerPixel) mean every pixmap in this table shares the same dimensions, so individual [`Pixmap`] records only need to carry `data`, not their own `custom_width`/`custom_height`/`custom_bits_per_pixel`.

Side Note: To learn more about the different configuration flags and modifier flags, check out the
[SPF File Specifications](https://github.com/SimplePixelFont/Specification).

### But what is a character in SimplePixelFonts?
Lets further discuss what a character is in the
context of a [`SimplePixelFont`](https://github.com/SimplePixelFont) font.

In simple terms a character in [`SimplePixelFont`](https://github.com/SimplePixelFont) is simply a [`Character::code_points`] string, which may be made up of multiple utf8 characters such as `a`, `<`, `😊`, `é`, etc.
A character optionally carries an [`Character::advance_x`], and explicitly links to its glyph via [`Character::pixmap_index`]. Additionally, if the table links to more than one [`PixmapTable`], [`Character::pixmap_table_index`] can be used to specify the exact table the pixmap comes from. 

The glyph itself lives in a [`Pixmap`]'s `data` which is a packed vector of bits, one per pixel is shown in this article, but can be up to 8 bits per pixel for up to 256 different colors. We will get into Color tables in a minute. 

Pixels are stored row-major, origin top-left, left to right then top to bottom, packed least-significant-bit-first within each byte. Pixel 0 is the lowest bit of `data[0]`, and so on. Here is a diagram which maps
each pixel of a character to their index in the pixmap:

[image link](https://github.com/SimplePixelFont/spf.rs/blob/main/res/articles/res/wInNumberedFramex4.png)
![](https://github.com/SimplePixelFont/spf.rs/blob/main/res/articles/res/wInNumberedFramex4.png?raw=true)

And this will result in the following character:

[image link](https://github.com/SimplePixelFont/spf.rs/blob/main/res/articles/res/wWithoutNumberedFramex4.png)
![](https://github.com/SimplePixelFont/spf.rs/blob/main/res/articles/res/wWithoutNumberedFramex4.png?raw=true)


### Adding Color

Each pixel's bits form a value that indexes into a linked [`core::ColorTable`]'s [`ColorTable::colors`]. We can add a color table to supply actual colors for our pixmaps:

```rs
let mut transparent = Color::default();
transparent.custom_alpha = Some(0); // Fully transparent.

let mut opaque = Color::default();
opaque.custom_alpha = Some(255); // Fully opaque.
opaque.red = 20;
opaque.green = 118;
opaque.blue = 192;

let mut color_table = ColorTable::default();
color_table.colors = vec![transparent, opaque];

layout.color_tables = vec![color_table];
```

Note:
- [`PixmapTableLinkFlags::LinkColorTables`] plus [`PixmapTable::color_table_indexes`] links the pixmap table to a palette the same way `CharacterTable` links to `PixmapTable`s. Our two colors mirror what `spf.rs` assumes when no `ColorTable` is linked at all: index `0` is  transparent, everything else opaque. For monochrome fonts, like our example, a Color Tale is not technically needed. However, for showcase we create one, plus now they're real colors, and nothing stops you from adding a third or fourth [`Color`]. Just remember to raise `constant_bits_per_pixel` to fit them.

With our color table, pixel values of `0` index our transparent [`Color`], and pixel values of `1` index our opaque `rgb(20, 118, 192)` [`Color`]. The renderer will use these colors as the default foreground and background colors for the text if they are not overridden.

[image link](https://github.com/SimplePixelFont/spf.rs/blob/main/res/articles/res/wRendered.png)
![](https://github.com/SimplePixelFont/spf.rs/blob/main/res/articles/res/wRendered.png?raw=true)

### Grouping characters into a font with the FontTable

A [`core::Layout`] can hold multiple [`CharacterTable`]s. A [`core::FontTable`] is what groups them into a named, authored, versioned font. Think Regular/Bold/Italic variants of the same typeface, each pointing at its own `CharacterTable`(s). Let's add one for the character table we already built:

```rs
let mut font = Font::default();
font.name = String::from("Sample Toy Font");
font.author = String::from("You");
font.version = 1;
font.font_type = FontType::Regular;
font.linked_character_table_indexes = vec![0]; // Which CharacterTables this specific font uses.

let mut font_table = FontTable::default();
font_table.link_flags = FontTableLinkFlags::LinkCharacterTables;
font_table.character_table_indexes = Some(vec![0]); // CharacterTables available to fonts in this table.
font_table.fonts = vec![font];

layout.font_tables = vec![font_table];
```

Two link arrays are in play here, at two different levels: [`FontTable::character_table_indexes`] is the collection of `CharacterTable`s available to every [`Font`] record in this table, while [`Font::linked_character_table_indexes`] is which tables from that set a *specific* font actually draws from. With one `CharacterTable` and one `Font`, both just point at index `0`. However, a file with Regular/Bold/Italic fonts that share some character tables and not others is exactly what this two-level indirection is for, along with keeping the format architecture consistent.

That's it! `Layout` now has everything: a [`Pixmap`], a [`Character`] using it, and a [`Font`] naming the table that character lives in.

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
