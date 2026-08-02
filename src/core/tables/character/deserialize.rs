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
    Character, CharacterTable, CharacterTableConfigurationFlags, CharacterTableLinkFlags,
    CharacterTableModifierFlags, DeserializeEngine, DeserializeError, TagWriter,
};
use crate::{vec, String, Vec};

#[cfg(feature = "tagging")]
use crate::tagging::{Span, TagKind};

#[cfg(feature = "log")]
use log::*;

impl CharacterTable {
    pub(crate) fn next_modifer_flags<R: ByteReader, T: TagWriter>(
        &mut self,
        engine: &mut DeserializeEngine<R, T>,
    ) {
        self.modifier_flags = CharacterTableModifierFlags::from_bits_retain(engine.bytes.next());
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::CharacterTableModifierFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            #[cfg(feature = "tagging")]
            vec![
                TagKind::CharacterTableUseAdvanceX {
                    table_index: engine.tagging_data.current_table_index,
                    value: self
                        .modifier_flags
                        .contains(CharacterTableModifierFlags::UseAdvanceX),
                },
                TagKind::CharacterTableUsePixmapIndex {
                    table_index: engine.tagging_data.current_table_index,
                    value: self
                        .modifier_flags
                        .contains(CharacterTableModifierFlags::UsePixmapIndex),
                },
                TagKind::CharacterTableUsePixmapTableIndex {
                    table_index: engine.tagging_data.current_table_index,
                    value: self
                        .modifier_flags
                        .contains(CharacterTableModifierFlags::UsePixmapTableIndex),
                },
            ],
            engine.bytes.byte_index(),
        );
    }
    pub(crate) fn next_configurations<R: ByteReader, T: TagWriter>(
        &mut self,
        engine: &mut DeserializeEngine<R, T>,
    ) {
        #[cfg(feature = "tagging")]
        let configurations_start = engine.bytes.byte_index();

        self.configuration_flags =
            CharacterTableConfigurationFlags::from_bits_retain(engine.bytes.next());
        let use_constant_code_point_count = self
            .configuration_flags
            .contains(CharacterTableConfigurationFlags::ConstantCodePointCount);

        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::CharacterTableConfigurationFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![TagKind::CharacterTableUseConstantClusterCodepoints {
                table_index: engine.tagging_data.current_table_index,
                value: use_constant_code_point_count,
            }],
            engine.bytes.byte_index(),
        );

        #[cfg(feature = "tagging")]
        let configuration_values_start = engine.bytes.byte_index();
        if use_constant_code_point_count {
            self.constant_code_point_count = Some(engine.bytes.next());
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::CharacterTableConstantClusterCodepoints {
                    table_index: engine.tagging_data.current_table_index,
                    value: self.constant_code_point_count.unwrap(),
                },
                engine.bytes.byte_index(),
            );
        }

        #[cfg(feature = "tagging")]
        {
            engine.tags.tag_span(
                TagKind::CharacterTableConfigurationValues {
                    table_index: engine.tagging_data.current_table_index,
                },
                Span::new(configuration_values_start, engine.bytes.byte_index()),
            );
            engine.tags.tag_span(
                TagKind::CharacterTableConfigurations {
                    table_index: engine.tagging_data.current_table_index,
                },
                Span::new(configurations_start, engine.bytes.byte_index()),
            );
        }
    }
    pub(crate) fn next_table_links<R: ByteReader, T: TagWriter>(
        &mut self,
        engine: &mut DeserializeEngine<R, T>,
    ) -> Result<(), DeserializeError> {
        #[cfg(feature = "tagging")]
        let links_start = engine.bytes.byte_index();

        self.link_flags = CharacterTableLinkFlags::from_bits_retain(engine.bytes.next());
        let link_pixmap_tables = self
            .link_flags
            .contains(CharacterTableLinkFlags::LinkPixmapTables);

        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::CharacterTableLinkFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![TagKind::CharacterTableLinkPixmapTables {
                table_index: engine.tagging_data.current_table_index,
                value: link_pixmap_tables,
            }],
            engine.bytes.byte_index(),
        );

        if link_pixmap_tables {
            #[cfg(feature = "tagging")]
            let pixmap_tables_start = engine.bytes.byte_index();

            let pixmap_tables_length = engine.bytes.next();
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::CharacterTablePixmapTableIndexesLength {
                    table_index: engine.tagging_data.current_table_index,
                    count: pixmap_tables_length,
                },
                engine.bytes.byte_index(),
            );

            #[cfg(feature = "tagging")]
            let pixmap_table_indexes_start = engine.bytes.byte_index();

            let mut pixmap_table_indexes = Vec::new();
            for _ in 0..pixmap_tables_length {
                let link_index = engine.bytes.next();
                pixmap_table_indexes.push(link_index);
                #[cfg(feature = "tagging")]
                engine.tags.tag_byte(
                    TagKind::CharacterTablePixmapTableIndex {
                        table_index: engine.tagging_data.current_table_index,
                        index: link_index,
                    },
                    engine.bytes.byte_index(),
                );
            }

            self.pixmap_table_indexes = Some(pixmap_table_indexes);

            #[cfg(feature = "tagging")]
            engine.tags.tag_span(
                TagKind::CharacterTablePixmapTableIndexes {
                    table_index: engine.tagging_data.current_table_index,
                    indexes: self.pixmap_table_indexes.as_ref().unwrap().clone(),
                },
                Span::new(pixmap_table_indexes_start, engine.bytes.byte_index()),
            );

            #[cfg(feature = "tagging")]
            engine.tags.tag_span(
                TagKind::CharacterTablePixmapTableLinks {
                    table_index: engine.tagging_data.current_table_index,
                },
                Span::new(pixmap_tables_start, engine.bytes.byte_index()),
            );
        }

        #[cfg(feature = "tagging")]
        engine.tags.tag_span(
            TagKind::CharacterTableLinks {
                table_index: engine.tagging_data.current_table_index,
            },
            Span::new(links_start, engine.bytes.byte_index()),
        );
        Ok(())
    }
}

