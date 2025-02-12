mod common;

use spf::core::*;

#[test]
fn write_font_file() -> Result<(), String> {
    let mut font = SimplePixelFont::new(
        ConfigurationFlags {
            alignment: ALIGNMENT_HEIGHT,
        },
        ModifierFlags { compact: false },
        4,
    );
    font.add_character(Character::inferred(
        'o',
        Bitmap::inferred(&[1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1]),
    ))?;
    font.add_character(Character::inferred(
        'w',
        Bitmap::inferred(&[1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1]),
    ))?;
    font.add_character(Character::inferred(
        '😊',
        Bitmap::inferred(&[0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0]),
    ))?;

    common::write_to_file("./res/sampleToyFont.spf", &font.to_vec_u8()).unwrap();
    Ok(())
}

#[test]
fn read_font_file() -> Result<(), String> {
    let mut buffer: Vec<u8> = vec![];
    common::read_from_file("./res/sampleToyFont.spf", &mut buffer).unwrap();

    let _font = SimplePixelFont::from_vec_u8(buffer);

    Ok(())
}
