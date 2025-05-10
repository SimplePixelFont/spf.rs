#[derive(Debug)]
pub(crate) struct ByteStorage {
    pub(crate) bytes: Vec<u8>,
    pub(crate) pointer: u8,
}

impl ByteStorage {
    pub(crate) fn new() -> Self {
        Self {
            bytes: vec![],
            pointer: 0,
        }
    }
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
            pointer: 0,
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

            let new_byte = byte >> 8 - self.pointer;
            self.bytes.push(new_byte);
        }
    }
    pub(crate) fn incomplete_push(&mut self, byte: u8, remainder: u8) {
        if self.pointer == 0 {
            self.bytes.push(byte);
        } else {
            let mask = byte << self.pointer;
            let last_index = self.bytes.len() - 1;
            self.bytes[last_index] |= mask;

            if self.pointer + (8 - remainder) > 8 {
                let new_byte = byte >> 8 - self.pointer;
                self.bytes.push(new_byte);
            }
        }
    }

    pub(crate) fn get(&self, index: usize) -> u8 {
        if self.pointer == 0 {
            self.bytes[index]
        } else {
            let mut byte = self.bytes[index] >> self.pointer;

            if index < self.bytes.len() - 1 {
                let mask = self.bytes[index] << 8 - self.pointer;
                byte |= mask;
            }
            byte
        }
    }
}
