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
