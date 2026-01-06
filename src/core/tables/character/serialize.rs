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

use crate::core::{CharacterTable, SerializeEngine, SerializeError, TableIdentifier, TagWriter};
use crate::{vec, String};

#[cfg(feature = "tagging")]
use crate::tagging::{Span, TagKind};

#[cfg(feature = "log")]
use crate::format;
#[cfg(feature = "log")]
use log::*;

impl CharacterTable {
    pub(crate) fn push_table_identifier<T: TagWriter>(&self, engine: &mut SerializeEngine<T>) {
        engine.bytes.push(TableIdentifier::Character as u8);
        #[cfg(feature = "tagging")]
        engine.tags.tag_byte(
            TagKind::TableIdentifier {
                table_type: crate::core::TableType::Character,
            },
            engine.bytes.byte_index(),
        );
    }
    pub(crate) fn push_modifier_flags<T: TagWriter>(&self, engine: &mut SerializeEngine<T>) {
        let mut modifier_flags = 0b00000000;
        if self.use_advance_x {
            modifier_flags |= 0b00000001;
        }
        if self.use_pixmap_index {
            modifier_flags |= 0b00000010;
        }

        engine.bytes.push(modifier_flags);
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
            ],
            engine.bytes.byte_index(),
        );
    }
    pub(crate) fn push_configurations<T: TagWriter>(&self, engine: &mut SerializeEngine<T>) {
        #[cfg(feature = "tagging")]
        let configurations_start = engine.bytes.byte_index();

        let mut configuration_flags = 0b00000000;
        if self.constant_cluster_codepoints.is_some() {
            configuration_flags |= 0b00000001;
        }

        engine.bytes.push(configuration_flags); // Configuration flags byte
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::CharacterTableConfigurationFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![TagKind::CharacterTableUseConstantClusterCodepoints {
                table_index: engine.tagging_data.current_table_index,
                value: self.constant_cluster_codepoints.is_some(),
            }],
            engine.bytes.byte_index(),
        );

        // Configuration values
        #[cfg(feature = "tagging")]
        let configuration_values_start = engine.bytes.byte_index();

        if self.constant_cluster_codepoints.is_some() {
            engine.bytes.push(self.constant_cluster_codepoints.unwrap());
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

    pub(crate) fn push_table_links<T: TagWriter>(
        &self,
        engine: &mut SerializeEngine<T>,
    ) -> Result<(), SerializeError> {
        #[cfg(feature = "tagging")]
        let links_start = engine.bytes.byte_index();

        // Table Links
        let mut link_flags = 0b00000000;
        if self.pixmap_table_indexes.is_some() {
            link_flags |= 0b00000001;
        }

        // Table relations
        engine.bytes.push(link_flags);
        #[cfg(feature = "tagging")]
        engine.tags.tag_bitflag(
            TagKind::CharacterTableLinkFlags {
                table_index: engine.tagging_data.current_table_index,
            },
            vec![TagKind::CharacterTableLinkPixmapTables {
                table_index: engine.tagging_data.current_table_index,
                value: self.pixmap_table_indexes.is_some(),
            }],
            engine.bytes.byte_index(),
        );

        if let Some(pixmap_table_indexes) = &self.pixmap_table_indexes {
            #[cfg(feature = "tagging")]
            let pixmap_tables_start = engine.bytes.byte_index();

            let pixmap_tables_length = pixmap_table_indexes.len();
            if pixmap_tables_length > 255 {
                return Err(SerializeError::StaticVectorTooLarge);
            }

            engine.bytes.push(pixmap_tables_length as u8);
            #[cfg(feature = "tagging")]
            engine.tags.tag_byte(
                TagKind::CharacterTablePixmapTableIndexesLength {
                    table_index: engine.tagging_data.current_table_index,
                    count: pixmap_tables_length as u8,
                },
                engine.bytes.byte_index(),
            );

            #[cfg(feature = "tagging")]
            let pixmap_table_indexes_start = engine.bytes.byte_index();

            for pixmap_table_index in pixmap_table_indexes {
                engine.bytes.push(*pixmap_table_index);
                #[cfg(feature = "tagging")]
                engine.tags.tag_byte(
                    TagKind::CharacterTablePixmapTableIndex {
                        table_index: engine.tagging_data.current_table_index,
                        index: *pixmap_table_index,
                    },
                    engine.bytes.byte_index(),
                );
            }

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

pub(crate) fn push_grapheme_cluster<T: TagWriter>(
    engine: &mut SerializeEngine<T>,
    constant_cluster_codepoints: Option<u8>,
    string: &String,
) {
    #[cfg(feature = "log")]
    let mut string_bit_string = String::new();

    #[cfg(feature = "tagging")]
    let string_start = engine.bytes.byte_index();

    string.bytes().for_each(|byte| {
        engine.bytes.push(byte);
        #[cfg(feature = "log")]
        string_bit_string.push_str(&format!("{:08b} ", byte));
    });
    if constant_cluster_codepoints.is_none() {
        engine.bytes.push(0);
        #[cfg(feature = "log")]
        string_bit_string.push_str(&format!("{:08b} ", 0));
    }

    #[cfg(feature = "tagging")]
    engine.tags.tag_span(
        TagKind::CharacterGraphemeCluster {
            table_index: engine.tagging_data.current_table_index,
            char_index: engine.tagging_data.current_record_index,
            value: string.clone(),
        },
        Span::new(string_start, engine.bytes.byte_index()),
    );

    #[cfg(feature = "log")]
    info!(
        "Pushed grapheme cluster '{}' with the following bits: {}",
        string, string_bit_string
    );
}
