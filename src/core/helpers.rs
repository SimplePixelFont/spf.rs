pub(crate) use super::*;

pub(crate) fn sign_buffer(buffer: &mut byte::ByteStorage) -> &mut byte::ByteStorage {
    buffer.bytes.insert(0, 70);
    buffer.bytes.insert(0, 115);
    buffer.bytes.insert(0, 102);

    #[cfg(feature = "log")]
    info!("Signed font data.");

    buffer
}
