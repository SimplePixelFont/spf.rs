use printer::Printer;
use spf::FormatVersion::*;
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
            data: vec![0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1],
        },
    });
    characters.push(Character {
        utf8: 'w',
        size: 5,
        bitmap: Bitmap {
            width: 5,
            height: 4,
            data: vec![1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1],
        },
    });
    characters.push(Character {
        utf8: '😊',
        size: 4,
        bitmap: Bitmap {
            width: 4,
            height: 4,
            data: vec![0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0],
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
    let font = SimplePixelFont::from_vec_u8(buffer).unwrap();
    println!("{:?}", font);
    let printer = Printer {
        font: font,
        letter_spacing: 1,
    };
    println!("{:?}", printer.new_text("ow".to_string()));
}
