pub(crate) use super::byte;
pub(crate) use super::common;
pub(crate) use super::MAGIC_BYTES;

#[cfg(feature = "log")]
use super::log::{LogLevel, LOGGER};

pub const ALIGNMENT_WIDTH: bool = false;
pub const ALIGNMENT_HEIGHT: bool = true;

impl Layout {
    /// Adds a new character to the `SimplePixelFont` struct.
    ///
    /// This method will automatically handle both inffered and non-infferred
    /// characters and set their appropiate dimensions if possible (for inffered characters).
    /// If the method fails to add character an error will be returned and character will
    /// not be added. If `cache` feature is enabled, this method will also add the character
    /// to the `cache` HashMap field.
    pub fn add_character(&mut self, character: Character) -> Result<(), String> {
        if self.header.configuration_flags.alignment == ALIGNMENT_HEIGHT {
            self.body.characters.push(character);
            return Ok(());
        } else {
            todo!();
        }
    }
}
