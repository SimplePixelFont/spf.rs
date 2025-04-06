//! Caching structs used by the [`crate::printer`] module.

pub(crate) use super::core::*;

/// A `CharacterCache` struct is used to store mappings between the utf8 characters and their index
/// from within a [`Layout::body::characters`] field.
pub struct CharacterCache {
    pub mappings: std::collections::HashMap<char, usize>,
}

impl CharacterCache {
    /// Creates a new `CharacterCache` struct with no mappings.
    ///
    /// This method will create a new `CharacterCache` struct with the mappings
    /// field set to an empty initialized `HashMap`.
    ///
    /// # Example
    /// ```
    /// # use spf::cache::CharacterCache;
    /// let cache = CharacterCache::empty();
    ///
    /// // We check that the character_mappings field has 0 keys.
    /// assert_eq!(cache.mappings.len(), 0);
    /// ```
    pub fn empty() -> Self {
        Self {
            mappings: std::collections::HashMap::new(),
        }
    }
    /// Creates a new [`CharacaterCache`] struct by mapping all characters in a [`Vec<Character>`].
    ///
    /// This method will create a new [`CharacterCache`] struct with the mappings
    /// field set to a [`std::colections::HashMap`] with all the utf8 Character fields as keys and the
    /// index in the [`Vec<Character>`] as values.
    ///
    /// # Example
    /// ```
    /// # use spf::cache::CharacterCache;
    /// # use spf::core::Character;
    ///
    /// let characters = vec![
    ///     Character {
    ///         utf8: 'o',
    ///         custom_size: 4,
    ///         pixmap: vec![0, 1, 1, 0,
    ///                        1, 0, 0, 1,
    ///                        1, 0, 0, 1,
    ///                        0, 1, 1, 0],
    ///     },
    ///     Character {
    ///        utf8: 'u',
    ///        custom_size: 4,
    ///        pixmap: vec![1, 0, 0, 1,
    ///                       1, 0, 0, 1,
    ///                       1, 0, 0, 1,
    ///                       1, 1, 1, 1],
    ///     },
    /// ];
    /// let cache = CharacterCache::from_characters(&characters);
    ///
    /// // We check that the character_mappings field has 2 keys.
    /// assert_eq!(cache.mappings.len(), 2);
    ///
    /// // We can retrieve the index of the 'u' character from the cache.
    /// assert_eq!(cache.mappings.get(&'u'), Some(&1));
    /// ```
    pub fn from_characters(characters: &Vec<Character>) -> Self {
        let mut mapping: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
        for (index, character) in characters.iter().enumerate() {
            mapping.insert(character.utf8, index);
        }
        Self { mappings: mapping }
    }
}
