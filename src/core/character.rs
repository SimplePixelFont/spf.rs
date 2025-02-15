pub(crate) use super::bitmap::Bitmap;

/// Represents a charater in the font.
#[derive(Clone, Debug)]
pub struct Character {
    pub utf8: char,
    pub size: u8,
    pub bitmap: Bitmap,
}

impl Character {
    pub fn new(utf8: char, size: u8, bitmap: Bitmap) -> Result<Self, String> {
        if !bitmap.is_inferred() {
            Ok(Self {
                utf8: utf8,
                size: size,
                bitmap: bitmap,
            })
        } else {
            Err("Bitmap provided is inferred, use Character::inferred() instead!".to_string())
        }
    }
    pub fn inferred(utf8: char, bitmap: Bitmap) -> Self {
        if bitmap.is_inferred() {
            return Self {
                utf8: utf8,
                size: 0,
                bitmap: bitmap,
            };
        }
        panic!("Not an inferred bitmap.")
    }
}
