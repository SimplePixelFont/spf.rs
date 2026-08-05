# Tagging with `spf.rs`

### Synopsis
The `tagging` feature, which is enabled by default, records the exact byte and bit range of every field `spf.rs` writes or reads, tagged by what it is. This is useful for inspecting a `.spf` file's structure: which bytes belong to which table, record, or field, right down to individual bits.

### Turning tagging on
Construct a [`core::DeserializeEngine`] with a [`tagging::TagWriterImpl`] instead of the default no-op writer, using [`core::DeserializeEngine::from_data_and_tags`]:

```rs
use spf::core::*;
use spf::tagging::TagWriterImpl;

let mut engine = DeserializeEngine::from_data_and_tags(&buffer, TagWriterImpl::default());
deserialize_with_engine(&mut engine).unwrap();

println!("{}", engine.tags);
```

[`tagging::TagWriterImpl`] implements Rust's standard `Display` trait, printing one line per tag. Using the following format: [`tagging::TagKind`] and its `start_byte:start_bit - end_byte:end_bit` span. Serializing works the same way — [`core::SerializeEngine`] takes a [`tagging::TagWriter`] too.

### Reading the output
Here's real output from parsing `res/sampleToyFont.spf`, the file used in `spf.rs`'s own integration tests. Only the first few lines and a `CharacterTable` are shown:

```text
Signature 0:0 - 4:0
Version  4:0 - 5:0
CompactFlag  5:0 - 5:1
Reserved 5:1 - 6:0
Header 5:0 - 6:0
TableIdentifier  6:0 - 7:0
CharacterTableUseAdvanceX  7:0 - 7:1
CharacterTableUsePixmapIndex  7:1 - 7:2
CharacterTableUsePixmapTableIndex  7:2 - 7:3
Reserved 7:3 - 8:0
CharacterTableModifierFlags  7:0 - 8:0
...
CharacterCodePoints  13:0 - 15:0
CharacterRecord  13:0 - 15:0
...
CharacterTable  6:0 - 25:0
```

Look at the spans, not just the order. `Header 5:0 - 6:0` contains both `CompactFlag 5:0 - 5:1` and `Reserved 5:1 - 6:0`. Here, the flag bit and the reserved bits share the Header byte span. Further down, `CharacterTable 6:0 - 25:0` contains every tag between it and `TableIdentifier`, including `CharacterRecord 13:0 - 15:0`, which itself contains `CharacterCodePoints 13:0 - 15:0`. Tags nest by span containment: a table's span contains its records' spans, which contain their fields' spans, down to individual configuration bits.

Note that this isn't a tree in the data structure, [`tagging::TagWriterImpl::tags`] is a flat `Vec<`[`tagging::Tag`]`>` stored in write/read order.

### Finding a specific field
Since [`tagging::Tag`] just pairs a [`tagging::TagKind`] with a [`tagging::Span`], filtering for what you want is a normal iterator operation:

```rs
let widths: Vec<_> = engine.tags.tags.iter()
    .filter(|tag| matches!(tag.kind, spf::tagging::TagKind::PixmapCustomWidth { .. }))
    .collect();
```

### Cost when disabled
If you don't need tags, use [`core::DeserializeEngine::from_data`] instead as it defaults to [`tagging::TagWriterNoOp`], whose [`tagging::TagWriter`] methods are empty function bodies the compiler removes entirely. There's no runtime cost for tagging you don't use.
