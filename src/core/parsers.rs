pub(crate) use super::*;

pub(crate) fn next_signature(buffer: &byte::ByteStorage, mut current_index: usize) -> usize {
    if current_index + 3 > buffer.bytes.len() {
        panic!("Unexpected end of file");
    }

    for byte in [102, 115, 70].iter() {
        if buffer.get(current_index) != *byte {
            panic!("File is not signed");
        }
        current_index += 1;
    }

    current_index
}

pub(crate) fn next_header(storage: &byte::ByteStorage, mut current_index: usize) -> usize {
    let file_properties = storage.get(current_index);

    //         configuration_flag_booleans = [
    //             (file_properties & 0b10000000) >> 7 == 1,
    //             (file_properties & 0b01000000) >> 6 == 1,
    //             (file_properties & 0b00100000) >> 5 == 1,
    //             (file_properties & 0b00010000) >> 4 == 1,
    //         ];

    //         layout.header.modifier_flags.compact = (file_properties & 0b00001000) >> 3 == 1;
    //     }
    //     _ => {
    //         for index in current_configuration_flag_index..4 {
    //             {
    //                 // Will need to look into this later.
    //                 current_configuration_flag_index = index + 1;
    //             }
    //             if configuration_flag_booleans[index] {
    //                 configuration_flag_values[index] = Some(storage.get(current_index));
    //                 break;
    //             }
    //         }
    //         if current_configuration_flag_index == 4 {
    //             // start main body parsing logic here;
    //         }
    current_index
}

pub(crate) fn next_grapheme_cluster(
    buffer: &byte::ByteStorage,
    header: &Header,
    mut current_index: usize,
) -> (String, usize) {
    let mut grapheme_cluster = String::new();
    let mut end_cluster = false;
    let mut codepoint_count = 0;

    while !end_cluster {
        let utf81 = buffer.get(current_index);
        let mut utf8_bytes: [u8; 4] = [0, 0, 0, 0];

        if utf81 >> 7 == 0b00000000 {
            utf8_bytes[0] = utf81;
        } else if utf81 >> 5 == 0b00000110 {
            utf8_bytes[0] = utf81;
            current_index += 1;
            utf8_bytes[1] = buffer.get(current_index);
        } else if utf81 >> 4 == 0b00001110 {
            utf8_bytes[0] = utf81;
            current_index += 1;
            utf8_bytes[1] = buffer.get(current_index);
            current_index += 1;
            utf8_bytes[2] = buffer.get(current_index);
        } else if utf81 >> 3 == 0b00011110 {
            utf8_bytes[0] = utf81;
            current_index += 1;
            utf8_bytes[1] = buffer.get(current_index);
            current_index += 1;
            utf8_bytes[2] = buffer.get(current_index);
            current_index += 1;
            utf8_bytes[3] = buffer.get(current_index);
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
            if buffer.get(current_index) == 0 {
                end_cluster = true;
            }
        }
    }

    (grapheme_cluster, current_index)
}
