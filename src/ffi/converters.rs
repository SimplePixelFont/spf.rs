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

#[cfg(feature = "std")]
use std::ffi::*;

#[cfg(feature = "std")]
use std::str::Utf8Error;

#[cfg(not(feature = "std"))]
use core::str::Utf8Error;

#[cfg(not(feature = "std"))]
use alloc::ffi::*;

use super::*;

#[derive(Debug, Clone)]
pub enum ConversionError {
    NulError(NulError),
    Utf8Error(Utf8Error),
}

impl From<NulError> for ConversionError {
    fn from(err: NulError) -> Self {
        ConversionError::NulError(err)
    }
}

impl From<Utf8Error> for ConversionError {
    fn from(err: Utf8Error) -> Self {
        ConversionError::Utf8Error(err)
    }
}

impl TryFrom<Character> for SPFCharacter {
    type Error = ConversionError;

    fn try_from(character: Character) -> Result<Self, Self::Error> {
        let grapheme_cluster = CString::new(character.grapheme_cluster.as_str())?;
        let grapheme_cluster_ptr = grapheme_cluster.into_raw();

        Ok(SPFCharacter {
            has_advance_x: character.advance_x.is_some() as c_uchar,
            advance_x: character.advance_x.unwrap_or(0) as c_uchar,
            has_pixmap_index: character.pixmap_index.is_some() as c_uchar,
            pixmap_index: character.pixmap_index.unwrap_or(0) as c_uchar,
            grapheme_cluster: grapheme_cluster_ptr,
        })
    }
}

impl TryInto<Character> for &SPFCharacter {
    type Error = ConversionError;

    fn try_into(self) -> Result<Character, Self::Error> {
        unsafe {
            let grapheme_cluster = CString::from_raw(self.grapheme_cluster)
                .to_str()?
                .to_owned();
            let advance_x = if self.has_advance_x == 0 {
                None
            } else {
                Some(self.advance_x as u8)
            };
            let pixmap_index = if self.has_pixmap_index == 0 {
                None
            } else {
                Some(self.pixmap_index as u8)
            };

            Ok(Character {
                advance_x,
                pixmap_index,
                grapheme_cluster,
            })
        }
    }
}

impl TryFrom<Pixmap> for SPFPixmap {
    type Error = ConversionError;

    fn try_from(pixmap: Pixmap) -> Result<Self, Self::Error> {
        let pixmap_len = pixmap.data.len();
        let pixmap_ptr = if pixmap_len == 0 {
            core::ptr::null_mut()
        } else {
            let mut pixmap_vec = pixmap.data.into_boxed_slice();
            let ptr = pixmap_vec.as_mut_ptr();
            core::mem::forget(pixmap_vec);
            ptr
        };
        Ok(SPFPixmap {
            has_custom_width: pixmap.custom_width.is_some() as c_uchar,
            custom_width: pixmap.custom_width.unwrap_or(0) as c_uchar,
            has_custom_height: pixmap.custom_height.is_some() as c_uchar,
            custom_height: pixmap.custom_height.unwrap_or(0) as c_uchar,
            has_custom_bits_per_pixel: pixmap.custom_bits_per_pixel.is_some() as c_uchar,
            custom_bits_per_pixel: pixmap.custom_bits_per_pixel.unwrap_or(0) as c_uchar,
            data: pixmap_ptr,
            data_length: pixmap_len as c_ulong,
        })
    }
}

impl TryInto<Pixmap> for &SPFPixmap {
    type Error = ConversionError;

    fn try_into(self) -> Result<Pixmap, Self::Error> {
        unsafe {
            let data = slice::from_raw_parts(self.data, self.data_length as usize).to_vec();
            let custom_width = if self.has_custom_width == 0 {
                None
            } else {
                Some(self.custom_width as u8)
            };
            let custom_height = if self.has_custom_height == 0 {
                None
            } else {
                Some(self.custom_height as u8)
            };
            let custom_bits_per_pixel = if self.has_custom_bits_per_pixel == 0 {
                None
            } else {
                Some(self.custom_bits_per_pixel as u8)
            };

            Ok(Pixmap {
                custom_width,
                custom_height,
                custom_bits_per_pixel,
                data,
            })
        }
    }
}

