mod common;

use spf::core::*;

#[test]
fn write_font_file() -> Result<(), String> {
    let mut font = Layout {
        header: Header {
            configuration_flags: ConfigurationFlags { alignment: true },
            modifier_flags: ModifierFlags { compact: false },
            required_values: RequiredValues { constant_size: 4 },
        },
        body: Body { characters: vec![] },
    };

    #[rustfmt::skip]
    font.body.characters.push(Character {
        utf8: 'o',
        custom_size: 4,
        byte_map: vec![1, 1, 1, 1,
                       1, 0, 0, 1,
                       1, 0, 0, 1,
                       1, 1, 1, 1],
    });

    #[rustfmt::skip]
    font.body.characters.push(Character {
        utf8: 'w',
        custom_size: 5,
        byte_map: vec![1, 0, 1, 0, 1,
                       1, 0, 1, 0, 1,
                       1, 0, 1, 0, 1,
                       1, 1, 1, 1, 1],
    });

    #[rustfmt::skip]
    font.body.characters.push(Character {
        utf8: '😊',
        custom_size: 4,
        byte_map: vec![0, 1, 1, 0,
                       0, 0, 0, 0,
                       1, 0, 0, 1,
                       0, 1, 1, 0],
    });

    common::write_to_file("./res/sampleToyFont.spf", &layout_to_data(&font)).unwrap();
    Ok(())
}

#[test]
fn read_font_file() -> Result<(), String> {
    let mut buffer: Vec<u8> = vec![];
    common::read_from_file("./res/sampleToyFont.spf", &mut buffer).unwrap();

    let _font = layout_from_data(buffer);

    Ok(())
}
