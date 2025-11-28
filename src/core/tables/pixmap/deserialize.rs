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

use crate::core::{byte, DeserializeEngine, Pixmap, PixmapTable, TagWriter};
use crate::{vec, Vec};

#[cfg(feature = "tagging")]
use crate::core::{Span, TagKind};

#[cfg(feature = "log")]
use log::*;

impl PixmapTable {
    pub(crate) fn next_modifier_flags<T: TagWriter>(&mut self, engine: &mut DeserializeEngine<T>) {
        let _modifier_flags = engine.bytes.next();
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::PixmapTableModifierFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            Vec::new(),
            engine.bytes.byte_index(),
        );
    }
    pub(crate) fn next_configurations<T: TagWriter>(&mut self, engine: &mut DeserializeEngine<T>) {
        #[cfg(feature = "tagging")]
        let configurations_start = engine.bytes.byte_index();

        let configuration_flags = engine.bytes.next();
        let use_constant_width = byte::get_bit(configuration_flags, 0);
        let use_constant_height = byte::get_bit(configuration_flags, 1);
        let use_constant_bits_per_pixel = byte::get_bit(configuration_flags, 2);

        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::PixmapTableConfigurationFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![
                TagKind::PixmapTableUseConstantWidth {
                    table_index: engine.tagging_data.current_table_index,
                    value: use_constant_width,
                },
                TagKind::PixmapTableUseConstantHeight {
                    table_index: engine.tagging_data.current_table_index,
                    value: use_constant_height,
                },
                TagKind::PixmapTableUseConstantBitsPerPixel {
                    table_index: engine.tagging_data.current_table_index,
                    value: use_constant_bits_per_pixel,
                },
            ],
            engine.bytes.byte_index(),
        );

        #[cfg(feature = "tagging")]
        let configuration_values_start = engine.bytes.byte_index();
        if use_constant_width {
            self.constant_width = Some(engine.bytes.next());
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::PixmapTableConstantWidth {
                    table_index: engine.tagging_data.current_table_index,
                    value: self.constant_width.unwrap(),
                },
                engine.bytes.byte_index(),
            );
        }
        if use_constant_height {
            self.constant_height = Some(engine.bytes.next());
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::PixmapTableConstantHeight {
                    table_index: engine.tagging_data.current_table_index,
                    value: self.constant_height.unwrap(),
                },
                engine.bytes.byte_index(),
            );
        }
        if use_constant_bits_per_pixel {
            self.constant_bits_per_pixel = Some(engine.bytes.next());
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::PixmapTableConstantBitsPerPixel {
                    table_index: engine.tagging_data.current_table_index,
                    value: self.constant_bits_per_pixel.unwrap(),
                },
                engine.bytes.byte_index(),
            );
        }

        #[cfg(feature = "tagging")]
        engine.tags.tag_span(
            TagKind::PixmapTableConfigurationValues {
                table_index: engine.tagging_data.current_table_index,
            },
            Span::new(configuration_values_start, engine.bytes.byte_index()),
        );

        #[cfg(feature = "tagging")]
        engine.tags.tag_span(
            TagKind::PixmapTableConfigurations {
                table_index: engine.tagging_data.current_table_index,
            },
            Span::new(configurations_start, engine.bytes.byte_index()),
        );
    }

    pub(crate) fn next_table_links<T: TagWriter>(&mut self, engine: &mut DeserializeEngine<T>) {
        #[cfg(feature = "tagging")]
        let links_start = engine.bytes.byte_index();

        let link_flags = engine.bytes.next();
        let link_color_tables = byte::get_bit(link_flags, 0);

        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::PixmapTableLinkFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![TagKind::PixmapTableLinkColorTables {
                table_index: engine.tagging_data.current_table_index,
                value: link_color_tables,
            }],
            engine.bytes.byte_index(),
        );

        if link_color_tables {
            #[cfg(feature = "tagging")]
            let color_tables_start = engine.bytes.byte_index();

            let color_tables_length = engine.bytes.next();
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::PixmapTableColorTableIndexesLength {
                    table_index: engine.tagging_data.current_table_index,
                    count: color_tables_length as u8,
                },
                engine.bytes.byte_index(),
            );

            #[cfg(feature = "tagging")]
            let color_table_indexes_start = engine.bytes.byte_index();

            let mut color_table_indexes = Vec::new();
            for _ in 0..color_tables_length {
                let link_index = engine.bytes.next();
                color_table_indexes.push(link_index);
                #[cfg(feature = "tagging")]
                engine.tags.tag_byte(
                    TagKind::PixmapTableColorTableIndex {
                        table_index: engine.tagging_data.current_table_index,
                        index: link_index,
                    },
                    engine.bytes.byte_index(),
                );
            }
            self.color_table_indexes = Some(color_table_indexes);

            #[cfg(feature = "tagging")]
            engine.tags.tag_span(
                TagKind::PixmapTableColorTableIndexes {
                    table_index: engine.tagging_data.current_table_index,
                    indexes: self.color_table_indexes.as_ref().unwrap().clone(),
                },
                Span::new(color_table_indexes_start, engine.bytes.byte_index()),
            );

            #[cfg(feature = "tagging")]
            engine.tags.tag_span(
                TagKind::PixmapTableColorTableLinks {
                    table_index: engine.tagging_data.current_table_index,
                },
                Span::new(color_tables_start, engine.bytes.byte_index()),
            );
        }

        #[cfg(feature = "tagging")]
        engine.tags.tag_span(
            TagKind::PixmapTableLinks {
                table_index: engine.tagging_data.current_table_index,
            },
            Span::new(links_start, engine.bytes.byte_index()),
        );
    }
}

