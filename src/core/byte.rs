#[derive(Debug, Clone, Copy)]
pub(crate) struct Byte {
    pub(crate) bits: [bool; 8],
}

impl Byte {
    pub(crate) fn to_u8(self) -> u8 {
        let mut value: u8 = 0;
        let mut modifier = 1;
        for bit in self.bits.iter() {
            if *bit {
                value += modifier as u8;
            }
            modifier *= 2;
        }
        value
    }

    pub(crate) fn from_u8(value: u8) -> Self {
        let mut bits: [bool; 8] = [false; 8];

        for (index, bit) in bits.iter_mut().enumerate() {
            *bit = ((1 << index) & value) > 0;
        }

        Self { bits }
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
        buffer
    }
    // Dev Comment: 0 0 0 0 0 0 0 0
    //       Index: 0 1 2 3 4 5 6 7
    // Ex write with pointer = 2:
    //              0 0 0 0 0 0 0 0
    //              (   left  ) (right)
    pub(crate) fn push(&mut self, byte: Byte) {
        if self.pointer == 0 {
            self.bytes.push(byte);
        } else {
            let left = byte.bits[0..8 - self.pointer].to_vec();
            let mut new_byte = byte.bits[8 - self.pointer..8].to_vec();
            let last_index = self.bytes.len() - 1;

            for (index, bit) in left.into_iter().enumerate() {
                self.bytes[last_index].bits[self.pointer + index] = bit;
            }

            new_byte.extend(vec![false; 8 - new_byte.len()]);
            self.bytes.push(Byte {
                bits: new_byte.try_into().unwrap(),
            });
        }
    }
    pub(crate) fn incomplete_push(&mut self, byte: Byte, remainder: usize) {
        if self.pointer == 0 {
            self.bytes.push(byte);
        } else {
            let left = byte.bits[0..8 - self.pointer].to_vec();
            let last_index = self.bytes.len() - 1;

            for (index, bit) in left.into_iter().enumerate() {
                self.bytes[last_index].bits[self.pointer + index] = bit;
            }

            if self.pointer + (8 - remainder) > 8 {
                let mut new_byte = byte.bits[8 - self.pointer..8].to_vec();
                new_byte.extend(vec![false; 8 - new_byte.len()]);

                self.bytes.push(Byte {
                    bits: new_byte.try_into().unwrap(),
                });
            }
        }
    }

    pub(crate) fn get(&self, index: usize) -> Byte {
        if self.pointer == 0 {
            self.bytes[index]
        } else {
            let mut left = self.bytes[index].bits[self.pointer..8].to_vec();
            let mut right = vec![];
            if index < self.bytes.len() - 1 {
                right = self.bytes[index + 1].bits[0..self.pointer].to_vec();
            } else {
                right.extend(vec![false; self.pointer]);
            }

            left.append(&mut right);
            Byte {
                bits: left.try_into().unwrap(),
            }
        }
    }
}
