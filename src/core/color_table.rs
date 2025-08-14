use crate::core::{Color, ColorTable, Table};

pub(crate) fn get_bit(byte: u8, index: u8) -> bool {
    (byte & 0b00000001) >> (index) == 1
}

#[repr(u8)]
enum TableIdentifier {
    ColorTable = 0b00000001,
}

impl Table for ColorTable {
    fn deserialize(
        storage: &mut crate::core::byte::ByteStorage,
    ) -> Result<Self, crate::core::ParseError> {
        let mut color_table = ColorTable::default();

        let table_property_flags = storage.get();

        if get_bit(table_property_flags, 0) {
            storage.index += 1;
            color_table.constant_alpha = Some(storage.get());
        }
        storage.index += 1;

        let color_count = storage.get();
        for _ in 0..color_count {
            let mut color = Color::default();
            storage.index += 1;
            if color_table.constant_alpha.is_none() {
                color.custom_alpha = Some(storage.get());
            }
            storage.index += 1;
            color.r = storage.get();
            storage.index += 1;
            color.g = storage.get();
            storage.index += 1;
            color.b = storage.get();
            color_table.colors.push(color);
        }

        Ok(color_table)
    }

    fn serialize(
        &self,
        buffer: &mut crate::core::byte::ByteStorage,
    ) -> Result<(), crate::core::SerializeError> {
        buffer.push(TableIdentifier::ColorTable as u8);

        let mut table_property_flags = 0b00000000;
        let mut table_property_values = Vec::new();

        if self.constant_alpha.is_some() {
            table_property_flags |= 0b00000001;
            table_property_values.push(self.constant_alpha.unwrap());
        }

        buffer.push(table_property_flags);
        for byte in table_property_values {
            buffer.push(byte);
        }

        buffer.push(self.colors.len() as u8);
        for color in &self.colors {
            if self.constant_alpha.is_none() {
                buffer.push(color.custom_alpha.unwrap());
            }
            buffer.push(color.r);
            buffer.push(color.g);
            buffer.push(color.b);
        }

        Ok(())
    }
}
