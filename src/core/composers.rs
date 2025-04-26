pub(crate) use super::*;
pub(crate) use log::*;

pub(crate) fn push_header<'a>(
    buffer: &'a mut byte::ByteStorage,
    header: &Header,
) -> &'a mut byte::ByteStorage {
    buffer.push(byte::Byte {
        bits: [
            header.configuration_flags.constant_cluster_codepoints,
            header.configuration_flags.constant_width,
            header.configuration_flags.constant_height,
            false,
            //header.configuration_flags.custom_bits_per_pixel,
            header.modifier_flags.compact,
            false,
            false,
            false,
        ],
    });

    if header.configuration_flags.constant_cluster_codepoints {
        buffer.push(byte::Byte::from_u8(
            header
                .configuration_values
                .constant_cluster_codepoints
                .unwrap(),
        ));
    }
    if header.configuration_flags.constant_width {
        buffer.push(byte::Byte::from_u8(
            header.configuration_values.constant_width.unwrap(),
        ));
    }
    if header.configuration_flags.constant_height {
        buffer.push(byte::Byte::from_u8(
            header.configuration_values.constant_height.unwrap(),
        ));
    }
    // if header.configuration_flags.custom_bits_per_pixel {
    //     buffer.push(byte::Byte::from_u8(
    //         header.configuration_values.custom_bits_per_pixel.unwrap(),
    //     ));
    // }

    buffer
}

pub(crate) fn push_grapheme_cluster<'a>(
    buffer: &'a mut byte::ByteStorage,
    header: &Header,
    string: &String,
) -> &'a mut byte::ByteStorage {
    let mut string_bit_string = String::new(); // part of log

    string.bytes().for_each(|byte| {
        buffer.push(byte::Byte::from_u8(byte));
        string_bit_string.push_str(&format!("{:08b} ", byte)); // part of log
    });

    if !header.configuration_flags.constant_cluster_codepoints {
        buffer.push(byte::Byte::from_u8(0));
        string_bit_string.push_str(&format!("{:08b} ", 0)); // part of log
    }

    #[cfg(feature = "log")]
    info!(
        "Pushed grapheme cluster '{}' with the following bits: {}",
        string, string_bit_string
    );

    buffer
}

pub(crate) fn push_width<'a>(
    buffer: &'a mut byte::ByteStorage,
    header: &Header,
    custom_width: Option<u8>,
) -> &'a mut byte::ByteStorage {
    if !header.configuration_flags.constant_width {
        let width = custom_width.unwrap();
        buffer.push(byte::Byte::from_u8(width));

        let width_bit_string = format!("{:08b}", width);

        #[cfg(feature = "log")]
        info!(
            "Pushed character width '{}' with the following bits: {}",
            width, width_bit_string
        )
    }

    buffer
}

pub(crate) fn push_height<'a>(
    buffer: &'a mut byte::ByteStorage,
    header: &Header,
    custom_height: Option<u8>,
) -> &'a mut byte::ByteStorage {
    if !header.configuration_flags.constant_height {
        let height = custom_height.unwrap();
        buffer.push(byte::Byte::from_u8(height));

        let height_bit_string = format!("{:08b}", height);

        #[cfg(feature = "log")]
        info!(
            "Pushed character height '{}' with the following bits: {}",
            height, height_bit_string
        )
    }

    buffer
}

pub(crate) fn push_pixmap(
    buffer: &mut byte::ByteStorage,
    header: &Header,
    character_bytes: Vec<u8>,
    remaining_space: usize,
) {
    let mut pixmap_bit_string = String::new();

    let used_bytes = character_bytes.len();

    for (index, byte) in character_bytes.iter().enumerate() {
        pixmap_bit_string.push_str(&format!("{:08b} ", byte));

        if header.modifier_flags.compact && index == used_bytes - 1 {
            buffer.incomplete_push(byte::Byte::from_u8(*byte), remaining_space);
        } else {
            buffer.push(byte::Byte::from_u8(*byte));
        }
    }

    #[cfg(feature = "log")]
    info!(
        "Pushed byte map with the following bits: {}",
        pixmap_bit_string
    );
}
