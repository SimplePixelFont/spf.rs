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

impl ColorTableBuilder {
    pub fn constant_alpha(&mut self, constant_alpha: u8) -> &mut Self {
        self.constant_alpha = Some(constant_alpha);
        self
    }
    pub fn color<T: Into<ColorBuilder>>(&mut self, color: T) -> &mut Self {
        // if self.constant_alpha.is_some() {
        //     color.custom_alpha = None;
        // }
        // if self.constant_alpha.is_none() && color.custom_alpha.is_none() {
        //     panic!("Neither constant_alpha nor custom_alpha are set!");
        // }
        self.colors.push(color.into());
        self
    }
    pub fn link(&mut self) -> ColorTableIndex {
        self.index.clone()
    }
    // pub fn set_index(&mut self, index: u8) {
    //     *self.index.0.borrow_mut() = index;
    // }
}

impl TableBuilder for ColorTableBuilder {
    fn build(&mut self) -> Box<impl Table> {
        let mut colors = vec![];
        for color_builder in self.colors.iter_mut() {
            if self.constant_alpha.is_some() {
                color_builder.custom_alpha = None;
            }
            if self.constant_alpha.is_none() && color_builder.custom_alpha.is_none() {
                panic!("Neither constant_alpha nor custom_alpha are set!");
            }
            colors.push(Color {
                custom_alpha: color_builder.custom_alpha,
                r: color_builder.r,
                g: color_builder.g,
                b: color_builder.b,
            });
        }
        Box::new(ColorTable {
            constant_alpha: self.constant_alpha,
            colors,
        })
    }
}

impl From<&[u8]> for ColorBuilder {
    fn from(rgba: &[u8]) -> Self {
        ColorBuilder {
            custom_alpha: Some(rgba.get(3).unwrap_or(&0).to_owned()),
            r: rgba.get(0).unwrap_or(&0).to_owned(),
            g: rgba.get(1).unwrap_or(&0).to_owned(),
            b: rgba.get(2).unwrap_or(&0).to_owned(),
        }
    }
}

impl ColorBuilder {
    pub fn rgba(&mut self, r: u8, g: u8, b: u8, a: u8) -> &mut Self {
        self.custom_alpha = Some(a);
        self.r = r;
        self.g = g;
        self.b = b;
        self
    }
    pub fn rgb(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        self.custom_alpha = None;
        self.r = r;
        self.g = g;
        self.b = b;
        self
    }
    pub fn a(&mut self, a: u8) -> &mut Self {
        self.custom_alpha = Some(a);
        self
    }
    pub fn r(&mut self, r: u8) -> &mut Self {
        self.r = r;
        self
    }
    pub fn g(&mut self, g: u8) -> &mut Self {
        self.g = g;
        self
    }
    pub fn b(&mut self, b: u8) -> &mut Self {
        self.b = b;
        self
    }
    pub fn black() -> Self {
        ColorBuilder {
            custom_alpha: Some(255),
            r: 0,
            g: 0,
            b: 0,
        }
    }
    pub fn white() -> Self {
        ColorBuilder {
            custom_alpha: Some(255),
            r: 255,
            g: 255,
            b: 255,
        }
    }
    pub fn transparent() -> Self {
        ColorBuilder {
            custom_alpha: Some(0),
            r: 0,
            g: 0,
            b: 0,
        }
    }
}
