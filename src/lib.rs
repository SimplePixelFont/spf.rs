#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod core;

#[cfg_attr(docsrs, doc(cfg(feature = "printer")))]
#[cfg(feature = "printer")]
pub mod printer;

pub(crate) mod byte;

/// Magic bytes of `*.spf` files
///
/// The magic bytes can be used to determine if a file is a SimplePixelFont regardless of
/// the file extension. That being said the magic bytes as u8 are are follows: `102, 115, 70`.
/// In utf8 encoding this spells out `fsF`.
pub const MAGIC_BYTES: [u8; 3] = [102, 115, 70];