impl TryFrom<Color> for SPFColor {
    type Error = ConversionError;

    fn try_from(color: Color) -> Result<Self, Self::Error> {
        Ok(SPFColor {
            has_custom_alpha: color.custom_alpha.is_some() as c_uchar,
            custom_alpha: color.custom_alpha.unwrap_or(0) as c_uchar,
            r: color.r as c_uchar,
            g: color.g as c_uchar,
            b: color.b as c_uchar,
        })
    }
}

impl TryInto<Color> for &SPFColor {
    type Error = ConversionError;

    fn try_into(self) -> Result<Color, Self::Error> {
        Ok(Color {
            custom_alpha: if self.has_custom_alpha == 0 {
                None
            } else {
                Some(self.custom_alpha as u8)
            },
            r: self.r as u8,
            g: self.g as u8,
            b: self.b as u8,
        })
    }
}

impl TryFrom<PixmapTable> for SPFPixmapTable {
    type Error = ConversionError;

    fn try_from(table: PixmapTable) -> Result<Self, Self::Error> {
        let color_table_indexes_len = if let Some(color_table_indexes) = &table.color_table_indexes
        {
            color_table_indexes.len()
        } else {
            0
        };
        let color_table_indexes_ptr = if color_table_indexes_len == 0 {
            core::ptr::null_mut()
        } else {
            let mut color_table_indexes_vec = table
                .color_table_indexes
                .clone()
                .unwrap()
                .into_boxed_slice();
            let ptr = color_table_indexes_vec.as_mut_ptr();
            core::mem::forget(color_table_indexes_vec);
            ptr
        };

        let pixmaps_len: usize = table.pixmaps.len();
        let mut pixmaps: Vec<SPFPixmap> = Vec::with_capacity(pixmaps_len);

        for pixmap in table.pixmaps {
            pixmaps.push(pixmap.try_into()?);
        }

        let pixmaps_ptr = if pixmaps_len == 0 {
            core::ptr::null_mut()
        } else {
            let mut pixmaps_raw = pixmaps.into_boxed_slice();
            let ptr = pixmaps_raw.as_mut_ptr();
            core::mem::forget(pixmaps_raw);
            ptr
        };

        Ok(SPFPixmapTable {
            has_constant_width: table.constant_width.is_some() as c_uchar,
            constant_width: table.constant_width.unwrap_or(0) as c_uchar,
            has_constant_height: table.constant_height.is_some() as c_uchar,
            constant_height: table.constant_height.unwrap_or(0) as c_uchar,
            has_constant_bits_per_pixel: table.constant_bits_per_pixel.is_some() as c_uchar,
            constant_bits_per_pixel: table.constant_bits_per_pixel.unwrap_or(0) as c_uchar,
            has_color_table_indexes: table.color_table_indexes.is_some() as c_uchar,
            color_table_indexes: color_table_indexes_ptr,
            color_table_indexes_length: color_table_indexes_len as c_ulong,
            pixmaps: pixmaps_ptr,
            pixmaps_length: pixmaps_len as c_ulong,
        })
    }
}

impl TryInto<PixmapTable> for &SPFPixmapTable {
    type Error = ConversionError;

    fn try_into(self) -> Result<PixmapTable, Self::Error> {
        unsafe {
            let color_table_indexes = slice::from_raw_parts(
                self.color_table_indexes,
                self.color_table_indexes_length as usize,
            )
            .to_vec();

            let pixmaps_len = self.pixmaps_length as usize;
            let mut pixmaps = Vec::with_capacity(pixmaps_len);
            for index in 0..pixmaps_len {
                let pixmap = &*self.pixmaps.add(index);
                pixmaps.push(pixmap.try_into()?);
            }

            Ok(PixmapTable {
                constant_width: if self.has_constant_width == 0 {
                    None
                } else {
                    Some(self.constant_width as u8)
                },
                constant_height: if self.has_constant_height == 0 {
                    None
                } else {
                    Some(self.constant_height as u8)
                },
                constant_bits_per_pixel: if self.has_constant_bits_per_pixel == 0 {
                    None
                } else {
                    Some(self.constant_bits_per_pixel as u8)
                },
                color_table_indexes: if self.has_color_table_indexes == 0 {
                    None
                } else {
                    Some(color_table_indexes)
                },
                pixmaps,
            })
        }
    }
}

