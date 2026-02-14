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

#[cfg(not(feature = "std"))]
use alloc::ffi::*;

#[cfg(feature = "std")]
use std::str::Utf8Error;

#[cfg(not(feature = "std"))]
use core::str::Utf8Error;

use super::*;
use crate::{
    ffi_to_option, option_vec_to_raw, vec_from_raw_with_conversion, vec_to_raw,
    vec_to_raw_with_conversion, ToOwned, Vec,
};

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
            has_pixmap_table_index: character.pixmap_table_index.is_some() as c_uchar,
            pixmap_table_index: character.pixmap_table_index.unwrap_or(0) as c_uchar,
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
            let advance_x = ffi_to_option!(self.has_advance_x, self.advance_x);
            let pixmap_index = ffi_to_option!(self.has_pixmap_index, self.pixmap_index);
            let pixmap_table_index =
                ffi_to_option!(self.has_pixmap_table_index, self.pixmap_table_index);

            Ok(Character {
                advance_x,
                pixmap_index,
                pixmap_table_index,
                grapheme_cluster,
            })
        }
    }
}

impl TryFrom<Pixmap> for SPFPixmap {
    type Error = ConversionError;

    fn try_from(pixmap: Pixmap) -> Result<Self, Self::Error> {
        let (pixmap_ptr, pixmap_len) = vec_to_raw!(pixmap.data);
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
            let custom_width = ffi_to_option!(self.has_custom_width, self.custom_width);
            let custom_height = ffi_to_option!(self.has_custom_height, self.custom_height);
            let custom_bits_per_pixel =
                ffi_to_option!(self.has_custom_bits_per_pixel, self.custom_bits_per_pixel);

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
        let custom_alpha = ffi_to_option!(self.has_custom_alpha, self.custom_alpha);
        Ok(Color {
            custom_alpha,
            r: self.r,
            g: self.g,
            b: self.b,
        })
    }
}

impl TryFrom<PixmapTable> for SPFPixmapTable {
    type Error = ConversionError;

    fn try_from(table: PixmapTable) -> Result<Self, Self::Error> {
        let (color_table_indexes_ptr, color_table_indexes_len) =
            option_vec_to_raw!(table.color_table_indexes);

        let (pixmaps_ptr, pixmaps_len) = vec_to_raw_with_conversion!(table.pixmaps, SPFPixmap);

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

            let pixmaps = vec_from_raw_with_conversion!(self.pixmaps, self.pixmaps_length);
            let constant_width = ffi_to_option!(self.has_constant_width, self.constant_width);
            let constant_height = ffi_to_option!(self.has_constant_height, self.constant_height);
            let constant_bits_per_pixel = ffi_to_option!(
                self.has_constant_bits_per_pixel,
                self.constant_bits_per_pixel
            );
            let color_table_indexes =
                ffi_to_option!(self.has_color_table_indexes, color_table_indexes);

            Ok(PixmapTable {
                constant_width,
                constant_height,
                constant_bits_per_pixel,
                color_table_indexes,
                pixmaps,
            })
        }
    }
}

impl TryFrom<CharacterTable> for SPFCharacterTable {
    type Error = ConversionError;

    fn try_from(table: CharacterTable) -> Result<Self, Self::Error> {
        let (pixmap_table_indexes_ptr, pixmap_table_indexes_len) =
            option_vec_to_raw!(table.pixmap_table_indexes);

        let (characters_ptr, characters_len) =
            vec_to_raw_with_conversion!(table.characters, SPFCharacter);

        Ok(SPFCharacterTable {
            use_advance_x: table.use_advance_x as c_uchar,
            use_pixmap_index: table.use_pixmap_index as c_uchar,
            use_pixmap_table_index: table.use_pixmap_table_index as c_uchar,
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

            let characters = vec_from_raw_with_conversion!(self.characters, self.characters_length);

            let constant_cluster_codepoints = ffi_to_option!(
                self.has_constant_cluster_codepoints,
                self.constant_cluster_codepoints
            );
            let pixmap_table_indexes =
                ffi_to_option!(self.has_pixmap_table_indexes, pixmap_table_indexes);

            Ok(CharacterTable {
                use_advance_x: self.use_advance_x != 0,
                use_pixmap_index: self.use_pixmap_index != 0,
                use_pixmap_table_index: self.use_pixmap_table_index != 0,
                constant_cluster_codepoints,
                pixmap_table_indexes,
                characters,
            })
        }
    }
}

impl TryFrom<ColorTable> for SPFColorTable {
    type Error = ConversionError;

    fn try_from(table: ColorTable) -> Result<Self, Self::Error> {
        let (colors_ptr, colors_len) = vec_to_raw_with_conversion!(table.colors, SPFColor);

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
            let colors = vec_from_raw_with_conversion!(self.colors, self.colors_length);
            let constant_alpha = ffi_to_option!(self.has_constant_alpha, self.constant_alpha);

            Ok(ColorTable {
                constant_alpha,
                colors,
            })
        }
    }
}

impl TryFrom<Layout> for SPFLayout {
    type Error = ConversionError;

    fn try_from(layout: Layout) -> Result<Self, Self::Error> {
        let (character_tables_ptr, character_tables_len) =
            vec_to_raw_with_conversion!(layout.character_tables, SPFCharacterTable);
        let (color_tables_ptr, color_tables_len) =
            vec_to_raw_with_conversion!(layout.color_tables, SPFColorTable);
        let (pixmap_tables_ptr, pixmap_tables_len) =
            vec_to_raw_with_conversion!(layout.pixmap_tables, SPFPixmapTable);

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
            let character_tables =
                vec_from_raw_with_conversion!(self.character_tables, self.character_tables_length);
            let color_tables =
                vec_from_raw_with_conversion!(self.color_tables, self.color_tables_length);
            let pixmap_tables =
                vec_from_raw_with_conversion!(self.pixmap_tables, self.pixmap_tables_length);

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
