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

use crate::core::byte::ByteReader;
use crate::core::{
    byte, Color, ColorTable, DeserializeEngine, DeserializeError, SerializeEngine, SerializeError,
    Table, TableIdentifier, TagWriter,
};
use crate::{vec, Vec};

#[cfg(feature = "tagging")]
use crate::core::{ByteIndex, Span, TableType, TagKind};

impl ColorTable {
    pub(crate) fn next_modifer_flags<R: ByteReader, T: TagWriter>(
        &mut self,
        engine: &mut DeserializeEngine<R, T>,
    ) {
        let _modifier_flags = engine.bytes.next();
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::ColorTableModifierFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![],
            engine.bytes.byte_index(),
        );
    }
    pub(crate) fn next_configurations<R: ByteReader, T: TagWriter>(
        &mut self,
        engine: &mut DeserializeEngine<R, T>,
    ) {
        #[cfg(feature = "tagging")]
        let configurations_start = engine.bytes.byte_index();

        let configuration_flags = engine.bytes.next();
        let use_constant_alpha = byte::get_bit(configuration_flags, 0);

        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::ColorTableConfigurationFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![TagKind::ColorTableUseConstantAlpha {
                table_index: engine.tagging_data.current_table_index,
                value: use_constant_alpha,
            }],
            engine.bytes.byte_index(),
        );

        #[cfg(feature = "tagging")]
        let configuration_values_start = engine.bytes.byte_index();
        if use_constant_alpha {
            self.constant_alpha = Some(engine.bytes.next());
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::ColorTableConstantAlpha {
                    table_index: engine.tagging_data.current_table_index,
                    value: self.constant_alpha.unwrap(),
                },
                engine.bytes.byte_index(),
            );
        }

        #[cfg(feature = "tagging")]
        {
            engine.tags.tag_span(
                TagKind::ColorTableConfigurationValues {
                    table_index: engine.tagging_data.current_table_index,
                },
                Span::new(configuration_values_start, engine.bytes.byte_index()),
            );
            engine.tags.tag_span(
                TagKind::ColorTableConfigurations {
                    table_index: engine.tagging_data.current_table_index,
                },
                Span::new(configurations_start, engine.bytes.byte_index()),
            );
        }
    }
    pub(crate) fn next_table_links<R: ByteReader, T: TagWriter>(
        &mut self,
        engine: &mut DeserializeEngine<R, T>,
    ) {
        #[cfg(feature = "tagging")]
        let links_start = engine.bytes.byte_index();

        let _link_flags = engine.bytes.next();
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::ColorTableLinkFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            Vec::new(),
            engine.bytes.byte_index(),
        );

        #[cfg(feature = "tagging")]
        {
            engine.tags.tag_span(
                TagKind::ColorTableLinks {
                    table_index: engine.tagging_data.current_table_index,
                },
                Span::new(links_start, engine.bytes.byte_index()),
            );
        }
    }
}

impl Table for ColorTable {
    fn deserialize<R: ByteReader, T: TagWriter>(
        engine: &mut DeserializeEngine<R, T>,
    ) -> Result<Self, DeserializeError> {
        #[cfg(feature = "tagging")]
        let table_start = engine.bytes.byte_index();
        #[cfg(feature = "tagging")]
        let table_start = ByteIndex::new(table_start.byte - 1, table_start.bit);
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::TableIdentifier {
                table_type: TableType::Color,
            },
            engine.bytes.byte_index(),
        );

        let mut color_table = ColorTable::default();
        color_table.next_modifer_flags(engine);
        color_table.next_configurations(engine);
        color_table.next_table_links(engine);

