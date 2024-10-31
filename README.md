A very simple and concrete parser for `*.spf` (SimplePixelFont) files for Rust. spf.rs provides
simple encoding and decoding for `*.spf` binary representation through a `Vec<u8>`. And also
includes optional features to conveiniently create a texture from a font rendering, which
can then be used in your favorite game engine / graphics framework.

### Example
Creates a new `SimplePixelFont` struct and adds the characters `o`, `w`, and `😊`.
```rs
use spf::core::*;

fn main() {
    let mut font = SimplePixelFont::new(FV0000, Alignment::Height, 4);
    font.add_character(Character::inferred(
        'o',
        Bitmap::inferred(&[
            true, true, true, true,
            true, false, false, true,
            true, false, false, true,
            true, true, true, true,
        ]),
    ));
    font.add_character(Character::inferred(
        'w',
        Bitmap::inferred(&[
            true, false, true, false, true,
            true, false, true, false, true,
            true, false, true, false, true,
            true, true, true, true, true,
        ]),
    ));
    font.add_character(Character::inferred(
        '😊',
        Bitmap::inferred(&[
            false, true, true, false,
            false, false, false, false,
            true, false, false, true,
            false, true, true, false,
        ]),
    ));
}
```
We can then encode the struct and use `std::fs` to write to a file:
```rs
let mut file = std::fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open("./utf8ToyFont.spf")
    .unwrap();
file.write_all(&font.to_vec_u8()).unwrap();
```
Or we can load an exsisting .spf file using `std::fs` aswell:
```rs
let mut file = fs::OpenOptions::new()
    .read(true)
    .create(true)
    .open("./utf8ToyFont.spf")
    .unwrap();
let mut buffer: Vec<u8> = vec![];
file.read(&mut buffer).unwrap();
let font = SimplePixelFont::from_vec_u8(buffer);
```
### Support Format Versions
| Format Version | Stability |
| --- | --- |
| `FV0000` (Vanilla) | ✔ |
