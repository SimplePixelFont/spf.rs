//! Render texts onto a bitmap using a font [`Layout`].

use crate::cache::*;
use crate::core::*;

/// A [`Surface`] is a one dimensional bitmap with blitting and manipulation methods.
///
/// # Example
/// ```
/// # use spf::printer::Surface;
/// // Creates a new surface with a height and width of 4.
/// // The new Surface will have 16 items of 0s in data field.
/// let surface = Surface::blank(4,4);
/// ```
#[derive(Debug, Clone)]
pub struct Surface {
    pub width: usize,
    pub height: usize,
    pub data: Vec<usize>,
}

impl Surface {
    /// Creates a new [`Surface`] with all the fields as function arguments.
    ///
    /// This function is provided for ease in creating [`Surface`] structs, it
    /// will use the same data types as the struct uses, with the exception
    /// of the [`Surface::data`] field, which must be provided as a referenced slice.
    /// This argument will be casted into a [`Vec<usize>`], and is simply to
    /// allow for more cleaner code.
    ///
    /// # Example
    /// ```
    /// # use spf::printer::Surface;
    /// let surface1 = Surface::new(2, 3, &[
    ///     0, 0,
    ///     1, 1,
    ///     2, 2
    /// ]);
    ///
    /// let surface2 = Surface::new(5, 5, &[
    ///     5, 5, 5, 5, 5,
    ///     5, 5, 5, 5, 5,
    ///     5, 5, 5, 5, 5,
    ///     5, 5, 5, 5, 5,
    ///     5, 5, 5, 5, 5,
    /// ]);
    /// ```
    pub fn new(width: usize, height: usize, data: &[usize]) -> Self {
        Self {
            data: data.to_owned(),
            width: width,
            height: height,
        }
    }

