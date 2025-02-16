pub use std::io::{Read, Write};

pub fn write_to_file(path: &'static str, data: &[u8]) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;
    file.write_all(data)?;
    Ok(())
}

pub fn read_from_file(path: &'static str, buffer: &mut Vec<u8>) -> std::io::Result<()> {
    let mut file = std::fs::OpenOptions::new().read(true).open(path)?;
    file.read_to_end(buffer)?;
    Ok(())
}
