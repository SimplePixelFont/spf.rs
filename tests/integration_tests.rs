use spf::core::*;
use std::io::{Read, Write};

#[test]
fn create_font_file() -> Result<(), String> {
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
    ))?;
    font.add_character(Character::inferred(
        'w',
        Bitmap::inferred(&[
            true, false, true, false, true, true, false, true, false, true, true, false, true,
            false, true, true, true, true, true, true,
        ]),
    ))?;
    font.add_character(Character::inferred(
        '😊',
        Bitmap::inferred(&[
            false, true, true, false, false, false, false, false, true, false, false, true, false,
            true, true, false,
        ]),
    ))?;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("./res/sampleToyFont.spf")
        .unwrap();
    file.write_all(&font.to_vec_u8()).unwrap();

    Ok(())
}

#[test]
fn read_font_file() -> Result<(), String> {
    let mut buffer: Vec<u8> = vec![];
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open("./res/sampleToyFont.spf")
        .unwrap();
    file.read_to_end(&mut buffer).unwrap();

    let _font = SimplePixelFont::from_vec_u8(buffer);

    Ok(())
}
