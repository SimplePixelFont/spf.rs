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

// #[cfg(feature = "std")]
// use std::str::Utf8Error;

// #[cfg(not(feature = "std"))]
// use core::str::Utf8Error;

#[cfg(not(feature = "std"))]
use alloc::ffi::*;

use super::*;

// #[derive(Debug, Clone)]
// pub enum ConversionError {
//     NulError(NulError),
//     Utf8Error(Utf8Error),
// }

// impl From<NulError> for ConversionError {
//     fn from(err: NulError) -> Self {
//         ConversionError::NulError(err)
//     }
// }

// impl From<Utf8Error> for ConversionError {
//     fn from(err: Utf8Error) -> Self {
//         ConversionError::Utf8Error(err)
//     }
// }

// impl TryFrom<Character> for SPFCharacter {
//     type Error = ConversionError;

//     fn try_from(character: Character) -> Result<Self, Self::Error> {
//         let pixmap_len = character.pixmap.len();
//         let pixmap_ptr = if pixmap_len == 0 {
//             core::ptr::null_mut()
//         } else {
//             let mut pixmap_vec = character.pixmap.into_boxed_slice();
//             let ptr = pixmap_vec.as_mut_ptr();
//             core::mem::forget(pixmap_vec);
//             ptr
//         };

//         let grapheme_cluster = CString::new(character.grapheme_cluster.as_str())?;

//         let grapheme_cluster_ptr = grapheme_cluster.as_ptr();
//         core::mem::forget(grapheme_cluster);

//         Ok(SPFCharacter {
//             grapheme_cluster: grapheme_cluster_ptr,
//             custom_width: character.custom_width.unwrap_or(0),
//             custom_height: character.custom_height.unwrap_or(0),
//             pixmap: pixmap_ptr,
//             pixmap_length: pixmap_len as c_ulong,
//         })
//     }
// }

// impl TryInto<Character> for &SPFCharacter {
//     type Error = ConversionError;

//     fn try_into(self) -> Result<Character, Self::Error> {
//         unsafe {
//             let grapheme_cluster = CStr::from_ptr(self.grapheme_cluster).to_str()?.to_owned();
//             let custom_width = if self.custom_width == 0 {
//                 None
//             } else {
//                 Some(self.custom_width)
//             };
//             let custom_height = if self.custom_height == 0 {
//                 None
//             } else {
//                 Some(self.custom_height)
//             };
//             let pixmap = slice::from_raw_parts(self.pixmap, self.pixmap_length as usize).to_vec();

//             Ok(Character {
//                 grapheme_cluster,
//                 custom_width,
//                 custom_height,
//                 pixmap,
//             })
//         }
//     }
// }

// impl TryFrom<Body> for SPFBody {
//     type Error = ConversionError;

//     fn try_from(body: Body) -> Result<Self, Self::Error> {
//         let characters_len: usize = body.characters.len();
//         let mut characters: Vec<SPFCharacter> = Vec::with_capacity(characters_len);

//         for character in body.characters {
//             characters.push(character.try_into()?);
//         }

//         let characters_ptr = if characters_len == 0 {
//             core::ptr::null_mut()
//         } else {
//             let mut characters_raw = characters.into_boxed_slice();
//             let ptr = characters_raw.as_mut_ptr();
//             core::mem::forget(characters_raw);
//             ptr
//         };

//         Ok(SPFBody {
//             characters: characters_ptr,
//             characters_length: characters_len as c_ulong,
//         })
//     }
// }

// impl TryInto<Body> for SPFBody {
//     type Error = ConversionError;

//     fn try_into(self) -> Result<Body, Self::Error> {
//         let characters_len = self.characters_length as usize;
//         let mut characters = Vec::with_capacity(characters_len);
//         for index in 0..characters_len {
//             unsafe {
//                 let character = &*self.characters.add(index);

//                 characters.push(character.try_into()?);
//             }
//         }
//         Ok(Body { characters })
//     }
// }

// impl TryFrom<Layout> for SPFLayout {
//     type Error = ConversionError;

//     fn try_from(layout: Layout) -> Result<Self, Self::Error> {
//         Ok(SPFLayout {
//             header: SPFHeader {
//                 configuration_flags: SPFConfigurationFlags {
//                     constant_cluster_codepoints: layout
//                         .header
//                         .configuration_flags
//                         .constant_cluster_codepoints
//                         as u8,
//                     constant_width: layout.header.configuration_flags.constant_width as u8,
//                     constant_height: layout.header.configuration_flags.constant_height as u8,
//                     custom_bits_per_pixel: layout.header.configuration_flags.custom_bits_per_pixel
//                         as u8,
//                 },
//                 modifier_flags: SPFModifierFlags {
//                     compact: layout.header.modifier_flags.compact as u8,
//                 },
//                 configuration_values: SPFConfigurationValues {
//                     constant_cluster_codepoints: layout
//                         .header
//                         .configuration_values
//                         .constant_cluster_codepoints
//                         .unwrap_or(0),
//                     constant_width: layout
//                         .header
//                         .configuration_values
//                         .constant_width
//                         .unwrap_or(0),
//                     constant_height: layout
//                         .header
//                         .configuration_values
//                         .constant_height
//                         .unwrap_or(0),
//                     custom_bits_per_pixel: layout
//                         .header
//                         .configuration_values
//                         .custom_bits_per_pixel
//                         .unwrap_or(0),
//                 },
//             },
//             body: layout.body.try_into()?,
//         })
//     }
// }

// impl TryInto<Layout> for SPFLayout {
//     type Error = ConversionError;

//     fn try_into(self) -> Result<Layout, Self::Error> {
//         let constant_cluster_codepoints =
//             if self.header.configuration_values.constant_cluster_codepoints == 0 {
//                 None
//             } else {
//                 Some(self.header.configuration_values.constant_cluster_codepoints)
//             };

