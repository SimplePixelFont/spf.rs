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

use std::cell::RefCell;
use std::rc::Rc;

pub(crate) use crate::core::*;
pub(crate) use crate::ergonomics::*;

// remove ToString trait
use crate::Vec;

impl From<&[u8]> for PixmapBuilder {
    fn from(data: &[u8]) -> Self {
        PixmapBuilder {
            custom_width: None,
            custom_height: None,
            custom_bits_per_pixel: None,
            data: data.to_vec(),
            index: None,
        }
    }
}

impl From<&mut PixmapBuilder> for PixmapBuilder {
    fn from(pixmap: &mut PixmapBuilder) -> Self {
        PixmapBuilder {
            custom_width: pixmap.custom_width,
            custom_height: pixmap.custom_height,
            custom_bits_per_pixel: pixmap.custom_bits_per_pixel,
            data: pixmap.data.clone(),
            index: pixmap.index.clone(),
        }
    }
}

impl PixmapBuilder {
    pub fn custom_width(&mut self, custom_width: u8) -> &mut Self {
        self.custom_width = Some(custom_width);
        self
    }
    pub fn custom_height(&mut self, custom_height: u8) -> &mut Self {
        self.custom_height = Some(custom_height);
        self
    }
    pub fn custom_bits_per_pixel(&mut self, custom_bits_per_pixel: u8) -> &mut Self {
        self.custom_bits_per_pixel = Some(custom_bits_per_pixel);
        self
    }
    pub fn data(&mut self, data: &[u8]) -> &mut Self {
        self.data = data.to_vec();
        self
    }
}

impl PixmapTableBuilder {
    pub fn constant_width(&mut self, constant_width: u8) -> &mut Self {
        self.constant_width = Some(constant_width);
        self
    }
    pub fn constant_height(&mut self, constant_height: u8) -> &mut Self {
        self.constant_height = Some(constant_height);
        self
    }
    pub fn constant_bits_per_pixel(&mut self, constant_bits_per_pixel: u8) -> &mut Self {
        self.constant_bits_per_pixel = Some(constant_bits_per_pixel);
        self
    }
    pub fn color_table_indexes(&mut self, color_table_indexes: &[ColorTableIndex]) -> &mut Self {
        if self.color_table_indexes.is_none() {
            self.color_table_indexes = Some(Vec::new());
        }
        self.color_table_indexes
            .as_mut()
            .unwrap()
            .append(color_table_indexes.to_vec().as_mut());
        self
    }
    pub fn pixmap<T: Into<PixmapBuilder>>(&mut self, pixmap: T) -> &mut Self {
        let mut pixmap = pixmap.into();
        pixmap.index = Some(PixmapIndex(
            self.link(),
            Rc::new(RefCell::new(self.pixmaps.len() as u8)),
        ));
        self.pixmaps.push(pixmap);
        self
    }
    pub fn pixmap_bind<T: Into<PixmapBuilder>>(
        &mut self,
        pixmap: T,
        pixmap_index: &mut PixmapIndex,
    ) -> &mut Self {
        self.pixmap(pixmap);
        *pixmap_index = self.pixmaps.last().unwrap().index.as_ref().unwrap().clone();
        self
    }
    pub fn pixmap_process<T: Into<PixmapBuilder>>(
        &mut self,
        pixmap: T,
        process: fn(&mut PixmapBuilder) -> &mut PixmapBuilder,
    ) -> &mut Self {
        self.pixmap(process(&mut pixmap.into()));
        self
    }
    pub fn bind_pixmap<T: Into<PixmapBuilder>>(&mut self, pixmap: T) -> PixmapIndex {
        self.pixmap(pixmap);
        self.pixmaps.last().unwrap().index.as_ref().unwrap().clone()
    }
    pub fn link(&mut self) -> PixmapTableIndex {
        self.index.clone()
    }
}
