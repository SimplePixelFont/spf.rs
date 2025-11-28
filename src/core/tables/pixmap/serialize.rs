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

use crate::core::{
    Pixmap, PixmapTable, SerializeEngine, SerializeError, TableIdentifier, TagWriter,
};
use crate::{format, vec, String};

#[cfg(feature = "tagging")]
use crate::core::{Span, TableType, TagKind};

#[cfg(feature = "log")]
pub(crate) use log::*;

impl PixmapTable {
    pub(crate) fn push_table_identifier<T: TagWriter>(&self, engine: &mut SerializeEngine<T>) {
        engine.bytes.push(TableIdentifier::Pixmap as u8);
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::TableIdentifier {
                table_type: TableType::Pixmap,
            },
            engine.bytes.byte_index(),
        );
    }
    pub(crate) fn push_modifier_flags<T: TagWriter>(&self, engine: &mut SerializeEngine<T>) {
        engine.bytes.push(0b00000000);
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::PixmapTableModifierFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![],
            engine.bytes.byte_index(),
        );
    }
    pub(crate) fn push_configurations<T: TagWriter>(&self, engine: &mut SerializeEngine<T>) {
        #[cfg(feature = "tagging")]
        let configurations_start = engine.bytes.byte_index();

        let mut configuration_flags = 0; // configuration flags
        if self.constant_width.is_some() {
            configuration_flags |= 0b00000001;
        }
        if self.constant_height.is_some() {
            configuration_flags |= 0b00000010;
        }
        if self.constant_bits_per_pixel.is_some() {
            configuration_flags |= 0b00000100;
        }

        engine.bytes.push(configuration_flags);
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::PixmapTableConfigurationFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![
                TagKind::PixmapTableUseConstantWidth {
                    table_index: engine.tagging_data.current_table_index,
                    value: self.constant_width.is_some(),
                },
                TagKind::PixmapTableUseConstantHeight {
                    table_index: engine.tagging_data.current_table_index,
                    value: self.constant_height.is_some(),
                },
                TagKind::PixmapTableUseConstantBitsPerPixel {
                    table_index: engine.tagging_data.current_table_index,
                    value: self.constant_bits_per_pixel.is_some(),
                },
            ],
            engine.bytes.byte_index(),
        );

        // configuration values
        #[cfg(feature = "tagging")]
        let configuration_values_start = engine.bytes.byte_index();
        if self.constant_width.is_some() {
            engine.bytes.push(self.constant_width.unwrap());
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::PixmapTableConstantWidth {
                    table_index: engine.tagging_data.current_table_index,
                    value: self.constant_width.unwrap(),
                },
                engine.bytes.byte_index(),
            );
        }
        if self.constant_height.is_some() {
            engine.bytes.push(self.constant_height.unwrap());
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::PixmapTableConstantHeight {
                    table_index: engine.tagging_data.current_table_index,
                    value: self.constant_height.unwrap(),
                },
                engine.bytes.byte_index(),
            );
        }
        if self.constant_bits_per_pixel.is_some() {
            engine.bytes.push(self.constant_bits_per_pixel.unwrap());
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
        {
            engine.tags.tag_span(
                TagKind::PixmapTableConfigurationValues {
                    table_index: engine.tagging_data.current_table_index,
                },
                Span::new(configuration_values_start, engine.bytes.byte_index()),
            );
            engine.tags.tag_span(
                TagKind::PixmapTableConfigurations {
                    table_index: engine.tagging_data.current_table_index,
                },
                Span::new(configurations_start, engine.bytes.byte_index()),
            );
        }
    }
    pub(crate) fn push_table_links<T: TagWriter>(
        &self,
        engine: &mut SerializeEngine<T>,
    ) -> Result<(), SerializeError> {
        #[cfg(feature = "tagging")]
        let table_links_start = engine.bytes.byte_index();

        let mut link_flags = 0b00000000;
        if self.color_table_indexes.is_some() {
            link_flags |= 0b00000001;
        }

        // Table relations
        engine.bytes.push(link_flags);
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::PixmapTableLinkFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![TagKind::PixmapTableLinkColorTables {
                table_index: engine.tagging_data.current_table_index,
                value: self.color_table_indexes.is_some(),
            }],
            engine.bytes.byte_index(),
        );

        if let Some(color_table_indexes) = &self.color_table_indexes {
            #[cfg(feature = "tagging")]
            let color_tables_start = engine.bytes.byte_index();

            let color_tables_length = color_table_indexes.len();
            if color_tables_length > 255 {
                return Err(SerializeError::StaticVectorTooLarge);
            }

            engine.bytes.push(color_tables_length as u8);
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

            for color_table_index in color_table_indexes {
                engine.bytes.push(*color_table_index);
                #[cfg(feature = "tagging")]
                engine.tags.tag_byte(
                    TagKind::PixmapTableColorTableIndex {
                        table_index: engine.tagging_data.current_table_index,
                        index: *color_table_index,
                    },
                    engine.bytes.byte_index(),
                );
            }

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

        // goes at very end :)
        #[cfg(feature = "tagging")]
        engine.tags.tag_span(
            TagKind::PixmapTableLinks {
                table_index: engine.tagging_data.current_table_index,
            },
            Span::new(table_links_start, engine.bytes.byte_index()),
        );

        Ok(())
    }
}