pub(crate) fn next_width<T: TagWriter>(
    engine: &mut DeserializeEngine<T>,
    pixmap: &mut Pixmap,
    constant_width: Option<u8>,
) {
    if constant_width.is_none() {
        pixmap.custom_width = Some(engine.bytes.next());
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::PixmapCustomWidth {
                table_index: engine.tagging_data.current_table_index,
                pixmap_index: engine.tagging_data.current_record_index,
                value: pixmap.custom_width.unwrap(),
            },
            engine.bytes.byte_index(),
        );

        #[cfg(feature = "log")]
        info!("Identified custom width: {:?}", pixmap.custom_width);
    }
}

pub(crate) fn next_height<T: TagWriter>(
    engine: &mut DeserializeEngine<T>,
    pixmap: &mut Pixmap,
    constant_height: Option<u8>,
) {
    if constant_height.is_none() {
        pixmap.custom_height = Some(engine.bytes.next());
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::PixmapCustomHeight {
                table_index: engine.tagging_data.current_table_index,
                pixmap_index: engine.tagging_data.current_record_index,
                value: pixmap.custom_height.unwrap(),
            },
            engine.bytes.byte_index(),
        );

        #[cfg(feature = "log")]
        info!("Identified custom height: {:?}", pixmap.custom_height);
    }
}

pub(crate) fn next_bits_per_pixel<T: TagWriter>(
    engine: &mut DeserializeEngine<T>,
    pixmap: &mut Pixmap,
    constant_bits_per_pixel: Option<u8>,
) {
    if constant_bits_per_pixel.is_none() {
        pixmap.custom_bits_per_pixel = Some(engine.bytes.next());
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::PixmapCustomBitsPerPixel {
                table_index: engine.tagging_data.current_table_index,
                pixmap_index: engine.tagging_data.current_record_index,
                value: pixmap.custom_bits_per_pixel.unwrap(),
            },
            engine.bytes.byte_index(),
        );

        #[cfg(feature = "log")]
        info!(
            "Identified custom bits per pixel: {:?}",
            pixmap.custom_bits_per_pixel
        );
    }
}

pub(crate) fn next_pixmap<T: TagWriter>(
    engine: &mut DeserializeEngine<T>,
    pixmap: &mut Pixmap,
    constant_width: Option<u8>,
    constant_height: Option<u8>,
    constant_bits_per_pixel: Option<u8>,
) {
    #[cfg(feature = "tagging")]
    let pixmap_start = engine.bytes.byte_index();

    let bits_per_pixel = constant_bits_per_pixel
        .or(pixmap.custom_bits_per_pixel)
        .unwrap();
    let width = constant_width.or(pixmap.custom_width).unwrap();
    let height = constant_height.or(pixmap.custom_height).unwrap();

    let pixels_used = width as u16 * height as u16;
    for _ in 0..pixels_used {
        let pixel = engine.bytes.incomplete_get(bits_per_pixel);
        pixmap.data.push(pixel);
        engine.bytes.pointer += bits_per_pixel;
        if engine.bytes.pointer >= 8 {
            engine.bytes.index += 1;
            engine.bytes.pointer -= 8;
        }
    }

    resolve_final_byte(engine, width, height, bits_per_pixel);

    #[cfg(feature = "tagging")]
    engine.tags.tag_span(
        TagKind::PixmapData {
            table_index: engine.tagging_data.current_table_index,
            pixmap_index: engine.tagging_data.current_record_index,
            data: pixmap.data.clone(),
        },
        Span::new(pixmap_start, engine.bytes.byte_index()),
    );

    #[cfg(feature = "log")]
    info!("Identified pixmap: {:?}", pixmap.data);
}

#[rustversion::since(1.87)]
pub(crate) fn resolve_final_byte<T: TagWriter>(
    engine: &mut DeserializeEngine<T>,
    width: u8,
    height: u8,
    bits_per_pixel: u8,
) {
    if !engine.layout.compact && !(width * height * bits_per_pixel).is_multiple_of(8) {
        engine.bytes.index += 1;
        engine.bytes.pointer = 0;
    }
}

#[rustversion::before(1.87)]
pub(crate) fn resolve_final_byte<T: TagWriter>(
    engine: &mut DeserializeEngine<T>,
    width: u8,
    height: u8,
    bits_per_pixel: u8,
) {
    if !engine.layout.compact && (width * height * bits_per_pixel) % 8 != 0 {
        engine.bytes.index += 1;
        engine.bytes.pointer = 0;
    }
}
