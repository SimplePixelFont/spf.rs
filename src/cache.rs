/*
 * Copyright 2025 SimplePixelFont
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Caching structs used by the [`crate::printer`] module.

use super::core::*;
use crate::{HashMap, String};

/// A `CharacterCache` struct is used to store mappings between the utf8 characters and their index
/// from within a [`Body::characters`] field.
pub struct CharacterCache {
    pub mappings: HashMap<String, usize>,
}

impl CharacterCache {
    /// Creates a new `CharacterCache` struct with no mappings.
    ///
    /// This method will create a new [`CharacterCache`] struct with the mappings
    /// field set to an empty initialized [`std::collections::HashMap`].
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
            mappings: HashMap::new(),
        }
    }
    /// Creates a new [`CharacterCache`] struct by mapping all characters in a [`Vec<Character>`].
    ///
    /// This method will create a new [`CharacterCache`] struct with the mappings
    /// field set to a [`std::collections::HashMap`] with all the [`Character::grapheme_cluster`]
    /// fields as keys and the index in the [`Vec<Character>`] as values.
    ///
    /// # Example
    /// ```
    /// # use spf::cache::CharacterCache;
    /// # use spf::core::Character;
    ///
    /// let characters = vec![
    ///     Character {
    ///         grapheme_cluster: "o".to_string(),
    ///         custom_width: Some(4),
    ///         custom_height: Some(4),
    ///         pixmap: vec![0, 1, 1, 0,
    ///                        1, 0, 0, 1,
    ///                        1, 0, 0, 1,
    ///                        0, 1, 1, 0],
    ///     },
    ///     Character {
    ///        grapheme_cluster: "u".to_string(),
    ///        custom_width: Some(4),
    ///        custom_height: Some(4),
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
    /// assert_eq!(cache.mappings.get(&"u".to_string()), Some(&1));
    /// ```
    pub fn from_characters(characters: &[Character]) -> Self {
        let mut mapping: HashMap<String, usize> = HashMap::new();
        for (index, character) in characters.iter().enumerate() {
            mapping.insert(character.grapheme_cluster.clone(), index);
        }
        Self { mappings: mapping }
    }
}
