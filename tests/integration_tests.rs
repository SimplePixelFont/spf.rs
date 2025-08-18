// #[cfg(test)]
// extern crate std;

// #[cfg(test)]
// mod common;

// #[cfg(test)]
// mod tests {
//     use std::io;

//     use super::common;
//     use spf::{core::*, printer::*};

//     fn init_logger() {
//         let _ = env_logger::builder()
//             .filter_level(log::LevelFilter::max())
//             .is_test(true)
//             .try_init();
//     }

//     fn sample_layout() -> Layout {
//         let mut font = Layout::default();

//         #[rustfmt::skip]
//         font.body.characters.push(Character {
//             grapheme_cluster: "o".to_string(),
//             custom_width: Some(4),
//             custom_height: None,
//             pixmap: vec![1, 1, 1, 1,
//                         1, 0, 0, 1,
//                         1, 0, 0, 1,
//                         1, 1, 1, 1],
//         });

//         #[rustfmt::skip]
//         font.body.characters.push(Character {
//             grapheme_cluster: "w".to_string(),
//             custom_width: Some(5),
//             custom_height: None,
//             pixmap: vec![1, 0, 1, 0, 1,
//                         1, 0, 1, 0, 1,
//                         1, 0, 1, 0, 1,
//                         1, 1, 1, 1, 1],
//         });

//         #[rustfmt::skip]
//         font.body.characters.push(Character {
//             grapheme_cluster: "😊".to_string(),
//             custom_width: Some(4),
//             custom_height: None,
//             pixmap: vec![0, 1, 1, 0,
//                         0, 0, 0, 0,
//                         1, 0, 0, 1,
//                         0, 1, 1, 0],
//         });

//         #[rustfmt::skip]
//         font.body.characters.push(Character {
//             grapheme_cluster: "!=".to_string(),
//             custom_width: Some(4),
//             custom_height: None,
//             pixmap: vec![0, 0, 0, 1,
//                         1, 1, 1, 1,
//                         1, 1, 1, 1,
//                         1, 0, 0, 0],
//         });

//         font.header.modifier_flags.compact = true;
//         font.header.configuration_flags.constant_height = true;
//         font.header.configuration_values.constant_height = Some(4);

//         font
//     }

//     #[test]
//     fn write_font_file() -> Result<(), io::Error> {
//         init_logger();

//         let font = sample_layout();

//         common::write_to_file("./res/sampleToyFont.spf", &layout_to_data(&font))?;
//         Ok(())
//     }

//     #[test]
//     fn read_font_file() -> Result<(), io::Error> {
//         let mut buffer: Vec<u8> = vec![];
//         common::read_from_file("./res/sampleToyFont.spf", &mut buffer)?;
//         let _font = layout_from_data(buffer);
//         Ok(())
//     }

//     #[test]
//     fn print_string() -> Result<(), ()> {
//         let printer = Printer::from_font(sample_layout());
//         let text = printer.print("wow".to_string());

//         #[rustfmt::skip]
//         assert_eq!(text.data, vec![
//             1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1,
//             1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1,
//             1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1,
//             1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1,
//         ]);

//         Ok(())
//     }
// }