impl TryFrom<CharacterTable> for SPFCharacterTable {
    type Error = ConversionError;

    fn try_from(table: CharacterTable) -> Result<Self, Self::Error> {
        let pixmap_table_indexes_len =
            if let Some(pixmap_table_indexes) = &table.pixmap_table_indexes {
                pixmap_table_indexes.len()
            } else {
                0
            };
        let pixmap_table_indexes_ptr = if pixmap_table_indexes_len == 0 {
            core::ptr::null_mut()
        } else {
            let mut pixmap_table_indexes_vec = table
                .pixmap_table_indexes
                .clone()
                .unwrap()
                .into_boxed_slice();
            let ptr = pixmap_table_indexes_vec.as_mut_ptr();
            core::mem::forget(pixmap_table_indexes_vec);
            ptr
        };

        let characters_len: usize = table.characters.len();
        let mut characters: Vec<SPFCharacter> = Vec::with_capacity(characters_len);

        for character in table.characters {
            characters.push(character.try_into()?);
        }

        let characters_ptr = if characters_len == 0 {
            core::ptr::null_mut()
        } else {
            let mut characters_raw = characters.into_boxed_slice();
            let ptr = characters_raw.as_mut_ptr();
            core::mem::forget(characters_raw);
            ptr
        };

        Ok(SPFCharacterTable {
            use_advance_x: table.use_advance_x as c_uchar,
            use_pixmap_index: table.use_pixmap_index as c_uchar,
            has_constant_cluster_codepoints: table.constant_cluster_codepoints.is_some() as c_uchar,
            constant_cluster_codepoints: table.constant_cluster_codepoints.unwrap_or(0) as c_uchar,
            has_pixmap_table_indexes: table.pixmap_table_indexes.is_some() as c_uchar,
            pixmap_table_indexes: pixmap_table_indexes_ptr,
            pixmap_table_indexes_length: pixmap_table_indexes_len as c_ulong,
            characters: characters_ptr,
            characters_length: characters_len as c_ulong,
        })
    }
}

impl TryInto<CharacterTable> for &SPFCharacterTable {
    type Error = ConversionError;

    fn try_into(self) -> Result<CharacterTable, Self::Error> {
        unsafe {
            let pixmap_table_indexes = slice::from_raw_parts(
                self.pixmap_table_indexes,
                self.pixmap_table_indexes_length as usize,
            )
            .to_vec();

            let characters_len = self.characters_length as usize;
            let mut characters = Vec::with_capacity(characters_len);
            for index in 0..characters_len {
                let character = &*self.characters.add(index);
                characters.push(character.try_into()?);
            }

            Ok(CharacterTable {
                use_advance_x: self.use_advance_x != 0,
                use_pixmap_index: self.use_pixmap_index != 0,
                constant_cluster_codepoints: if self.has_constant_cluster_codepoints == 0 {
                    None
                } else {
                    Some(self.constant_cluster_codepoints as u8)
                },
                pixmap_table_indexes: if self.has_pixmap_table_indexes == 0 {
                    None
                } else {
                    Some(pixmap_table_indexes)
                },
                characters,
            })
        }
    }
}

impl TryFrom<ColorTable> for SPFColorTable {
    type Error = ConversionError;

    fn try_from(table: ColorTable) -> Result<Self, Self::Error> {
        let colors_len: usize = table.colors.len();
        let mut colors: Vec<SPFColor> = Vec::with_capacity(colors_len);

        for color in table.colors {
            colors.push(color.try_into()?);
        }

        let colors_ptr = if colors_len == 0 {
            core::ptr::null_mut()
        } else {
            let mut colors_raw = colors.into_boxed_slice();
            let ptr = colors_raw.as_mut_ptr();
            core::mem::forget(colors_raw);
            ptr
        };

        Ok(SPFColorTable {
            has_constant_alpha: table.constant_alpha.is_some() as c_uchar,
            constant_alpha: table.constant_alpha.unwrap_or(0) as c_uchar,
            colors: colors_ptr,
            colors_length: colors_len as c_ulong,
        })
    }
}

