A very simple parser for Simple Pixel Fonts (spf), this crate will include multiple helpful api's in the future for rendering texts as images.
Learn more about the SPF Project at https://github.com/SimplePixelFont

### Usage
creates a .spf file with the characters 'o', 'w', and '😊'
```rs
use spf::FormatVersion::*;
use spf::Pixel::*;
use spf::*;
use std::fs;
use std::io::{Read, Write};
use std::vec;

fn main() {
    let mut characters = Vec::new();
    characters.push(Character {
        utf8: 'o',
        size: 4,
        bitmap: Bitmap {
            width: 4,
            height: 4,
            data: vec![
                Filled, Filled, Filled, Filled, Filled, Empty, Empty, Filled, Filled, Empty, Empty,
                Filled, Filled, Filled, Filled, Filled,
            ],
        },
    });
    characters.push(Character {
        utf8: 'w',
        size: 5,
        bitmap: Bitmap {
            width: 5,
            height: 4,
            data: vec![
                Filled, Empty, Filled, Empty, Filled, Filled, Empty, Filled, Empty, Filled, Filled,
                Empty, Filled, Empty, Filled, Filled, Filled, Filled, Filled, Filled,
            ],
        },
    });
    characters.push(Character {
        utf8: '😊',
        size: 4,
        bitmap: Bitmap {
            width: 4,
            height: 4,
            data: vec![
                Empty, Filled, Filled, Empty, Empty, Empty, Empty, Empty, Filled, Empty, Empty,
                Filled, Empty, Filled, Filled, Empty,
            ],
        },
    });

    let font = SimplePixelFont {
        version: FV0000,
        alignment: Alignment::Height,
        size: 4,
        characters: characters,
    };

    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("./utf8ToyFont.spf")
        .unwrap();

    file.write_all(&font.to_vec_u8()).unwrap();
}
```
You can then read the file via:
```rs
    ...
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("./utf8ToyFont.spf")
        .unwrap();

    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).unwrap();
    file.read(&mut buffer).unwrap();
    let font = SimplePixelFont::from_vec_u8(buffer);
```