//         let constant_width = if self.header.configuration_values.constant_width == 0 {
//             None
//         } else {
//             Some(self.header.configuration_values.constant_width)
//         };

//         let constant_height = if self.header.configuration_values.constant_height == 0 {
//             None
//         } else {
//             Some(self.header.configuration_values.constant_height)
//         };

//         let custom_bits_per_pixel = if self.header.configuration_values.custom_bits_per_pixel == 0 {
//             None
//         } else {
//             Some(self.header.configuration_values.custom_bits_per_pixel)
//         };

//         Ok(Layout {
//             header: Header {
//                 configuration_flags: ConfigurationFlags {
//                     constant_cluster_codepoints: self
//                         .header
//                         .configuration_flags
//                         .constant_cluster_codepoints
//                         != 0,
//                     constant_width: self.header.configuration_flags.constant_width != 0,
//                     constant_height: self.header.configuration_flags.constant_height != 0,
//                     custom_bits_per_pixel: self.header.configuration_flags.custom_bits_per_pixel
//                         != 0,
//                 },
//                 modifier_flags: ModifierFlags {
//                     compact: self.header.modifier_flags.compact != 0,
//                 },
//                 configuration_values: ConfigurationValues {
//                     constant_cluster_codepoints,
//                     constant_width,
//                     constant_height,
//                     custom_bits_per_pixel,
//                 },
//             },
//             body: self.body.try_into()?,
//         })
//     }
// }

// impl TryFrom<CharacterCache> for SPFCharacterCache {
//     type Error = ConversionError;

//     fn try_from(cache: CharacterCache) -> Result<Self, Self::Error> {
//         let keys: Vec<*const c_char> = cache
//             .mappings
//             .keys()
//             .map(|a| {
//                 let utf8 =
//                     CString::new(a.to_string().as_bytes().to_vec().into_boxed_slice()).unwrap();
//                 let utf8_ptr = utf8.as_ptr();
//                 core::mem::forget(utf8);
//                 utf8_ptr as *const c_char
//             })
//             .collect();
//         let values: Vec<usize> = cache.mappings.values().copied().collect();
//         let length = keys.len();

//         let keys_ptr = if length == 0 {
//             core::ptr::null_mut()
//         } else {
//             let mut keys_vec = keys.into_boxed_slice();
//             let ptr = keys_vec.as_mut_ptr();
//             core::mem::forget(keys_vec);
//             ptr
//         };

//         let values_ptr = if length == 0 {
//             core::ptr::null_mut()
//         } else {
//             let mut values_vec = values.into_boxed_slice();
//             let ptr = values_vec.as_mut_ptr();
//             core::mem::forget(values_vec);
//             ptr
//         };

//         Ok(SPFCharacterCache {
//             mappings_keys: keys_ptr,
//             mappings_values: values_ptr as *mut c_ulong,
//             mappings_length: length as c_ulong,
//         })
//     }
// }

// impl TryInto<CharacterCache> for SPFCharacterCache {
//     type Error = ConversionError;

//     fn try_into(self) -> Result<CharacterCache, Self::Error> {
//         unsafe {
//             let mut mappings = HashMap::new();
//             let keys =
//                 slice::from_raw_parts(self.mappings_keys, self.mappings_length as usize).to_vec();
//             let values =
//                 slice::from_raw_parts(self.mappings_values, self.mappings_length as usize).to_vec();

//             for (key, value) in keys.into_iter().zip(values.into_iter()) {
//                 let grapheme_cluster = CStr::from_ptr(key).to_str()?.to_owned();
//                 mappings.insert(grapheme_cluster, value as usize);
//             }

//             Ok(CharacterCache { mappings })
//         }
//     }
// }

// impl TryFrom<Surface> for SPFSurface {
//     type Error = ConversionError;

//     fn try_from(surface: Surface) -> Result<Self, Self::Error> {
//         let data_len = surface.data.len();
//         let data_ptr = if data_len == 0 {
//             core::ptr::null_mut()
//         } else {
//             let mut data_vec = surface.data.into_boxed_slice();
//             let ptr = data_vec.as_mut_ptr();
//             core::mem::forget(data_vec);
//             ptr
//         };
//         Ok(SPFSurface {
//             width: surface.width as c_ulong,
//             height: surface.height as c_ulong,
//             data: data_ptr as *mut c_ulong,
//             data_length: data_len as c_ulong,
//         })
//     }
// }

// impl TryInto<Surface> for SPFSurface {
//     type Error = ConversionError;

//     fn try_into(self) -> Result<Surface, Self::Error> {
//         unsafe {
//             let data = slice::from_raw_parts(self.data, self.data_length as usize)
//                 .iter()
//                 .map(|a| *a as usize)
//                 .collect();
//             Ok(Surface {
//                 width: self.width as usize,
//                 height: self.height as usize,
//                 data,
//             })
//         }
//     }
// }

// impl TryFrom<Printer> for SPFPrinter {
//     type Error = ConversionError;

//     fn try_from(value: Printer) -> Result<Self, Self::Error> {
//         Ok(SPFPrinter {
//             font: value.font.try_into()?,
//             character_cache: value.character_cache.try_into()?,
//             letter_spacing: value.letter_spacing as c_ulong,
//         })
//     }
// }

// impl TryInto<Printer> for SPFPrinter {
//     type Error = ConversionError;

//     fn try_into(self) -> Result<Printer, Self::Error> {
//         Ok(Printer {
//             font: self.font.try_into()?,
//             character_cache: self.character_cache.try_into()?,
//             letter_spacing: self.letter_spacing as usize,
//         })
//     }
// }