    /// Creates a new [`Surface`] struct with all [`Surface::data`] values equaling 0.
    ///
    /// This function is provided to allow creation of [`Surface`]'s without
    /// needing to define each value in the [`Surface::data`] field. It will simply set
    /// each of the values to 0.
    ///
    /// # Example
    /// ```
    /// # use spf::printer::Surface;
    /// let surface = Surface::blank(4,2);
    ///
    /// assert_eq!(surface.data, &[
    ///     0, 0, 0, 0,
    ///     0, 0, 0, 0
    /// ]);
    /// ```
    pub fn blank(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            data: vec![0; width * height],
        }
    }
    /// Gets the value of (x, y) in the one-dimensional [`Surface::data`] field vector.
    ///
    /// This function provides a two dimensional-like interface for [`Surface`]
    /// structs for convenience. It will return the value at (x, y) with (0, 0)
    /// being the top-left corner. If the point does not exist, either `x > width`
    /// and/or `y > height` then the function will return `None`.
    ///
    /// # Example
    /// ```
    /// # use spf::printer::Surface;
    /// // Creates a new surface
    /// let surface = Surface::new(2, 2, &[
    ///     0, 1,
    ///     2, 3
    /// ]);
    ///
    /// assert_eq!(surface.get_pixel(1,0).unwrap(), 1);
    /// assert_eq!(surface.get_pixel(3,1).is_none(), true);
    /// ```
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<usize> {
        if self.data.len() >= y * self.width + x {
            Some(self.data[y * self.width + x])
        } else {
            None
        }
    }
    /// Blits another [`Surface`] to the current, at (x, y).
    ///
    /// This function will copy over all the values of another [`Surface`] onto its
    /// self at the specified (x, y). If the secondary [`Surface`] is larger, and/or
    /// has points that will be outside the range of the primary [`Surface`], this
    /// function will not panic and will instead ignore said points.
    ///
    /// # Example
    /// ```
    /// # use spf::printer::Surface;
    /// // Creates a new mutable surface; we will blit onto this surface.
    /// let mut surface1 = Surface::new(3, 3, &[
    ///     0, 1, 1,
    ///     1, 1, 1,
    ///     1, 1, 1
    /// ]);
    ///
    /// // Creates another new surface that will be used as the secondary surface.
    /// let surface2 = Surface::new(2, 2, &[
    ///     2, 2,
    ///     2, 3
    /// ]);
    ///
    /// // We blit surface2 onto surface1 starting at (1, 1).
    /// surface1.blit(&surface2, 1, 1);
    ///
    /// assert_eq!(surface1.data, &[
    ///    0, 1, 1,
    ///    1, 2, 2,
    ///    1, 2, 3
    /// ]);
    /// ```
    pub fn blit(&mut self, surface: &Surface, x: usize, y: usize) {
        let mut offset_x = 0;
        let mut offset_y = 0;
        let mut iter = 0;

        while iter < surface.width * surface.height {
            if !self.get_pixel(x + offset_x, y + offset_y).is_none() {
                let self_index = (y + offset_y) * self.width + x + offset_x;
                self.data[self_index] = surface.data[iter];
            }
            iter += 1;
            offset_x += 1;
            if offset_x >= surface.width {
                offset_x = 0;
                offset_y += 1;
            }
        }
    }
    /// Returns a [`Vec<T>`] by replacing all the indicies in the [`Surface::data`] with values
    /// provided.
    ///
    /// This method is provided for convience in replacing all the values of the
    /// [`Surface`]'s [`Surface::data`] field with predefined values. Please note that `T` must
    /// implement the [`Copy`] trait.
    ///
    /// More in depth, this method will iterate over all values of the [`Surface::data`] field, and
    /// use the value to determine the index of the value to use within the `values` vector.
    /// Given a surface is composed of `0` and `1`, you must supply at least two items for
    /// the `values` vector.
    ///
    /// # Example
    /// ```
    /// # use spf::printer::Surface;
    /// let surface = Surface::blank(3,1);
    ///
    /// assert_eq!(surface.replace(&['a']), vec!['a', 'a', 'a']);
    /// ```
    pub fn replace<T: Copy>(&self, values: &[T]) -> Vec<T> {
        let mut returner: Vec<T> = vec![];
        for flag in self.data.iter() {
            returner.push(values[*flag]);
        }
        returner
    }
    /// Return a [`Vec<T>`] by flattening out an array replacing each value.
    pub fn flatten_replace<T: Copy>(&self, values: &[Vec<T>]) -> Vec<T> {
        let mut returner: Vec<T> = vec![];
        for flag in self.data.iter() {
            for part in values[*flag].iter() {
                returner.push(*part);
            }
        }
        returner
    }
    /// Flips a surface
    /// # Example
    ///
    /// ```
    /// # use spf::printer::Surface;
    /// let surface = Surface::new(5, 5, &[
    ///     2, 1, 2, 3, 1,
    ///     5, 3, 1, 3, 1,
    ///     2, 1, 4, 3, 4,
    ///     4, 2, 1, 2, 5,
    ///     5, 2, 1, 3, 4
    /// ]);
    /// let flipped = surface.flip_vertical();
    /// assert_eq!(flipped.data, &[
    ///     5, 2, 1, 3, 4,
    ///     4, 2, 1, 2, 5,
    ///     2, 1, 4, 3, 4,
    ///     5, 3, 1, 3, 1,
    ///     2, 1, 2, 3, 1
    /// ]);
    /// ```
    pub fn flip_vertical(&self) -> Self {
        let mut returner = self.clone();
        let mut current_x = 0;
        let mut current_y = 0;

        while current_y < self.height / 2 {
            let first_index = (current_y * returner.width) + current_x;
            let second_index = ((self.height - 1 - current_y) * self.width) + current_x;

            returner.data[first_index] = self.data[second_index];
            returner.data[second_index] = self.data[first_index];

            current_x += 1;
            if current_x >= self.width {
                current_x = 0;
                current_y += 1;
            }
        }
        returner
    }
    /// # Example
    ///
    /// ```
    /// # use spf::printer::Surface;
    /// let surface = Surface::new(5, 5, &[
    ///     2, 1, 2, 3, 1,
    ///     5, 3, 1, 3, 1,
    ///     2, 1, 4, 3, 4,
    ///     4, 2, 1, 2, 5,
    ///     5, 2, 1, 3, 4
    /// ]);
    /// let flipped = surface.flip_horizontal();
    /// assert_eq!(flipped.data, &[
    ///     1, 3, 2, 1, 2,
    ///     1, 3, 1, 3, 5,
    ///     4, 3, 4, 1, 2,
    ///     5, 2, 1, 2, 4,
    ///     4, 3, 1, 2, 5
    /// ]);
    /// ```
    pub fn flip_horizontal(&self) -> Self {
        let mut returner = self.clone();
        let mut current_x = 0;
        let mut current_y = 0;

        while current_y < self.height {
            let first_index = current_y * self.width + current_x;
            let second_index = (current_y * self.width) + ((self.width - 1) - current_x);

            returner.data[first_index] = self.data[second_index];
            returner.data[second_index] = self.data[first_index];

            current_x += 1;
            if current_x >= self.width / 2 {
                current_x = 0;
                current_y += 1;
            }
        }

        returner
    }
}