pub(crate) fn next_code_points<R: ByteReader, T: TagWriter>(
    engine: &mut DeserializeEngine<R, T>,
    character: &mut Character,
    constant_code_point_count: Option<u8>,
) {
    #[cfg(feature = "tagging")]
    let start = engine.bytes.byte_index();

    let mut code_points = String::new();
    let mut end_cluster = false;
    let mut codepoint_count = 0;

    while !end_cluster {
        let utf81 = engine.bytes.next();
        let mut utf8_bytes: [u8; 4] = [0, 0, 0, 0];

        if utf81 >> 7 == 0b00000000 {
            utf8_bytes[0] = utf81;
        } else if utf81 >> 5 == 0b00000110 {
            utf8_bytes[0] = utf81;
            utf8_bytes[1] = engine.bytes.next();
        } else if utf81 >> 4 == 0b00001110 {
            utf8_bytes[0] = utf81;
            utf8_bytes[1] = engine.bytes.next();
            utf8_bytes[2] = engine.bytes.next();
        } else if utf81 >> 3 == 0b00011110 {
            utf8_bytes[0] = utf81;
            utf8_bytes[1] = engine.bytes.next();
            utf8_bytes[2] = engine.bytes.next();
            utf8_bytes[3] = engine.bytes.next();
        }

        code_points.push(
            String::from_utf8(utf8_bytes.to_vec())
                .unwrap()
                .chars()
                .next()
                .unwrap(),
        );
        codepoint_count += 1;

        if let Some(constant_code_point_count) = constant_code_point_count {
            if codepoint_count == constant_code_point_count {
                end_cluster = true;
            }
        } else if engine.bytes.get() == 0 {
            end_cluster = true;
            engine.bytes.next();
        }
    }

    #[cfg(feature = "tagging")]
    engine.tags.tag_span(
        TagKind::CharacterGraphemeCluster {
            table_index: engine.tagging_data.current_table_index,
            char_index: engine.tagging_data.current_record_index,
            value: code_points.clone(),
        },
        Span::new(start, engine.bytes.byte_index()),
    );

    #[cfg(feature = "log")]
    info!("Identified code points: {:?}", code_points);

    character.code_points = code_points;
}
