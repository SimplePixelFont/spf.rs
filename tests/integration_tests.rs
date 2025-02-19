mod common;

use spf::core::*;

#[test]
fn write_font_file() -> Result<(), String> {
    let mut font = Layout::new(
        Header {
            configuration_flags: ConfigurationFlags {
                alignment: ALIGNMENT_HEIGHT,
            },
            modifier_flags: ModifierFlags { compact: false },
            required_values: RequiredValues { constant_size: 4 },
        },
        Body { characters: vec![] },
    );

    #[rustfmt::skip]
    font.add_character(Character::new(
        'o',
        4,
        vec![1, 1, 1, 1,
             1, 0, 0, 1,
             1, 0, 0, 1,
             1, 1, 1, 1],
    ))?;

    #[rustfmt::skip]
    font.add_character(Character::new(
        'w',
        5,
        vec![1, 0, 1, 0, 1,
             1, 0, 1, 0, 1,
             1, 0, 1, 0, 1,
             1, 1, 1, 1, 1],
    ))?;

    #[rustfmt::skip]
    font.add_character(Character::new(
        '😊',
        4,
        vec![0, 1, 1, 0,
             0, 0, 0, 0,
             1, 0, 0, 1,
             0, 1, 1, 0],
    ))?;

    common::write_to_file("./res/sampleToyFont.spf", &font.to_data()).unwrap();
    Ok(())
}

#[test]
fn read_font_file() -> Result<(), String> {
    let mut buffer: Vec<u8> = vec![];
    common::read_from_file("./res/sampleToyFont.spf", &mut buffer).unwrap();

    let _font = Layout::from_data(buffer);

    Ok(())
}
