pub(crate) use super::*;

pub(crate) fn next_signature(storage: &byte::ByteStorage, mut current_index: usize) -> usize {
    if current_index + 3 > storage.bytes.len() {
        panic!("Unexpected end of file");
    }

    for byte in [102, 115, 70].iter() {
        if storage.get(current_index) != *byte {
            panic!("File is not signed");
        }
        current_index += 1;
    }

    current_index
}

pub(crate) fn next_header(
    layout: &mut Layout,
    storage: &byte::ByteStorage,
    mut current_index: usize,
) -> usize {
    let file_properties = storage.get(current_index);

    let configuration_flag_booleans = [
        (file_properties & 0b10000000) >> 7 == 1,
        (file_properties & 0b01000000) >> 6 == 1,
        (file_properties & 0b00100000) >> 5 == 1,
        (file_properties & 0b00010000) >> 4 == 1,
    ];

    layout.header.modifier_flags.compact = (file_properties & 0b00001000) >> 3 == 1;
    current_index += 1;

    let mut configuration_flag_values = [None; 4];

    for (current_configuration_flag_index, configuration_flag) in
        configuration_flag_booleans.iter().enumerate()
    {
        if *configuration_flag {
            configuration_flag_values[current_configuration_flag_index] =
                Some(storage.get(current_index));
            current_index += 1;
        }
    }

    layout
        .header
        .configuration_flags
        .constant_cluster_codepoints = configuration_flag_booleans[0];
    layout.header.configuration_flags.constant_width = configuration_flag_booleans[1];
    layout.header.configuration_flags.constant_height = configuration_flag_booleans[2];
    layout.header.configuration_flags.custom_bits_per_pixel = configuration_flag_booleans[3];

    layout
        .header
        .configuration_values
        .constant_cluster_codepoints = configuration_flag_values[0];
    layout.header.configuration_values.constant_width = configuration_flag_values[1];
    layout.header.configuration_values.constant_height = configuration_flag_values[2];
    layout.header.configuration_values.custom_bits_per_pixel = configuration_flag_values[3];

    current_index
}

pub(crate) fn next_grapheme_cluster(
    storage: &byte::ByteStorage,
    header: &Header,
    character: &mut Character,
    mut current_index: usize,
) -> usize {
    let mut grapheme_cluster = String::new();
    let mut end_cluster = false;
    let mut codepoint_count = 0;

    while !end_cluster {
        let utf81 = storage.get(current_index);
        let mut utf8_bytes: [u8; 4] = [0, 0, 0, 0];

        if utf81 >> 7 == 0b00000000 {
            utf8_bytes[0] = utf81;
        } else if utf81 >> 5 == 0b00000110 {
            utf8_bytes[0] = utf81;
            current_index += 1;
            utf8_bytes[1] = storage.get(current_index);
        } else if utf81 >> 4 == 0b00001110 {
            utf8_bytes[0] = utf81;
            current_index += 1;
            utf8_bytes[1] = storage.get(current_index);
            current_index += 1;
            utf8_bytes[2] = storage.get(current_index);
        } else if utf81 >> 3 == 0b00011110 {
            utf8_bytes[0] = utf81;
            current_index += 1;
            utf8_bytes[1] = storage.get(current_index);
            current_index += 1;
            utf8_bytes[2] = storage.get(current_index);
            current_index += 1;
            utf8_bytes[3] = storage.get(current_index);
        }

        grapheme_cluster.push(
            String::from_utf8(utf8_bytes.to_vec())
                .unwrap()
                .chars()
                .next()
                .unwrap(),
        );
        codepoint_count += 1;

        if header.configuration_flags.constant_cluster_codepoints {
            if codepoint_count
                == header
                    .configuration_values
                    .constant_cluster_codepoints
                    .unwrap()
            {
                end_cluster = true;
            }
        } else {
            current_index += 1;
            if storage.get(current_index) == 0 {
                end_cluster = true;
            }
        }
    }

    #[cfg(feature = "log")]
    info!("Identified grapheme cluster: {:?}", grapheme_cluster);

    character.grapheme_cluster = grapheme_cluster;
    current_index
}

pub(crate) fn next_width(
    storage: &byte::ByteStorage,
    header: &Header,
    character: &mut Character,
    mut current_index: usize,
) -> (u8, usize) {
    let current_character_width;
    if !header.configuration_flags.constant_width {
        character.custom_width = Some(storage.get(current_index));
        current_character_width = character.custom_width.unwrap();
        current_index += 1;

        #[cfg(feature = "log")]
        info!("Identified custom width: {:?}", current_character_width);
    } else {
        current_character_width = header.configuration_values.constant_width.unwrap();
    }

    (current_character_width, current_index)
}

pub(crate) fn next_height(
    storage: &byte::ByteStorage,
    header: &Header,
    character: &mut Character,
    mut current_index: usize,
) -> (u8, usize) {
    let current_character_height;
    if !header.configuration_flags.constant_height {
        character.custom_height = Some(storage.get(current_index));
        current_character_height = character.custom_height.unwrap();
        current_index += 1;

        #[cfg(feature = "log")]
        info!("Identified custom height: {:?}", current_character_height);
    } else {
        current_character_height = header.configuration_values.constant_height.unwrap();
    }

    (current_character_height, current_index)
}

pub(crate) fn next_pixmap(
    storage: &mut byte::ByteStorage,
    header: &Header,
    character: &mut Character,
    width: u8,
    height: u8,
    bits_per_pixel: u8,
    mut current_index: usize,
) -> usize {
    let pixels_used = width * height;
    let mut current_bit = storage.pointer;
    for _ in 0..pixels_used {
        let pixel = storage.incomplete_get(current_index, bits_per_pixel);
        character.pixmap.push(pixel);
        current_bit += bits_per_pixel;
        if current_bit >= 8 {
            current_index += 1;
            current_bit = 0;
        }
        storage.pointer = current_bit;
    }

    if !header.modifier_flags.compact && (width * height * bits_per_pixel) % 8 != 0 {
        current_index += 1;
        storage.pointer = 0;
    }

    #[cfg(feature = "log")]
    info!("Identified pixmap: {:?}", character.pixmap);

    current_index
}
