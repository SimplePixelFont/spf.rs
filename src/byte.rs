#[derive(Debug)]
pub(crate) struct Byte {
    pub(crate) bits: [bool; 8],
}

impl Byte {
    pub(crate) fn to_u8(&self) -> u8 {
        let mut value: u8 = 0;
        let mut modifier = 1;
        for bit in self.bits.iter() {
            if bit.clone() {
                value += modifier as u8;
            }
            modifier *= 2;
        }
        return value;
    }

    pub(crate) fn from_u8(value: u8) -> Self {
        let mut bits: [bool; 8] = [false; 8];
        for i in 0..8 {
            let bit: bool = ((1 << i) & value) > 0;
            bits[i] = bit;
        }
        return Self { bits: bits };
    }
}

#[derive(Debug)]
pub(crate) struct ByteStorage {
    pub(crate) bytes: Vec<Byte>,
    pub(crate) pointer: usize,
}

impl ByteStorage {
    pub(crate) fn new() -> Self {
        Self {
            bytes: vec![],
            pointer: 0,
        }
    }
    pub(crate) fn to_vec_u8(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        for byte in self.bytes.iter() {
            buffer.push(byte.to_u8());
        }
        return buffer;
    }
    pub(crate) fn push(&mut self, byte: Byte) {
        if self.pointer == 0 || self.pointer == 8 {
            self.bytes.push(byte);
        } else {
            let left = byte.bits[0..self.pointer + 1].to_vec();
            let mut new_byte = byte.bits[self.pointer + 1..8].to_vec();
            let last_index = self.bytes.len() - 1;

            let mut index = 0;
            for bit in left {
                self.bytes[last_index].bits[self.pointer + index] = bit;
                index += 1;
            }

            for _ in 0..8 - new_byte.len() {
                new_byte.push(false);
            }
        }
    }
    pub(crate) fn get(&self, index: usize) -> Byte {
        let mut left = self.bytes[index].bits[self.pointer + 1..8].to_vec();
        let mut right = self.bytes[index].bits[0..self.pointer + 1].to_vec();
        left.append(&mut right);
        Byte {
            bits: left.try_into().unwrap(),
        }
    }
}
