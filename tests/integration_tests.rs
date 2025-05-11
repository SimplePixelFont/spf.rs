mod common;

use spf::core::*;

fn init_logger() {
    let _ = env_logger::builder()
        // Include all events in tests
        .filter_level(log::LevelFilter::max())
        // Ensure events are captured by `cargo test`
        .is_test(true)
        // Ignore errors initializing the logger if tests race to configure it
        .try_init();
}

#[test]
fn write_font_file() -> Result<(), String> {
    init_logger();

    let mut font = Layout::default();

    font.header.modifier_flags.compact = true;
    font.header.configuration_flags.constant_height = true;
    font.header.configuration_values.constant_height = Some(4);

    #[rustfmt::skip]
    font.body.characters.push(Character {
        grapheme_cluster: "o".to_string(),
        custom_width: Some(4),
        custom_height: None,
        pixmap: vec![1, 1, 1, 1,
                     1, 0, 0, 1,
                     1, 0, 0, 1,
                     1, 1, 1, 1],
    });

    #[rustfmt::skip]
    font.body.characters.push(Character {
        grapheme_cluster: "w".to_string(),
        custom_width: Some(5),
        custom_height: None,
        pixmap: vec![1, 0, 1, 0, 1,
                     1, 0, 1, 0, 1,
                     1, 0, 1, 0, 1,
                     1, 1, 1, 1, 1],
    });

    #[rustfmt::skip]
    font.body.characters.push(Character {
        grapheme_cluster: "😊".to_string(),
        custom_width: Some(4),
        custom_height: None,
        pixmap: vec![0, 1, 1, 0,
                     0, 0, 0, 0,
                     1, 0, 0, 1,
                     0, 1, 1, 0],
    });

    #[rustfmt::skip]
    font.body.characters.push(Character {
        grapheme_cluster: "!=".to_string(),
        custom_width: Some(4),
        custom_height: None,
        pixmap: vec![0, 0, 0, 1,
                     1, 1, 1, 1,
                     1, 1, 1, 1,
                     1, 0, 0, 0],
    });

    common::write_to_file("./res/sampleToyFont.spf", &layout_to_data(&font)).unwrap();
    panic!();
    Ok(())
}

#[test]
fn read_font_file() -> Result<(), String> {
    let mut buffer: Vec<u8> = vec![];
    common::read_from_file("./res/sampleToyFont.spf", &mut buffer).unwrap();
    buffer.iter().for_each(|a| print!("{:08b} ", a));
    println!("");
    let _font = layout_from_data(buffer);
    panic!();
    Ok(())
}