pub(crate) fn push_width<T: TagWriter>(
    engine: &mut SerializeEngine<T>,
    constant_width: Option<u8>,
    custom_width: Option<u8>,
) {
    if constant_width.is_none() {
        let width = custom_width.unwrap();
        engine.bytes.push(width);
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::PixmapCustomWidth {
                table_index: engine.tagging_data.current_table_index,
                pixmap_index: engine.tagging_data.current_record_index,
                value: width,
            },
            engine.bytes.byte_index(),
        );

        #[cfg(feature = "log")]
        {
            let width_bit_string = format!("{:08b}", width);
            info!(
                "Pushed character width '{}' with the following bits: {}",
                width, width_bit_string
            )
        }
    }
}

pub(crate) fn push_height<T: TagWriter>(
    engine: &mut SerializeEngine<T>,
    constant_height: Option<u8>,
    custom_height: Option<u8>,
) {
    if constant_height.is_none() {
        let height = custom_height.unwrap();
        engine.bytes.push(height);
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::PixmapCustomHeight {
                table_index: engine.tagging_data.current_table_index,
                pixmap_index: engine.tagging_data.current_record_index,
                value: height,
            },
            engine.bytes.byte_index(),
        );

        #[cfg(feature = "log")]
        {
            let height_bit_string = format!("{:08b}", height);
            info!(
                "Pushed character height '{}' with the following bits: {}",
                height, height_bit_string
            )
        }
    }
}

pub(crate) fn push_bits_per_pixel<T: TagWriter>(
    engine: &mut SerializeEngine<T>,
    constant_bits_per_pixel: Option<u8>,
    custom_bits_per_pixel: Option<u8>,
) {
    if constant_bits_per_pixel.is_none() {
        let bits_per_pixel = custom_bits_per_pixel.unwrap();
        engine.bytes.push(bits_per_pixel);
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::PixmapCustomBitsPerPixel {
                table_index: engine.tagging_data.current_table_index,
                pixmap_index: engine.tagging_data.current_record_index,
                value: bits_per_pixel,
            },
            engine.bytes.byte_index(),
        );

        #[cfg(feature = "log")]
        {
            let bits_per_pixel_bit_string = format!("{:08b}", bits_per_pixel);
            info!(
                "Pushed character bits_per_pixel '{}' with the following bits: {}",
                bits_per_pixel, bits_per_pixel_bit_string
            )
        }
    }
}

pub(crate) fn push_pixmap<T: TagWriter>(
    engine: &mut SerializeEngine<T>,
    constant_width: Option<u8>,
    constant_height: Option<u8>,
    constant_bits_per_pixel: Option<u8>,
    pixmap: &Pixmap,
) -> Result<(), SerializeError> {
    #[cfg(feature = "tagging")]
    let pixmap_start = engine.bytes.byte_index();

    let mut pixmap_bit_string = String::new();
    let mut bits_used: u64 = 0;

    let bits_per_pixel = constant_bits_per_pixel
        .or(pixmap.custom_bits_per_pixel)
        .unwrap();
    let width = constant_width.or(pixmap.custom_width).unwrap();
    let height = constant_height.or(pixmap.custom_height).unwrap();

    if pixmap.data.len() > width as usize * height as usize {
        return Err(SerializeError::StaticVectorTooLarge);
    }

    for pixel in pixmap.data.iter() {
        pixmap_bit_string.push_str(&format!(
            "{:0bits_per_pixel$b} ",
            pixel,
            bits_per_pixel = bits_per_pixel as usize
        ));
        engine.bytes.incomplete_push(*pixel, bits_per_pixel);
        bits_used += bits_per_pixel as u64;
    }

    if !engine.layout.compact && engine.bytes.pointer != 0 {
        engine.bytes.incomplete_push(0, 8 - (bits_used % 8) as u8);
    }

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
    info!(
        "Pushed pixmap with the following bits: {}",
        pixmap_bit_string
    );
    Ok(())
}
