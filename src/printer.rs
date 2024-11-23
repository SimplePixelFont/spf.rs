use super::core::Character;
use super::core::SimplePixelFont;

/// A Surface is a extended Bitmap struct with blitting and manipulation methods.
///
/// # Example
/// ```
/// // Creates a new surface with a height and width of 4.
/// // The new Surface will have 16 items of 0s in data field.
/// let surface = Surface::blank(4,4);
/// ```
#[derive(Debug)]
pub struct Surface {
    pub width: usize,
    pub height: usize,
    pub data: Vec<usize>,
}

impl Surface {
    /// Creates a new Surface with all the fields as function arguments.
    ///
    /// This function is provided for ease in creating `Surface` structs, it
    /// will use the same data types as the struct uses, with the exception
    /// of the `data` field, which must be provided as a referrenced slice.
    /// This argument will be casted into a `Vec<usize>`, and is simply to
    /// allow for more cleaner code.
    ///
    /// # Example
    /// ```
    /// let surface1 = Surface::new(2, 3, [
    ///     0, 0,
    ///     1, 1,
    ///     2, 2
    /// ]);
    ///
    /// let surface2 = Surface::new(5, 5, [
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

    /// Creates a new Surface struct with all data values equaling 0.
    ///
    /// This function is provided to allow creation of `Surface`'s without
    /// needing to define each value in the `data` field. It will simply set
    /// each of the values to 0.
    ///
    /// # Example
    /// ```
    /// let surface = Surface::blank(4,2);
    ///
    /// assert_eq!(surface.data, [
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
    /// Gets the value of (x, y) in the one-dimensional `data` field vector.
    ///
    /// This function provides a two dimensional-like interface for Surface
    /// structs for convenience. It will return the value at (x, y) with (0, 0)
    /// being the top-left corner. If the point does not exist, either `x > width`
    /// and/or `y > height` then the function will return `None`.
    ///
    /// # Example
    /// ```
    /// // Creates a new surface
    /// let surface = Surface::new(2, 2, [
    ///     0, 1,
    ///     2, 3
    /// ]);
    ///
    /// assert_eq!(surface.get_point(1,0), 1);
    /// aseert_eq!(surface.get_point(3,1), None);
    /// ```
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<usize> {
        if self.data.len() >= y * self.width + x {
            Some(self.data[y * self.width + x])
        } else {
            None
        }
    }
    /// Blits another surface to the current, at (x, y).
    ///
    /// This function will copy over all the values of another surface onto its
    /// self at the specified (x, y). If the secondary surface is larger, and/or
    /// has points that will be outside the range of the primary surface, this
    /// function will not panic and will instead ignore said points.
    ///
    /// # Example
    /// ```
    /// // Creates a new mutable surface; we will blit onto this surface.
    /// let mut surface1 = Surface::new(3, 3, [
    ///     0, 1, 1,
    ///     1, 1, 1,
    ///     1, 1, 1
    /// ]);
    ///
    /// // Creates another new surface that will be used as the secondary surface.
    /// let surface2 = Surface::new(2, 2, [
    ///     2, 2,
    ///     2, 3
    /// ]);
    ///
    /// // We blit surface2 onto surface1 starting at (1, 1).
    /// surface1.append(surface2, 1, 1);
    ///
    /// assert_eq!(surface1.data, [
    ///    0, 1, 1,
    ///    1, 2, 2,
    ///    1, 2, 3
    /// ]);
    /// ```
    pub fn append(&mut self, surface: &Surface, x: usize, y: usize) {
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
    /// Returns a Vec<T> by replacing all the indicies in the `data` with values provided.
    ///
    /// This method is provided for convience in replacing all the values of the
    /// `Surface`'s `data` field with predefined values. Please note that `T` must implement
    /// the `Copy` trait.
    ///
    /// More in depth, this method will iterate over all values of the `data` field, and
    /// use the value to determine the index of the value to use within the `values` vector.
    /// Given a surface is composed of `0` and `1`, you must supply at least two items for
    /// the `values` vector.
    ///
    /// # Example
    /// ```
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
    pub fn flatten_replace<T: Copy>(&self, values: &[Vec<T>]) -> Vec<T> {
        let mut returner: Vec<T> = vec![];
        for flag in self.data.iter() {
            for part in values[*flag].iter() {
                returner.push(*part);
            }
        }
        returner
    }
}

/// Printer is a struct for generating `Surface`'s
///
/// A `Printer` struct will hold a `SimplePixelFont` struct to decide the font to
/// use when generating a Surface. It also has a letter_spacing field to decide
/// how apart each character should be printed from.
pub struct Printer {
    pub font: SimplePixelFont,
    pub letter_spacing: usize,
}

impl Printer {
    /// Returns a `Surface` from a `String`
    ///
    /// This method will use the characters defined in the `SimplePixelFont` struct
    /// field, and place the bitmaps next to each other in a generated `Surface`
    pub fn new_text(&self, text: String) -> Surface {
        let characters: Vec<char> = text.chars().collect();
        let mut fetched_character: Vec<Character> = vec![];
        let mut width = (characters.len() - 1) * self.letter_spacing;
        characters.iter().for_each(|character| {
            let fchar = self.font.characters[self.font.cache[character]].clone();
            width += fchar.size as usize;
            fetched_character.push(fchar);
        });
        let mut surface = Surface {
            data: vec![0; self.font.size as usize * width],
            height: self.font.size as usize,
            width: width,
        };
        let mut current_x = 0;
        for character in fetched_character {
            let mut bitmap = vec![];
            character
                .bitmap
                .data
                .iter()
                .for_each(|x| bitmap.push(x.clone() as usize));
            surface.append(
                &Surface {
                    data: bitmap,
                    height: character.bitmap.height as usize,
                    width: character.bitmap.width as usize,
                },
                current_x,
                0,
            );
            current_x += self.letter_spacing + character.bitmap.width as usize;
        }
        surface
    }
}
