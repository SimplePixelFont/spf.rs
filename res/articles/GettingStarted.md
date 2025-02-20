# Getting Started in Rust
### Synopsis
`spf.rs` brings the world of SimplePixelFont(s) into the programming realm. Written in the Rust programming language, spf.rs aims to be effective and simple to use, providing a native crate api for Rust and also an FFI compatible with C-like languages and C-interopable languages. It provides a low-level interface to the binary representation of `.spf` files via the `core` module. And includes helpful and powerful optional modules that allow integration to be faster for your projects.
### Resources
It is important that before you begin you have a general understanding of the Rust programming languages and that you understand at the bare-minimum how .spf files are structured. This guide will explain the structural representation of .spf files in Rust, which is equivalent to the binary structure, so you should be able to follow along eitherways.
### The `spf::core` module
The most important module is the `core` module, and it contains the lowest-level structures to represent a `.spf` file. The most important struct is the `core::SimplePixelFont` struct, which is the binary representation of a `.spf` file as a Rust Structure.  It contains the File Properties (`ConfigurationFlags`,  and `ModifierFlags`) `ConstantSize`, and the `Characters` of the font.  
  
We can use the `SimplePixelFont::new()` function to create a new `SimplePixelFont` structure supplying all the header values as arguments.
```rs
use spf::core::*;
...
fn main() {
    ...
    let font = Font::new()
       .set_alignment(ALIGNMENT_HEIGHT)
       .set_compact(true)
       .set_max_flags(u8::MAX);
    let font = SimplePixelFont::new(
        ConfigurationFlags {
            0: ALIGNMENT_HEIGHT // Our characters in our font will be aligned by height, and thus will have the same height.
            ..Default::default()
        }
        ModifierFlags::default(), // No modifier flags are supported by spf.rs, we can ignore this field.
        8 // Each character in font will have a hight of 8
    );
}
```
The above code will create a `SimplePixelFont` struct with no characters defined, we can add characters with the `add_character()` method. To learn more about the different configuration flags and modifier flags, check out the [SPF File Specifications]().
#### `core::Character` and `core::Bitmap`
Before we dicuss how to add a character to our font, we first need to learn what a character is in the context of a .spf file.
  
In simple terms a character in SPF is simply a utf8 character such as `a`, `<` `ðŸª`, etc. And a bitmap that defines what pixels the character uses. Lets dig in more into a bitmap. A Bitmap is simply a one dimentional vector containing true/false values. If the value is true then the character uses the pixel, if it is false then the character does not. A bitmap also has a width and height field in order to bring the one-dimensional vector into a two-dimensional-like vector. Lets tale a look at an example to clarify everything:
```rs
use spf::core::Bitmap
...
Bitmap(
    6, // The width of the Bitmap
    4, height of the Bitmap
    &[ // Bitmap data
        0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0
    ]
);
        