impl TryInto<ColorTable> for &SPFColorTable {
    type Error = ConversionError;

    fn try_into(self) -> Result<ColorTable, Self::Error> {
        unsafe {
            let colors_len = self.colors_length as usize;
            let mut colors = Vec::with_capacity(colors_len);
            for index in 0..colors_len {
                let color = &*self.colors.add(index);
                colors.push(color.try_into()?);
            }

            Ok(ColorTable {
                constant_alpha: if self.has_constant_alpha == 0 {
                    None
                } else {
                    Some(self.constant_alpha as u8)
                },
                colors,
            })
        }
    }
}

impl TryFrom<Layout> for SPFLayout {
    type Error = ConversionError;

    fn try_from(layout: Layout) -> Result<Self, Self::Error> {
        let character_tables_len: usize = layout.character_tables.len();
        let mut character_tables: Vec<SPFCharacterTable> = Vec::with_capacity(character_tables_len);

        for character_table in layout.character_tables {
            character_tables.push(character_table.try_into()?);
        }

        let character_tables_ptr = if character_tables_len == 0 {
            core::ptr::null_mut()
        } else {
            let mut character_tables_raw = character_tables.into_boxed_slice();
            let ptr = character_tables_raw.as_mut_ptr();
            core::mem::forget(character_tables_raw);
            ptr
        };

        let color_tables_len: usize = layout.color_tables.len();
        let mut color_tables: Vec<SPFColorTable> = Vec::with_capacity(color_tables_len);

        for color_table in layout.color_tables {
            color_tables.push(color_table.try_into()?);
        }

        let color_tables_ptr = if color_tables_len == 0 {
            core::ptr::null_mut()
        } else {
            let mut color_tables_raw = color_tables.into_boxed_slice();
            let ptr = color_tables_raw.as_mut_ptr();
            core::mem::forget(color_tables_raw);
            ptr
        };

        let pixmap_tables_len: usize = layout.pixmap_tables.len();
        let mut pixmap_tables: Vec<SPFPixmapTable> = Vec::with_capacity(pixmap_tables_len);

        for pixmap_table in layout.pixmap_tables {
            pixmap_tables.push(pixmap_table.try_into()?);
        }

        let pixmap_tables_ptr = if pixmap_tables_len == 0 {
            core::ptr::null_mut()
        } else {
            let mut pixmap_tables_raw = pixmap_tables.into_boxed_slice();
            let ptr = pixmap_tables_raw.as_mut_ptr();
            core::mem::forget(pixmap_tables_raw);
            ptr
        };

        Ok(SPFLayout {
            version: layout.version as c_uchar,
            compact: layout.compact as c_uchar,
            character_tables: character_tables_ptr,
            character_tables_length: character_tables_len as c_ulong,
            color_tables: color_tables_ptr,
            color_tables_length: color_tables_len as c_ulong,
            pixmap_tables: pixmap_tables_ptr,
            pixmap_tables_length: pixmap_tables_len as c_ulong,
        })
    }
}

impl TryInto<Layout> for SPFLayout {
    type Error = ConversionError;

    fn try_into(self) -> Result<Layout, Self::Error> {
        unsafe {
            let character_tables_len = self.character_tables_length as usize;
            let mut character_tables = Vec::with_capacity(character_tables_len);
            for index in 0..character_tables_len {
                let character_table = &*self.character_tables.add(index);
                character_tables.push(character_table.try_into()?);
            }

            let color_tables_len = self.color_tables_length as usize;
            let mut color_tables = Vec::with_capacity(color_tables_len);
            for index in 0..color_tables_len {
                let color_table = &*self.color_tables.add(index);
                color_tables.push(color_table.try_into()?);
            }

            let pixmap_tables_len = self.pixmap_tables_length as usize;
            let mut pixmap_tables = Vec::with_capacity(pixmap_tables_len);
            for index in 0..pixmap_tables_len {
                let pixmap_table = &*self.pixmap_tables.add(index);
                pixmap_tables.push(pixmap_table.try_into()?);
            }

            Ok(Layout {
                version: Version::try_from(self.version).unwrap(),
                compact: self.compact != 0,
                character_tables,
                color_tables,
                pixmap_tables,
            })
        }
    }
}
