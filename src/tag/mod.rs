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

pub struct ByteIndex {
    pub byte: usize,
    pub bit: usize,
}

pub struct Span {
    pub start: ByteIndex,
    pub end: ByteIndex,
}

pub enum TagValue {
    MagicBytes(Vec<u8>),
    Version(u8),
    Header(u8),
}

pub enum TagType {
    MagicBytes,
    Version,
    Header,
    Table,
}
