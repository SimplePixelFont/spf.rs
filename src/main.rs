use spf::core::*;
use spf::printer::Printer;
use std::fs;
use std::io::{Read, Write};
use std::vec;

fn main() {
    let mut font = SimplePixelFont {
        size: 4,
        ..Default::default()
    };
    font.add_character(Character::inferred(
        'o',
        Bitmap::inferred(&[
            false, true, true, false, true, false, false, true, true, false, false, true, false,
            true, true, false,
        ]),
    ));
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
            true, true, true, true, true, false, false, true, true, false, false, true, true, true,
            true, true,
        ]),
    ));
    font.add_character(Character::inferred(
        'w',
        Bitmap::inferred(&[
            true, false, true, false, true, true, false, true, false, true, true, false, true,
            false, true, true, true, true, true, true,
        ]),
    ));
    font.add_character(Character::inferred(
        '😊',
        Bitmap::inferred(&[
            false, true, true, false, false, false, false, false, true, false, false, true, false,
            true, true, false,
        ]),
    ));

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
    let printer = Printer::from_font(font);
    println!("{:?}", printer.print("ow"));
    println!(
        "{:?}",
        printer
            .print("o")
            .flatten_replace(&[vec![0, 0, 0, 0], vec![255, 0, 0, 255]])
    );
}
