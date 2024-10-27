use super::Character;
use super::SimplePixelFont;

#[derive(Debug)]
pub struct Surface {
    data: Vec<usize>,
    height: usize,
    width: usize,
}

impl Surface {
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<usize> {
        if self.data.len() >= y * self.width + x {
            Some(self.data[y * self.width + x])
        } else {
            None
        }
    }
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
    pub fn replace<T: Copy>(&self, values: &Vec<T>) -> Vec<T> {
        let mut returner: Vec<T> = vec![];
        for flag in self.data.clone() {
            returner.push(values[flag]);
        }
        returner
    }
}

pub struct Printer {
    pub font: SimplePixelFont,
    pub letter_spacing: usize,
}

impl Printer {
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