// /// Holds the current data for the processed pixel by the Printer.
// pub struct PixelProcess {
//     pub character: Character,
//     absolute_position: (usize, usize),
//     relative_position: (usize, usize),
//     state: usize,
// }

/// Printer is a struct for generating [`Surface`]'s
///
/// A [`Printer`] struct will hold a [`Layout`] struct to decide the font to
/// use when generating a Surface. It also has a letter_spacing field to decide
/// how apart each character should be printed from.
pub struct Printer {
    pub font: Layout,
    pub character_cache: CharacterCache,
    pub letter_spacing: usize,
    //pub surface_width: Option<usize>,
    //pub surface_height: Option<usize>,
    //pub word_warp: bool,
}

impl Printer {
    pub fn from_font(font: Layout) -> Self {
        let character_cache = CharacterCache::from_characters(&font.body.characters);
        Self {
            font: font,
            character_cache: character_cache,
            letter_spacing: 1,
            // surface_width: None,
            // surface_height: None,
            // word_warp: false,
        }
    }
    /// Returns a `Surface` from a `String`
    ///
    /// This method will use the characters defined in the `SimplePixelFont` struct
    /// field, and place the bitmaps next to each other in a generated `Surface`
    pub fn print(&self, text: String) -> Surface {
        let characters: Vec<char> = text.chars().collect();
        let mut fetched_character: Vec<Character> = vec![];
        let mut width = (characters.len() - 1) * self.letter_spacing;
        characters.iter().for_each(|character| {
            let fchar = self.font.body.characters[self.character_cache.mappings[character]].clone();
            width += fchar.custom_size as usize;
            fetched_character.push(fchar);
        });
        let mut surface = Surface {
            data: vec![0; self.font.header.required_values.constant_size as usize * width],
            height: self.font.header.required_values.constant_size as usize,
            width: width,
        };
        let mut current_x = 0;
        for character in fetched_character {
            let mut bitmap = vec![];
            character
                .byte_map
                .iter()
                .for_each(|x| bitmap.push(x.clone() as usize));
            surface.blit(
                &Surface {
                    data: bitmap,
                    height: self.font.header.required_values.constant_size as usize,
                    width: character.custom_size as usize,
                },
                current_x,
                0,
            );
            current_x += self.letter_spacing + character.custom_size as usize;
        }
        surface
    }

    // pub fn pretty_print(
    //     &self,
    //     text: &'static str,
    //     processor: fn(PixelProcess) -> usize,
    // ) -> Surface {
    //     todo!()
    // }
}
