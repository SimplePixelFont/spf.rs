#[cfg(test)]
extern crate std;

#[cfg(test)]
mod common;

#[cfg(test)]
mod tests {
    use std::io;

    use super::common;
    use spf::{
        core::*,
        ergonomics::{CharacterBuilder, CharacterTableBuilder, ColorTableBuilder, LayoutBuilder},
    };

    fn init_logger() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::max())
            .is_test(true)
            .try_init();
    }
    fn second_sample_pixmap_table() -> PixmapTable {
        PixmapTable {
            constant_width: None,
            constant_height: Some(4),
            constant_bits_per_pixel: Some(7),
            color_table_indexes: Some(vec![0]),
            pixmaps: vec![Pixmap {
                custom_width: Some(1),
                custom_height: None,
                custom_bits_per_pixel: None,
                data: vec![0b1000010, 0b1000010, 0b1000010, 0b1000010],
            }],
        }
    }

    fn sample_pixmap_table() -> PixmapTable {
        PixmapTable {
            constant_width: None,
            constant_height: Some(4),
            constant_bits_per_pixel: Some(1),
            color_table_indexes: Some(vec![0]),
            pixmaps: vec![
                Pixmap {
                    custom_width: Some(4),
                    custom_height: None,
                    custom_bits_per_pixel: None,
                    data: vec![1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1],
                },
                Pixmap {
                    custom_width: Some(5),
                    custom_height: None,
                    custom_bits_per_pixel: None,
                    data: vec![1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1],
                },
                Pixmap {
                    custom_width: Some(4),
                    custom_height: None,
                    custom_bits_per_pixel: None,
                    data: vec![0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0],
                },
                Pixmap {
                    custom_width: Some(4),
                    custom_height: None,
                    custom_bits_per_pixel: None,
                    data: vec![0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
                },
            ],
        }
    }

    fn sample_color_table() -> ColorTable {
        ColorTable {
            constant_alpha: None,
            colors: vec![
                Color {
                    custom_alpha: Some(0),
                    r: 0,
                    g: 0,
                    b: 0,
                },
                Color {
                    custom_alpha: Some(255),
                    r: 36,
                    g: 174,
                    b: 214,
                },
            ],
        }
    }

    fn sample_layout() -> Layout {
        let mut font = Layout::default();

        font.character_tables = vec![CharacterTable {
            use_advance_x: false,
            use_pixmap_index: false,
            constant_cluster_codepoints: None,
            pixmap_table_indexes: Some(vec![0]),
            characters: vec![
                Character {
                    advance_x: None,
                    pixmap_index: None,
                    grapheme_cluster: "o".to_string(),
                },
                Character {
                    advance_x: None,
                    pixmap_index: None,
                    grapheme_cluster: "w".to_string(),
                },
                Character {
                    advance_x: None,
                    pixmap_index: None,
                    grapheme_cluster: "😊".to_string(),
                },
                Character {
                    advance_x: None,
                    pixmap_index: None,
                    grapheme_cluster: "!=".to_string(),
                },
            ],
        }];
        font.pixmap_tables = vec![sample_pixmap_table(), second_sample_pixmap_table()];
        font.color_tables = vec![sample_color_table()];

        font.compact = true;
        font
    }

    #[test]
    fn write_font_file() -> Result<(), io::Error> {
        init_logger();

        let font = sample_layout();

        common::write_to_file("./res/sampleToyFont.spf", &layout_to_data(&font).unwrap())?;
        Ok(())
    }

    #[test]
    fn read_font_file() -> Result<(), io::Error> {
        init_logger();

        let mut buffer: Vec<u8> = vec![];
        common::read_from_file("./res/sampleToyFont.spf", &mut buffer)?;
        let _font = layout_from_data(buffer);
        Ok(())
    }

    #[test]
    fn builder_pattern() {
        init_logger();
        let mut layout = LayoutBuilder::default();
        layout.compact(true);

        let palette = ColorTableBuilder::default()
            .constant_alpha(255)
            .rgb(0, 0, 0)
            .rgb(255, 255, 255);

        let o = CharacterBuilder::from("o");
        let characters = CharacterTableBuilder::default();
    }
}
