struct Font {
    meta_table: Option<MetaTable>,
    mapping_tables: Vec<MappingTable>,
    palette_tables: Vec<PaletteTable>,
    bitmap_tables: Vec<BitmapTable>,

    debug: bool,
    compact: bool,
}

struct MetaTable {
    index: u16,
    name: String,
    checksum: u32,
    author: String,
    description: String,
    version: String,
}

struct BitmapTable {
    constant_width: Option<u16>,
    constant_height: Option<u16>,
    custom_bits_per_pixel: Option<u8>,

    palette_table_index: u16,
    index: u16,
}

struct Bitmap {
    data: Vec<u8>,
}

struct MappingTable {
    bitmap_table_index: u16,
    mappings: HashMap<String, u16>,
}

struct PaletteTable {
    index: u16,
    use_alpha_channel: bool,
    colors: HashMap<u8, Vec<u8>>,
}
