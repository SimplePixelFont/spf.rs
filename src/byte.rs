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

pub(crate) fn three_byte_checksum(data: &[u8]) -> [u8; 3] {
    let mut checksum: u32 = 0;
    for byte in data {
        checksum += *byte as u32;
        checksum = (checksum >> 8) | (checksum << 24);
    }
    checksum = checksum & 0xFFFFFF;

    checksum.to_be_bytes()[1..].try_into().unwrap()
}