        let color_count = engine.bytes.next();
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::ColorTableColorCount {
                table_index: engine.tagging_data.current_table_index,
                count: color_count,
            },
            engine.bytes.byte_index(),
        );
        for index in 0..color_count {
            #[cfg(feature = "tagging")]
            {
                engine.tagging_data.current_record_index = index;
            }
            #[cfg(feature = "tagging")]
            let color_start = engine.bytes.byte_index();

            let mut color = Color::default();
            if color_table.constant_alpha.is_none() {
                color.custom_alpha = Some(engine.bytes.next());
                #[cfg(feature = "tagging")]
                engine.tags.tag_byte(
                    TagKind::ColorCustomAlpha {
                        table_index: engine.tagging_data.current_table_index,
                        color_index: engine.tagging_data.current_record_index,
                        value: color.custom_alpha.unwrap(),
                    },
                    engine.bytes.byte_index(),
                );
            }
            color.r = engine.bytes.next();
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::ColorR {
                    table_index: engine.tagging_data.current_table_index,
                    color_index: engine.tagging_data.current_record_index,
                    value: color.r,
                },
                engine.bytes.byte_index(),
            );
            color.g = engine.bytes.next();
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::ColorG {
                    table_index: engine.tagging_data.current_table_index,
                    color_index: engine.tagging_data.current_record_index,
                    value: color.g,
                },
                engine.bytes.byte_index(),
            );
            color.b = engine.bytes.next();
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::ColorB {
                    table_index: engine.tagging_data.current_table_index,
                    color_index: engine.tagging_data.current_record_index,
                    value: color.b,
                },
                engine.bytes.byte_index(),
            );

            color_table.colors.push(color);

            #[cfg(feature = "tagging")]
            engine.tags.tag_span(
                TagKind::ColorRecord {
                    table_index: engine.tagging_data.current_table_index,
                    color_index: engine.tagging_data.current_record_index,
                },
                Span::new(color_start, engine.bytes.byte_index()),
            );
        }

        #[cfg(feature = "tagging")]
        engine.tags.tag_span(
            TagKind::ColorTable {
                index: engine.tagging_data.current_table_index,
            },
            Span::new(table_start, engine.bytes.byte_index()),
        );
        Ok(color_table)
    }

    fn serialize<T: TagWriter>(
        &self,
        engine: &mut SerializeEngine<T>,
    ) -> Result<(), crate::core::SerializeError> {
        #[cfg(feature = "tagging")]
        let table_start = engine.bytes.byte_index();

        self.push_table_identifier(engine);
        self.push_modifier_flags(engine);
        self.push_configurations(engine);
        self.push_table_links(engine);

        if self.colors.len() > 255 {
            return Err(SerializeError::StaticVectorTooLarge);
        }
        engine.bytes.push(self.colors.len() as u8);
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::ColorTableColorCount {
                table_index: engine.tagging_data.current_table_index,
                count: self.colors.len() as u8,
            },
            engine.bytes.byte_index(),
        );

        for (index, color) in self.colors.iter().enumerate() {
            #[cfg(feature = "tagging")]
            {
                engine.tagging_data.current_record_index = index as u8;
            }
            #[cfg(feature = "tagging")]
            let color_start = engine.bytes.byte_index();

            if self.constant_alpha.is_none() {
                engine.bytes.push(color.custom_alpha.unwrap());
                #[cfg(feature = "tagging")]
                engine.tags.tag_byte(
                    TagKind::ColorCustomAlpha {
                        table_index: engine.tagging_data.current_table_index,
                        color_index: engine.tagging_data.current_record_index,
                        value: color.custom_alpha.unwrap(),
                    },
                    engine.bytes.byte_index(),
                );
            }
            engine.bytes.push(color.r);
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::ColorR {
                    table_index: engine.tagging_data.current_table_index,
                    color_index: engine.tagging_data.current_record_index,
                    value: color.r,
                },
                engine.bytes.byte_index(),
            );
            engine.bytes.push(color.g);
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::ColorG {
                    table_index: engine.tagging_data.current_table_index,
                    color_index: engine.tagging_data.current_record_index,
                    value: color.g,
                },
                engine.bytes.byte_index(),
            );
            engine.bytes.push(color.b);
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::ColorB {
                    table_index: engine.tagging_data.current_table_index,
                    color_index: engine.tagging_data.current_record_index,
                    value: color.b,
                },
                engine.bytes.byte_index(),
            );

            #[cfg(feature = "tagging")]
            engine.tags.tag_span(
                TagKind::ColorRecord {
                    table_index: engine.tagging_data.current_table_index,
                    color_index: engine.tagging_data.current_record_index,
                },
                Span::new(color_start, engine.bytes.byte_index()),
            );
        }

        #[cfg(feature = "tagging")]
        engine.tags.tag_span(
            TagKind::ColorTable {
                index: engine.tagging_data.current_table_index,
            },
            Span::new(table_start, engine.bytes.byte_index()),
        );

        Ok(())
    }
}

impl ColorTable {
    pub(crate) fn push_table_identifier<T: TagWriter>(&self, engine: &mut SerializeEngine<T>) {
        engine.bytes.push(TableIdentifier::Color as u8);
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::TableIdentifier {
                table_type: TableType::Color,
            },
            engine.bytes.byte_index(),
        );
    }
    pub(crate) fn push_modifier_flags<T: TagWriter>(&self, engine: &mut SerializeEngine<T>) {
        engine.bytes.push(0b00000000);
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::ColorTableModifierFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![],
            engine.bytes.byte_index(),
        );
    }
    pub(crate) fn push_configurations<T: TagWriter>(&self, engine: &mut SerializeEngine<T>) {
        #[cfg(feature = "tagging")]
        let configurations_start = engine.bytes.byte_index();

        let mut configuration_flags = 0;
        if self.constant_alpha.is_some() {
            configuration_flags |= 0b00000001;
        }

        engine.bytes.push(configuration_flags); // configuration flags
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::ColorTableConfigurationFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![TagKind::ColorTableUseConstantAlpha {
                table_index: engine.tagging_data.current_table_index,
                value: self.constant_alpha.is_some(),
            }],
            engine.bytes.byte_index(),
        );

        // configuration values
        #[cfg(feature = "tagging")]
        let configuration_values_start = engine.bytes.byte_index();
        if self.constant_alpha.is_some() {
            engine.bytes.push(self.constant_alpha.unwrap());
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::ColorTableConstantAlpha {
                    table_index: engine.tagging_data.current_table_index,
                    value: self.constant_alpha.unwrap(),
                },
                engine.bytes.byte_index(),
            );
        }

        #[cfg(feature = "tagging")]
        {
            engine.tags.tag_span(
                TagKind::ColorTableConfigurationValues {
                    table_index: engine.tagging_data.current_table_index,
                },
                Span::new(configuration_values_start, engine.bytes.byte_index()),
            );
            engine.tags.tag_span(
                TagKind::ColorTableConfigurations {
                    table_index: engine.tagging_data.current_table_index,
                },
                Span::new(configurations_start, engine.bytes.byte_index()),
            );
        }
    }
    pub(crate) fn push_table_links<T: TagWriter>(&self, engine: &mut SerializeEngine<T>) {
        #[cfg(feature = "tagging")]
        let links_start = engine.bytes.byte_index();

        engine.bytes.push(0b00000000); // Link flags
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::ColorTableLinkFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![],
            engine.bytes.byte_index(),
        );

        #[cfg(feature = "tagging")]
        {
            engine.tags.tag_span(
                TagKind::ColorTableLinks {
                    table_index: engine.tagging_data.current_table_index,
                },
                Span::new(links_start, engine.bytes.byte_index()),
            );
        }
    }
}
