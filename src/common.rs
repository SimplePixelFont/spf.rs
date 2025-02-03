pub(crate) use super::byte;

pub(crate) fn sign_buffer(buffer: &mut byte::ByteStorage) -> &mut byte::ByteStorage {
    buffer.bytes.insert(0, byte::Byte::from_u8(70));
    buffer.bytes.insert(0, byte::Byte::from_u8(115));
    buffer.bytes.insert(0, byte::Byte::from_u8(102));
    buffer

    // stdout
    //     .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
    //     .unwrap();
    // write!(&mut stdout, "[ Info: ");
    // stdout.reset().unwrap();
    // writeln!(&mut stdout, "Signed font data vector.");

    // let string = " ok ok";

    // let string2: Vec<&str> = string.split("\n").collect();
    // println!("{:?}", string2);
}

// HeaderProperties?
SimplePixelFont {
    Header {
        flags: [8, u8],
        Properties {
            constantSize: u8,
        }
    },
    Body {
        characters: Vec<Character {
            utf8: char,
            customSize: u8,
            data: Vec<u8>
        }>
    }
}

pub(crate) fn push_header(buffer: &mut byte::ByteStorage, flags: [8: bool]) -> byte::ByteStorage {}