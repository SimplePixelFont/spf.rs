pub(crate) use super::super::byte;

/// Represents a bitmap for a character in your font.
/// Note: This is a one dimensional vector, you can use the `get_pixel()` method to get a two dimensional-like interface.
/// Note: Only the first `width * height` items are used, the rest are ignored when encoding and decoding from/to a `Vec<u8>`
#[derive(Debug, Clone)]
pub struct Bitmap {
    pub width: u8,
    pub height: u8,
    pub data: Vec<u8>,
    pub(crate) inferred: bool,
}

impl Bitmap {
    /// Creates a standard non-inferred `Bitmap` struct with all fields.
    ///
    /// This function is provided to create a `Bitmap` for characters providing all
    /// fields; width, height, and data. The `Bitmap` returned will have the inffered
    /// field set to false and can also be used within the `add_character` method of a
    /// `SimplePixelFont` struct. Keep in mind that this function requires a `Vec<u8>`
    /// for the data field instead of a `&[u8]` like the `Bitmap::inferred()` function.
    ///
    /// # Example:
    /// ```
    /// # use spf::core::Bitmap;
    /// let bitmap = Bitmap::new(4, 4, vec![
    ///     0, 0, 0, 0,
    ///     0, 1, 1, 0,
    ///     0, 1, 1, 0,
    ///     0, 0, 0, 0
    /// ]).unwrap();
    ///
    /// assert_eq!(bitmap.is_inferred(), false);
    pub fn new(width: u8, height: u8, data: Vec<u8>) -> Result<Self, String> {
        if width as usize * height as usize == data.len() {
            return Ok(Self {
                width: width,
                height: height,
                data: data,
                inferred: false,
            });
        } else {
            return Err("Bitmap width*height does not equal data.len()!".to_string());
        }
    }
    /// Creates an inferred `Bitmap` struct which dimensions are unknown.
    ///
    /// This function is provided to make creating bitmaps for character much easier.
    /// Rather then providing the width and height, this Bitmap will automatically choose
    /// the right dimensions for the character bitmap depending on the `SimplePixelFont`
    /// struct `alignment`, and `size` fields. As such it is advised to use only inferred
    /// `Bitmap`'s when you use the `unchecked_add_character` or `add_character` methods of
    /// a `SimplePixelFont`
    ///
    /// # Example
    /// ```
    /// # use spf::core::Bitmap;
    /// # use spf::core::SimplePixelFont;
    /// # use spf::core::Character;
    /// # use spf::core::ConfigurationFlags;
    /// # use spf::core::ModifierFlags;
    /// # use spf::core::ALIGNMENT_HEIGHT;
    ///
    /// let mut font = SimplePixelFont::new(
    ///     ConfigurationFlags { alignment: ALIGNMENT_HEIGHT },
    ///     ModifierFlags { compact: false },
    ///     4
    /// );
    /// font.add_character(Character::inferred('o', Bitmap::inferred(&[
    ///     0, 1, 1, 0,
    ///     1, 0, 0, 1,
    ///     1, 0, 0, 1,
    ///     0, 1, 1, 0
    /// ])));
    /// ```
    pub fn inferred(data: &[u8]) -> Self {
        Self {
            width: 0,
            height: 0,
            data: data.to_owned(),
            inferred: true,
        }
    }
    /// Returns a boolean depending if the Bitmap is inferred or not.
    ///
    /// Inferred Bitmap's can only be used when creating inferred characters.
    pub fn is_inferred(&self) -> bool {
        return self.inferred;
    }
    pub(crate) fn segment_into_u8s(&self) -> (Vec<u8>, usize) {
        let mut chunks = self.data.chunks(8);
        let mut buffer: Vec<u8> = Vec::new();
        let mut remainder = 0;

        let mut iter = chunks.next();
        while !iter.is_none() {
            let chunk = iter.unwrap();
            remainder = 8 - chunk.len();
            let mut byte = byte::Byte { bits: [false; 8] };
            let mut index: usize = 0;
            for pixel in chunk {
                byte.bits[index] = *pixel == 1; // will need to be changed later
                index += 1;
            }
            for index in 8 - remainder..8 {
                byte.bits[index] = false;
            }
            buffer.push(byte.to_u8());
            iter = chunks.next();
        }
        return (buffer, remainder);
    }
}
