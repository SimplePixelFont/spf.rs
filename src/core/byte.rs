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
pub(crate) struct ByteWriter {
    pub(crate) bytes: Vec<u8>,
    pub(crate) pointer: u8,
    pub(crate) index: usize,
}

impl ByteWriter {
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
        self.index += 1;
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
            self.index += 1;
        }
    }
}

pub trait ByteReader {
    fn get(&self) -> u8;
    fn incomplete_get(&self, number_of_bits: u8) -> u8;
    fn next(&mut self) -> u8; // maybe Option<u8>
    fn incomplete_next(&mut self, number_of_bits: u8) -> u8;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn index(&self) -> usize;

    #[cfg(feature = "tagging")]
    fn byte_index(&self) -> super::ByteIndex;
}

#[derive(Debug)]
pub struct ByteReaderImpl<'a> {
    bytes: &'a [u8],
    pointer: u8,
    index: usize,
}

impl<'a> ByteReaderImpl<'a> {
    pub(crate) fn from(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            pointer: 0,
            index: 0,
        }
    }
}

impl<'a> ByteReader for ByteReaderImpl<'a> {
    fn get(&self) -> u8 {
        if self.pointer == 0 {
            self.bytes[self.index]
        } else {
            let mut byte = self.bytes[self.index] >> self.pointer;

            if self.index < self.len() - 1 {
                let mask = self.bytes[self.index + 1] << (8 - self.pointer);
                byte |= mask;
            }

            byte
        }
    }
    fn incomplete_get(&self, number_of_bits: u8) -> u8 {
        if number_of_bits == 8 {
            return self.get();
        }
        self.get() << (8 - number_of_bits) >> (8 - number_of_bits)
    }
    fn next(&mut self) -> u8 {
        let byte = self.get();
        self.index += 1;
        byte
    }
    fn incomplete_next(&mut self, number_of_bits: u8) -> u8 {
        let byte = self.incomplete_get(number_of_bits);
        self.pointer += number_of_bits;
        if self.pointer >= 8 {
            self.index += 1;
            self.pointer -= 8;
        }
        byte
    }
    fn len(&self) -> usize {
        self.bytes.len()
    }
    fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
    fn index(&self) -> usize {
        self.index
    }

    #[cfg(feature = "tagging")]
    fn byte_index(&self) -> super::ByteIndex {
        super::ByteIndex::new(self.index, self.pointer)
    }
}

#[derive(Debug)]
pub struct ByteReaderIter<'a, I: Iterator<Item = u8>> {
    iterator: &'a mut I,
    buffered_bytes: [u8; 2],
    pointer: u8,
    index: usize,
    length: usize,
}

impl<'a, I: Iterator<Item = u8>> ByteReaderIter<'a, I> {
    pub fn from(iterator: &'a mut I, length: usize) -> Self {
        let mut reader = Self {
            iterator,
            buffered_bytes: [0, 0],
            pointer: 0,
            index: 0,
            length,
        };
        reader.init_buffer();
        reader
    }
    fn init_buffer(&mut self) {
        if let Some(item) = self.iterator.next() {
            self.buffered_bytes[0] = item;
        }
        if let Some(item) = self.iterator.next() {
            self.buffered_bytes[1] = item;
        }
    }
    fn proceed_iter(&mut self) {
        self.buffered_bytes[0] = self.buffered_bytes[1];
        if let Some(item) = self.iterator.next() {
            self.buffered_bytes[1] = item;
        } else {
            self.buffered_bytes[1] = 0;
        }
    }
}

impl<'a, I: Iterator<Item = u8>> ByteReader for ByteReaderIter<'a, I> {
    fn get(&self) -> u8 {
        if self.pointer == 0 {
            self.buffered_bytes[0]
        } else {
            let mut byte = self.buffered_bytes[0] >> self.pointer;
            if self.index < self.len() - 1 {
                let mask = self.buffered_bytes[1] << (8 - self.pointer);
                byte |= mask;
            }

            byte
        }
    }
    fn incomplete_get(&self, number_of_bits: u8) -> u8 {
        if number_of_bits == 8 {
            return self.get();
        }
        self.get() << (8 - number_of_bits) >> (8 - number_of_bits)
    }
    fn next(&mut self) -> u8 {
        let byte = self.get();
        self.index += 1;
        self.proceed_iter();
        byte
    }
    fn incomplete_next(&mut self, number_of_bits: u8) -> u8 {
        let byte = self.incomplete_get(number_of_bits);
        self.pointer += number_of_bits;
        if self.pointer >= 8 {
            self.index += 1;
            self.proceed_iter();
            self.pointer -= 8;
        }
        byte
    }
    fn len(&self) -> usize {
        self.length
    }
    fn is_empty(&self) -> bool {
        self.length == 0
    }
    fn index(&self) -> usize {
        self.index
    }

    #[cfg(feature = "tagging")]
    fn byte_index(&self) -> super::ByteIndex {
        super::ByteIndex::new(self.index, self.pointer)
    }
}

pub(crate) fn get_bit(byte: u8, index: u8) -> bool {
    (byte >> (index)) & 0b00000001 == 1
}
