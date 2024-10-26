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

    let font = SimplePixelFont::new(FV0000, Alignment::Height, 4, characters);

    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("./letterO.spf")
        .unwrap();
    //file.write_all(&font.to_vec_u8()).unwrap();

    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).unwrap();
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
    let font = SimplePixelFont::from_vec_u8(buffer);
    println!("{:?}", font);
}
