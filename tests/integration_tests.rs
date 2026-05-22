#[cfg(test)]
extern crate std;

#[cfg(test)]
mod common;

#[cfg(test)]
mod tests {
    use std::io;

    use super::common;
    use spf::core::*;

    fn init_logger() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::max())
            .is_test(true)
            .try_init();
    }
    fn second_sample_pixmap_table() -> PixmapTable {
        let mut pixmap = Pixmap::default();
        pixmap.custom_width =  Some(1);
        pixmap.custom_height = None;
        pixmap.custom_bits_per_pixel = None;
        pixmap.data =  vec![0b01000010, 0b01000010, 0b01000010, 0b00001111];

        let mut pixmap_table = PixmapTable::default();
        pixmap_table.configuration_flags = PixmapTableConfigurationFlags::ConstantWidth       |
                                           PixmapTableConfigurationFlags::ConstantHeight      |
                                           PixmapTableConfigurationFlags::ConstantBitsPerPixel;
        pixmap_table.constant_width = None;
        pixmap_table.constant_height = Some(4);
        pixmap_table.constant_bits_per_pixel = Some(7);

        pixmap_table.link_flags = PixmapTableLinkFlags::LinkColorTables;
        pixmap_table.color_table_indexes = Some(vec![0]);
        pixmap_table.pixmaps = vec![pixmap];
        pixmap_table
    }

    fn sample_pixmap_table() -> PixmapTable {
        let mut pixmap1 = Pixmap::default();
        pixmap1.custom_width = Some(4);
        pixmap1.data = vec![0b10011111, 0b11111001];

        let mut pixmap2 = Pixmap::default();
        pixmap2.custom_width = Some(5);
        pixmap2.data = vec![0b10110101, 0b11010110, 0b00001111];

        let mut pixmap3 = Pixmap::default();
        pixmap3.custom_width = Some(4);
        pixmap3.data = vec![0b00000110, 0b01101001];

        let mut pixmap4 = Pixmap::default();
        pixmap4.custom_width = Some(4);
        pixmap4.data = vec![0b11110001, 0b10001111];

        let mut pixmap_table = PixmapTable::default();
        pixmap_table.constant_height = Some(4);
        pixmap_table.constant_bits_per_pixel = Some(1);
        pixmap_table.color_table_indexes = Some(vec![0]);
        pixmap_table.pixmaps = vec![pixmap1, pixmap2, pixmap3, pixmap4];
        pixmap_table
    }

    fn sample_color_table() -> ColorTable {
        let mut transparent_color = Color::default();
        transparent_color.color_type = None;
        transparent_color.custom_alpha = Some(0);
        transparent_color.r = 0;
        transparent_color.g = 0;
        transparent_color.b = 0;

        let mut opaque_color = Color::default();
        opaque_color.color_type = None;
        opaque_color.custom_alpha = Some(255);
        opaque_color.r = 36;
        opaque_color.g = 174;
        opaque_color.b = 214;

        let mut color_table = ColorTable::default();
        color_table.colors = vec![transparent_color, opaque_color];
        color_table
    }

    fn sample_font_table() -> FontTable {
        let mut font = Font::default();
        font.name = "SampleToyFont".into();
        font.author = "The-Nice-One".into();
        font.version = 0;
        font.font_type = FontType::Regular;
        font.character_table_indexes = vec![0];

        let mut font_table = FontTable::default();
        font_table.character_table_indexes = Some(vec![0]);
        font_table.fonts = vec![font];
        font_table
    }

    fn sample_layout() -> Layout {
        let mut font = Layout::default();

        let mut char1 = Character::default();
        char1.grapheme_cluster = "o".to_string();

        let mut char2 = Character::default();
        char2.grapheme_cluster = "w".to_string();

        let mut char3 = Character::default();
        char3.grapheme_cluster = "😊".to_string();

        let mut char4 = Character::default();
        char4.grapheme_cluster = "!=".to_string();

        let mut character_table = CharacterTable::default();
        character_table.pixmap_table_indexes = Some(vec![0]);
        character_table.characters = vec![char1, char2, char3, char4];

        font.character_tables = vec![character_table];
        font.pixmap_tables = vec![sample_pixmap_table(), second_sample_pixmap_table()];
        font.color_tables = vec![sample_color_table()];
        font.font_tables = vec![sample_font_table()];

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

        let mut standard_engine = DeserializeEngine::from_data(&buffer);
        deserialize_with_engine(&mut standard_engine).unwrap();

        let mut buffer_iter = buffer.iter().copied();
        let reader = byte::ByteReaderIter::from(&mut buffer_iter, buffer.len());
        let mut iterator_engine = DeserializeEngine::from_reader(reader);
        deserialize_with_engine(&mut iterator_engine).unwrap();

        //assert_eq!(standard_engine.layout, iterator_engine.layout);

        Ok(())
    }
}
