# Getting Started in Rust
### Synopsis
`spf.rs` brings the world of SimplePixelFont(s) into the programming realm. Written in the Rust programming language, spf.rs aims to be effective and simple to use, providing a native crate api for Rust and also an FFI compatible with C-like languages and C-interopable languages. It provides a low-level interface to the binary representation of `SimplePixelFont` files via the `core` module. And includes helpful and powerful optional modules that allow integration to be faster for your projects.

### Resources
It is important that before you begin you have a general understanding of the Rust programming languages and that you understand at the bare-minimum how `SimplePixelFont` files are structured. This guide will explain the structural representation of `SimplePixelFont` files in Rust which aims to reflect the binary structure, so you should be able to follow along eitherways.

### The `spf::core` module
The most important module is the `core` module, and it contains the lowest-level structures to represent a `SimplePixelFont` file. The most important struct is the `core::Layout` struct, which is the binary representation of a `SimplePixelFont` file as a Rust Structure. Lets take a look at an example of a font `Layout` struct:

```rs
Layout {
    header: Header { //Header Properties
        configuration_flags: ConfigurationFlags {
            alignment: true // Font will be alligned by Height
        },
        modifier_flags: ModifierFlags {
            compact: true // Strips any padding bytes when converting struct to data.
        },
        required_values: RequiredValues {
            constant_size: 3 // Each character in this font will have a height of 3, note how this is because the font is alligned by height.
        }
    },
    body {
        characters: vec![ // Includes each chatacter
            Character {
                utf8: 'w', // A valid utf8 character
                custom_size: 5, // Each character can have a custom size which is opppsite to the alignment constant_size. In this case 5 is the width of the character.
                byte_map: vec![ //The pixels of the character. 0 means an empty pixel.
                    1, 0, 1, 0, 1,
                    1, 0, 1, 0, 1,
                    1, 1, 1, 1, 1]
            }
        ]
    }
}
```
This is a lot to take in, luckily in Rust we dont need to write a [`layout`] struct directly, instead we turn to the [`egronomics`] module which provides the [`LayoutBuilder`] struct. Keep in mind that the [`egronomics`] module is only availible in Rust.  
Lets use the [`LayoutBuilder`] to create the same [`Layout`] struct we have above:
```rs
let mut font = LayoutBuilder::new()
    .alignment(ALIGNMENT_HEIGHT)
    .size(3)
    .inferred('w', &[
        1, 0, 1, 0, 1,
        1, 0, 1, 0, 1,
        1, 1, 1, 1, 1
    ])
    .compact(true)
    .build().unwrap();
```
This is a lot more easier to read and understand, so now lets explain each method.
```rs
.alignment(ALIGNMENT_HEIGHT)
```
This method will set the font to have characters aligned by height. What does this mean? By having the alignment set to height each character must have the same height which is determined by the following method `.size(3)` which sets the `constant_size` field of the font's `RequiredValues` struct. Note that 255x255 (width x height) is currently the largest possible character within a `SimplePixelFont` font file. Now that the [`LayoutBuilder`] has a defined `constant_size` and `alignment` we can add characters to our font using the `.chatacter()` or the `inferred()` method as used in the sample above.  
  
Side Note: To learn more about the different configuration flags and modifier flags, check out the [SPF File Specifications]().
### But what is a character in `SimplePixelFont`s?
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
```

![](./res/wInNumberedFramex4.png)

Creates a new `Layout` struct with the characters `o`, `w`, and `😊` using the `LayoutBuilder`.
```rs
use spf::egronomics::*;

fn main() {
    
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