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
