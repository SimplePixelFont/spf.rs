pub(crate) use super::*;

pub(crate) fn next_grapheme_cluster(
    body_buffer: &mut byte::ByteStorage,
    header: &Header,
    mut current_index: usize,
) -> (String, usize) {
    let mut grapheme_cluster = String::new();
    let mut end_cluster = false;
    let mut codepoint_count = 0;

    while !end_cluster {
        let utf81 = body_buffer.get(current_index);
        let mut utf8_bytes: [u8; 4] = [0, 0, 0, 0];

        if utf81 << 3 == 0b01111000 {
            utf8_bytes[0] = utf81;
            current_index += 1;
            utf8_bytes[1] = body_buffer.get(current_index);
            current_index += 1;
            utf8_bytes[2] = body_buffer.get(current_index);
            current_index += 1;
            utf8_bytes[3] = body_buffer.get(current_index);
        } else if utf81 << 4 == 0b01110000 {
            utf8_bytes[0] = utf81;
            current_index += 1;
            utf8_bytes[1] = body_buffer.get(current_index);
            current_index += 1;
            utf8_bytes[2] = body_buffer.get(current_index);
        } else if utf81 << 5 == 0b01100000 {
            utf8_bytes[0] = utf81;
            current_index += 1;
            utf8_bytes[1] = body_buffer.get(current_index);
        } else if utf81 << 7 == 0b00000000 {
            utf8_bytes[0] = utf81;
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
            if body_buffer.get(current_index) == 0 {
                end_cluster = true;
            }
        }
    }

    (grapheme_cluster, current_index)
}
