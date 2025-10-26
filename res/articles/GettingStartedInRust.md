**Note: This article is out of data but will be updated soon as part of the v0.7.x development.**

# Getting Started in Rust

### Synopsis
`spf.rs` brings the world of [`SimplePixelFont`](https://github.com/SimplePixelFont)(s) into the programming realm. Written in the Rust
programming language, `spf.rs` aims to be effective and simple to use, providing a native crate api for
Rust and also an FFI compatible with C-like languages and C-interopable languages. It provides a
low-level interface to the binary representation of [`SimplePixelFont`](https://github.com/SimplePixelFont) files via the [`crate::core`]
module. And includes helpful and powerful optional modules that allow integration to be faster for your
projects.

### Resources
It is important that before you begin you have a general understanding of the Rust programming
languages and that you understand at the bare-minimum how [`SimplePixelFont`](https://github.com/SimplePixelFont) files are structured.
This guide will explain the structural representation of [`SimplePixelFont`](https://github.com/SimplePixelFont) files in Rust which aims to
reflect the binary structure, so you should be able to follow along eitherways.

### The [`crate::core`] module
The most important module is the [`crate::core`] module, and it contains the lowest-level structures to
represent a [`SimplePixelFont`](https://github.com/SimplePixelFont) file. The most important struct is the [`core::Layout`] struct, which is
the binary representation of a [`SimplePixelFont`](https://github.com/SimplePixelFont) file as a Rust Structure. Lets take a look at an
example of a font [`core::Layout`] struct:

```rs
Layout {
    header: Header { //Header Properties
        configuration_flags: ConfigurationFlags {
            constant_cluster_codepoints: false,
            constant_width: false,
            constant_height: true,
        },
        modifier_flags: ModifierFlags {
            compact: true // Strips any padding bytes when converting struct to data.
        },
        configuration_values: ConfigurationValues {
            constant_cluster_codepoints: None,
            constant_width: None,
            constant_height: Some(3), // Each character in this font will have a height of 3.
        }
    },
    body {
        characters: vec![ // Includes each chatacter
            Character {
                grapheme_cluster: String::from("w"), // A valid grapheme_cluster which may be made up of multiple utf8 characters.
                custom_width: Some(5), // Since we didn't set the constant_width configuration
                //flag to true, each character must have a custom width.
                custom_height: None, // We don't need't set the custom_height because we set the
                //constant_height configuration flag to true, which means each character must have
                //a height of 4 in this case.
                pixmap: vec![ //The pixels of the character. 0 means an empty pixel.
                    1, 0, 1, 0, 1,
                    1, 0, 1, 0, 1,
                    1, 1, 1, 1, 1]
            }
        ]
    }
}
```

This is a lot to take in, luckily in Rust we don't need to write a [`core::Layout`] struct
directly, instead we turn to the [`crate::ergonomics`] module which provides the
[`ergonomics::LayoutBuilder`] struct. Keep in mind that the [`crate::ergonomics`] module is only
availible in Rust. Lets use the [`ergonomics::LayoutBuilder`] to create the same
[`core::Layout`] struct we have above:

```rs
use spf::ergonomics::*;

...

let mut font = LayoutBuilder::new()
    .constant_height(4)
    .character("w", Some(4), None, &[
        1, 0, 1, 0, 1,
        1, 0, 1, 0, 1,
        1, 1, 1, 1, 1
    ])
    .build().unwrap();
```

This is a lot more easier to read and understand, so now lets explain each method:
`.constant_height(4)` This method will set the font to have characters with the same height.
What does this mean? By specifying a constant height each character must have the same height,
and thus we no longer need to specify the [`Character::custom_height`] field. Note that 255x255 (width x height) is currently
the largest possible character within a [`SimplePixelFont`](https://github.com/SimplePixelFont) font file. Now that we have defined the header of our font,  we can add characters to our font using the [`LayoutBuilder::character()`] method as used in the sample
above.

Side Note: To learn more about the different configuration flags and modifier flags, check out the
[SPF File Specifications](https://github.com/SimplePixelFont/Specification).

### But what is a character in SimplePixelFonts?
Before we discuss how to add a character to our font, we first need to learn what a character is in the
context of a [`SimplePixelFont`](https://github.com/SimplePixelFont) font.

In simple terms a character in [`SimplePixelFont`](https://github.com/SimplePixelFont) is simply a grapheme_cluster which may be made up of multiple utf8 characters such as `a`, `<` `😊`, etc.
Optional [`Character::custom_width`] and [`Character::custom_height`] which defines the width and height of the character if the font does not have a constant width or height. And a [`Character::pixmap`] that defines what pixels
the character uses. Lets dig in more into a pixmap. A pixmap is simply a one dimentional vector
containing either 0 or 1 values (at the moment). If the value something other than 0 the character uses
the pixel, if it is 0 then the character does not. Lets take a look at an example to clarify
everything:

```rs
Character {
    grapheme_cluster: String::from("w"),
    custom_width: Some(5),
    custom_height: None,
    pixmap: vec![ //The pixels of the character. 0 means an empty pixel.
        1, 0, 1, 0, 1,
        1, 0, 1, 0, 1,
        1, 1, 1, 1, 1]
}
```

In particular the pixmap shown above can be rewritten as a vector in a single line:

```rs
vec![ 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0]
```

Now we can see that in `SimplePixelFont` [`Character::pixmap`] are defined from top-left corner pixel
and continue until the rightmost pixel before going down the next row. Here is a diagram which maps
each pixel of a character to their index in the pixmap vector:

[image link](https://github.com/SimplePixelFont/spf.rs/blob/main/res/articles/res/wInNumberedFramex4.png)
![](https://github.com/SimplePixelFont/spf.rs/blob/main/res/articles/res/wInNumberedFramex4.png?raw=true)

And this will result in the following character:

[image link](https://github.com/SimplePixelFont/spf.rs/blob/main/res/articles/res/wWithoutNumberedFramex4.png)
![](https://github.com/SimplePixelFont/spf.rs/blob/main/res/articles/res/wWithoutNumberedFramex4.png?raw=true)

### Font Example

We can define as many characters using the [`ergonomics::LayoutBuilder`] and the
[`LayoutBuilder::character()`] method. Here is an example
of a font with 3 characters and a [`ConfigurationValues::constant_height`] of 4:

```rs
let mut font = LayoutBuilder::new()
    .constant_height(4)
    .character("o", Some(4), None, &[
        1, 1, 1, 1,
        1, 0, 0, 1,
        1, 0, 0, 1,
        1, 1, 1, 1,
    ])
    .character("w", Some(5), None, &[
        1, 0, 1, 0, 1,
        1, 0, 1, 0, 1,
        1, 0, 1, 0, 1,
        1, 1, 1, 1, 1,
    ])
    .character('😊', Some(4), None, &[
        0, 1, 1, 0,
        0, 0, 0, 0,
        1, 0, 0, 1,
        0, 1, 1, 0,
    ])
    .build();
```

### Saving & Loading `spf.rs` fonts with [`std::fs`]

We can then encode the struct and use [`std::fs`] to write to a file:

```rs
let mut file = std::fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open("./sampleToyFont.spf")
    .unwrap();
file.write_all(&layout_to_data(&font).unwrap());
```

Or we can load an exsisting `.spf` file using [`std::fs`] aswell:

```rs
let mut file = std::fs::OpenOptions::new()
    .read(true)
    .open("./sampleToyFont.spf")
    .unwrap();
let mut buffer: Vec<u8> = vec![];
file.read_to_end(&mut buffer).unwrap();
let font = layout_from_data(buffer);
```
