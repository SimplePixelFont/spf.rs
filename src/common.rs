pub(crate) use super::byte;

pub(crate) fn sign_buffer(buffer: &mut byte::ByteStorage) -> &mut byte::ByteStorage {
    buffer.bytes.insert(0, byte::Byte::from_u8(70));
    buffer.bytes.insert(1, byte::Byte::from_u8(115));
    buffer.bytes.insert(2, byte::Byte::from_u8(102));
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
