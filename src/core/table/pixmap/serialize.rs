use crate::core::{ParseError, Pixmap, PixmapTable, SerializeError, Table, TableIdentifier};
use crate::Vec;

pub(crate) fn serialize(
    &self,
    buffer: &mut crate::core::byte::ByteStorage,
) -> Result<(), SerializeError> {
    buffer.push(TableIdentifier::PixmapTable as u8);

    buffer.push(0b00000000); // Modifiers Byte

    let mut table_property_flags = 0b00000000;
    let mut table_property_values = Vec::new();

    // Configuration flags
    if self.constant_width.is_some() {
        table_property_flags |= 0b00000001;
        table_property_values.push(self.constant_width.unwrap());
    }
    if self.constant_height.is_some() {
        table_property_flags |= 0b00000010;
        table_property_values.push(self.constant_height.unwrap());
    }
    if self.constant_bits_per_pixel.is_some() {
        table_property_flags |= 0b00000100;
        table_property_values.push(self.constant_bits_per_pixel.unwrap());
    }

    buffer.push(table_property_flags);
    buffer.append(&table_property_values);

    // Table Links
    let mut table_link_flags = 0b00000000;
    let mut table_link_bytes = Vec::new();
    if self.color_tables_indices.is_some() {
        table_link_flags |= 0b00000001;
        let colors_tables_length = self.color_tables_indices.as_ref().unwrap().len();
        if colors_tables_length > 255 {
            return Err(SerializeError::StaticVectorTooLarge);
        }
        buffer.push(colors_tables_length as u8);
        for table_index in self.color_tables_indices.as_ref().unwrap() {
            table_link_bytes.push(*table_index);
        }
    }

    // Table relations
    buffer.push(table_link_flags);
    buffer.append(&table_link_bytes);

    if self.pixmaps.len() > 255 {
        return Err(SerializeError::StaticVectorTooLarge);
    }
    buffer.push(self.pixmaps.len() as u8);
    for pixmap in self.pixmaps.iter() {
        push_width(buffer, self.constant_width, pixmap.custom_width);
        push_height(buffer, self.constant_height, pixmap.custom_height);
        push_bits_per_pixel(
            buffer,
            self.constant_bits_per_pixel,
            pixmap.custom_bits_per_pixel,
        );
        push_pixmap(
            buffer,
            true,
            self.constant_width,
            self.constant_height,
            self.constant_bits_per_pixel,
            pixmap,
        );
    }

    Ok(())
}

pub(crate) fn push_width<'a>(
    buffer: &mut byte::ByteStorage,
    constant_width: Option<u8>,
    custom_width: Option<u8>,
) {
    if constant_width.is_none() {
        let width = custom_width.unwrap();
        buffer.push(width);

        #[cfg(feature = "log")]
        {
            let width_bit_string = format!("{:08b}", width);
            info!(
                "Pushed character width '{}' with the following bits: {}",
                width, width_bit_string
            )
        }
    }
}

pub(crate) fn push_height(
    buffer: &mut byte::ByteStorage,
    constant_height: Option<u8>,
    custom_height: Option<u8>,
) {
    if constant_height.is_none() {
        let height = custom_height.unwrap();
        buffer.push(height);

        #[cfg(feature = "log")]
        {
            let height_bit_string = format!("{:08b}", height);
            info!(
                "Pushed character height '{}' with the following bits: {}",
                height, height_bit_string
            )
        }
    }
}

pub(crate) fn push_bits_per_pixel(
    buffer: &mut byte::ByteStorage,
    constant_bits_per_pixel: Option<u8>,
    custom_bits_per_pixel: Option<u8>,
) {
    if constant_bits_per_pixel.is_none() {
        let bits_per_pixel = custom_bits_per_pixel.unwrap();
        buffer.push(bits_per_pixel);

        #[cfg(feature = "log")]
        {
            let bits_per_pixel_bit_string = format!("{:08b}", bits_per_pixel);
            info!(
                "Pushed character bits_per_pixel '{}' with the following bits: {}",
                bits_per_pixel, bits_per_pixel_bit_string
            )
        }
    }
}

pub(crate) fn push_pixmap(
    buffer: &mut byte::ByteStorage,
    compact: bool,
    constant_width: Option<u8>,
    constant_height: Option<u8>,
    constant_bits_per_pixel: Option<u8>,
    pixmap: &Pixmap,
) -> Result<(), SerializeError> {
    let mut pixmap_bit_string = String::new();
    let mut bits_used = 0;

    let bits_per_pixel = constant_bits_per_pixel.unwrap_or(pixmap.custom_bits_per_pixel.unwrap());
    let width = constant_width.unwrap_or(pixmap.custom_width.unwrap());
    let height = constant_height.unwrap_or(pixmap.custom_height.unwrap());

    if pixmap.data.len() > width as usize * height as usize {
        return Err(SerializeError::StaticVectorTooLarge);
    }

    for pixel in pixmap.data.iter() {
        pixmap_bit_string.push_str(&format!(
            "{:0bits_per_pixel$b} ",
            pixel,
            bits_per_pixel = bits_per_pixel as usize
        ));
        buffer.incomplete_push(*pixel, bits_per_pixel);
        bits_used += bits_per_pixel;
    }

    if !compact {
        buffer.incomplete_push(0, 8 - (bits_used % 8));
    }

    #[cfg(feature = "log")]
    info!(
        "Pushed pixmap with the following bits: {}",
        pixmap_bit_string
    );
    Ok(())
}
