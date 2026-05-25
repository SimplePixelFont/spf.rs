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

#![doc(hidden)]
//! Free functions for all heap-allocated SPF FFI types.
//!
//! Every allocation produced by [`spf_core_layout_from_data`] or [`spf_core_layout_to_data`]
//! must be released by the corresponding free function here. Failing to do so leaks memory.
//! Freeing the same allocation twice is undefined behaviour — do not call these functions on
//! stack-allocated or already-freed values.

use super::*;

#[cfg(feature = "std")]
use std::ffi::CString;
#[cfg(not(feature = "std"))]
use alloc::ffi::CString;

#[no_mangle]
/// Frees the byte buffer owned by an [`SPFData`] value.
///
/// Pass the [`SPFData`] by value; after this call the contained pointer is invalid.
/// A null `data` pointer is safe to pass.
pub unsafe extern "C" fn spf_free_data(data: SPFData) {
    if !data.data.is_null() {
        unsafe {
            drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(
                data.data,
                data.data_length as usize,
            )));
        }
    }
}

#[no_mangle]
/// Recursively frees all heap memory owned by an [`SPFLayout`] value.
///
/// This includes all table arrays, their contents, and every `CString` field inside
/// [`SPFCharacter`] and [`SPFFont`] entries. Pass the [`SPFLayout`] by value; after
/// this call all contained pointers are invalid.
/// Null table pointers are safe to pass (no-op per table).
pub unsafe extern "C" fn spf_free_layout(layout: SPFLayout) {
    unsafe {
        free_character_tables(layout.character_tables, layout.character_tables_length as usize);
        free_color_tables(layout.color_tables, layout.color_tables_length as usize);
        free_pixmap_tables(layout.pixmap_tables, layout.pixmap_tables_length as usize);
        free_font_tables(layout.font_tables, layout.font_tables_length as usize);
    }
}

/// Frees an array of [`SPFCharacterTable`] values along with all nested allocations.
///
/// For each table: frees each character's `grapheme_cluster` CString, then the characters
/// array, then the optional `pixmap_table_indexes` array. Finally frees the tables array itself.
unsafe fn free_character_tables(ptr: *mut SPFCharacterTable, len: usize) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let tables = core::slice::from_raw_parts(ptr, len);
        for table in tables {
            if !table.characters.is_null() {
                let chars = core::slice::from_raw_parts(
                    table.characters,
                    table.characters_length as usize,
                );
                for ch in chars {
                    if !ch.grapheme_cluster.is_null() {
                        drop(CString::from_raw(ch.grapheme_cluster));
                    }
                }
                drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(
                    table.characters,
                    table.characters_length as usize,
                )));
            }
            if !table.pixmap_table_indexes.is_null() {
                drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(
                    table.pixmap_table_indexes,
                    table.pixmap_table_indexes_length as usize,
                )));
            }
        }
        drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(ptr, len)));
    }
}

/// Frees an array of [`SPFColorTable`] values along with all nested allocations.
///
/// [`SPFColor`] contains no heap pointers, so only the colors slice and the tables
/// array itself need to be freed.
unsafe fn free_color_tables(ptr: *mut SPFColorTable, len: usize) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let tables = core::slice::from_raw_parts(ptr, len);
        for table in tables {
            if !table.colors.is_null() {
                drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(
                    table.colors,
                    table.colors_length as usize,
                )));
            }
        }
        drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(ptr, len)));
    }
}

/// Frees an array of [`SPFPixmapTable`] values along with all nested allocations.
///
/// For each table: frees each pixmap's `data` byte array, then the pixmaps slice,
/// then the optional `color_table_indexes` array. Finally frees the tables array itself.
unsafe fn free_pixmap_tables(ptr: *mut SPFPixmapTable, len: usize) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let tables = core::slice::from_raw_parts(ptr, len);
        for table in tables {
            if !table.pixmaps.is_null() {
                let pixmaps = core::slice::from_raw_parts(
                    table.pixmaps,
                    table.pixmaps_length as usize,
                );
                for pixmap in pixmaps {
                    if !pixmap.data.is_null() {
                        drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(
                            pixmap.data,
                            pixmap.data_length as usize,
                        )));
                    }
                }
                drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(
                    table.pixmaps,
                    table.pixmaps_length as usize,
                )));
            }
            if !table.color_table_indexes.is_null() {
                drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(
                    table.color_table_indexes,
                    table.color_table_indexes_length as usize,
                )));
            }
        }
        drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(ptr, len)));
    }
}

/// Frees an array of [`SPFFontTable`] values along with all nested allocations.
///
/// For each table: frees each font's `name` and `author` CStrings and its
/// `character_table_indexes` byte array, then the fonts slice, then the optional
/// table-level `character_table_indexes` array. Finally frees the tables array itself.
unsafe fn free_font_tables(ptr: *mut SPFFontTable, len: usize) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let tables = core::slice::from_raw_parts(ptr, len);
        for table in tables {
            if !table.fonts.is_null() {
                let fonts = core::slice::from_raw_parts(
                    table.fonts,
                    table.fonts_length as usize,
                );
                for font in fonts {
                    if !font.name.is_null() {
                        drop(CString::from_raw(font.name));
                    }
                    if !font.author.is_null() {
                        drop(CString::from_raw(font.author));
                    }
                    if !font.character_table_indexes.is_null() {
                        drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(
                            font.character_table_indexes,
                            font.character_tables_indexes_length as usize,
                        )));
                    }
                }
                drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(
                    table.fonts,
                    table.fonts_length as usize,
                )));
            }

            if !table.character_table_indexes.is_null() {
                drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(
                    table.character_table_indexes,
                    table.character_table_indexes_length as usize,
                )));
            }
        }
        drop(Box::from_raw(core::ptr::slice_from_raw_parts_mut(ptr, len)));
    }
}
