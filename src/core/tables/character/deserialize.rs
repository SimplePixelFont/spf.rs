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
    byte, Character, CharacterTable, DeserializeEngine, DeserializeError, TagWriter,
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
        let modifier_flags = engine.bytes.next();
        if byte::get_bit(modifier_flags, 0) {
            self.use_advance_x = true;
        }
        if byte::get_bit(modifier_flags, 1) {
            self.use_pixmap_index = true;
        }
        if byte::get_bit(modifier_flags, 2) {
            self.use_pixmap_table_index = true;
        }
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::CharacterTableModifierFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            #[cfg(feature = "tagging")]
            vec![
                TagKind::CharacterTableUseAdvanceX {
                    table_index: engine.tagging_data.current_table_index,
                    value: self.use_advance_x,
                },
                TagKind::CharacterTableUsePixmapIndex {
                    table_index: engine.tagging_data.current_table_index,
                    value: self.use_pixmap_index,
                },
                TagKind::CharacterTableUsePixmapTableIndex {
                    table_index: engine.tagging_data.current_table_index,
                    value: self.use_pixmap_table_index,
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

        let configuration_flags = engine.bytes.next();
        let use_constant_cluster_codepoints = byte::get_bit(configuration_flags, 0);

        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::CharacterTableConfigurationFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![TagKind::CharacterTableUseConstantClusterCodepoints {
                table_index: engine.tagging_data.current_table_index,
                value: use_constant_cluster_codepoints,
            }],
            engine.bytes.byte_index(),
        );

        #[cfg(feature = "tagging")]
        let configuration_values_start = engine.bytes.byte_index();
        if use_constant_cluster_codepoints {
            self.constant_cluster_codepoints = Some(engine.bytes.next());
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::CharacterTableConstantClusterCodepoints {
                    table_index: engine.tagging_data.current_table_index,
                    value: self.constant_cluster_codepoints.unwrap(),
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

        let link_flags = engine.bytes.next();
        let link_pixmap_tables = byte::get_bit(link_flags, 0);

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

pub(crate) fn next_grapheme_cluster<R: ByteReader, T: TagWriter>(
    engine: &mut DeserializeEngine<R, T>,
    character: &mut Character,
    constant_cluster_codepoints: Option<u8>,
) {
    #[cfg(feature = "tagging")]
    let start = engine.bytes.byte_index();

    let mut grapheme_cluster = String::new();
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

        grapheme_cluster.push(
            String::from_utf8(utf8_bytes.to_vec())
                .unwrap()
                .chars()
                .next()
                .unwrap(),
        );
        codepoint_count += 1;

        if let Some(constant_cluster_codepoints) = constant_cluster_codepoints {
            if codepoint_count == constant_cluster_codepoints {
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
            value: grapheme_cluster.clone(),
        },
        Span::new(start, engine.bytes.byte_index()),
    );

    #[cfg(feature = "log")]
    info!("Identified grapheme cluster: {:?}", grapheme_cluster);

    character.grapheme_cluster = grapheme_cluster;
}
