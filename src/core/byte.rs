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

use crate::Vec;

#[derive(Debug)]
pub(crate) struct ByteStorage {
    pub(crate) bytes: Vec<u8>,
    pub(crate) pointer: u8,
    pub(crate) index: usize,
}

impl ByteStorage {
    pub(crate) fn new() -> Self {
        Self {
            bytes: Vec::new(),
            pointer: 0,
            index: 0,
        }
    }

    // Dev Comment: 0 0 0 0 0 0 0 0
    //       Index: 7 6 5 4 3 2 1 0
    pub(crate) fn push(&mut self, byte: u8) {
        if self.pointer == 0 {
            self.bytes.push(byte);
        } else {
            let mask = byte << self.pointer;
            let last_index = self.bytes.len() - 1;
            self.bytes[last_index] |= mask;

            let new_byte = byte >> (8 - self.pointer);
            self.bytes.push(new_byte);
        }
    }
    pub(crate) fn incomplete_push(&mut self, byte: u8, number_of_bits: u8) {
        if number_of_bits == 8 {
            self.push(byte);
            return;
        }
        if self.pointer == 0 {
            self.bytes.push(byte);
            self.pointer += number_of_bits;
            return;
        }

        let mut mask = byte << self.pointer;

        // Sanitizes the mask to ensure it doesn't affect other bits in the byte
        if self.pointer + number_of_bits < 8 {
            let shift = 8 - self.pointer - number_of_bits;
            mask = mask << shift >> shift;
        }
        let last_index = self.bytes.len() - 1;
        self.bytes[last_index] |= mask;
        self.pointer += number_of_bits;

        if self.pointer >= 8 {
            self.pointer -= 8;
            if self.pointer != 0 {
                let new_byte = byte >> (number_of_bits - self.pointer);
                self.bytes.push(new_byte);
            }
        }
    }

    pub(crate) fn get(&self) -> u8 {
        if self.pointer == 0 {
            self.bytes[self.index]
        } else {
            let mut byte = self.bytes[self.index] >> self.pointer;

            if self.index < self.bytes.len() - 1 {
                let mask = self.bytes[self.index + 1] << (8 - self.pointer);
                byte |= mask;
            }

            byte
        }
    }
    pub(crate) fn incomplete_get(&self, number_of_bits: u8) -> u8 {
        if number_of_bits == 8 {
            return self.get();
        }
        self.get() << (8 - number_of_bits) >> (8 - number_of_bits)
    }
    pub(crate) fn next(&mut self) -> u8 {
        let byte = self.get();
        self.index += 1;
        byte
    }
    pub(crate) fn peek(&self) -> u8 {
        self.bytes[self.index]
    }
    pub(crate) fn append(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.push(*byte);
        }
    }
}

pub(crate) fn get_bit(byte: u8, index: u8) -> bool {
    (byte >> (index)) & 0b00000001 == 1
}
