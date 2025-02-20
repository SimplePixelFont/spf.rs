#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(html_logo_url = "https://github.com/The-Nice-One/spf.rs/blob/main/res/spf.rs.png")]

#[cfg_attr(docsrs, doc(cfg(feature = "log")))]
#[cfg(feature = "log")]
pub mod log;

#[cfg_attr(docsrs, doc(cfg(feature = "core")))]
#[cfg(feature = "core")]
pub mod core;

#[cfg_attr(docsrs, doc(cfg(feature = "cache")))]
#[cfg(feature = "cache")]
pub mod cache;

#[cfg_attr(docsrs, doc(cfg(feature = "printer")))]
#[cfg(feature = "printer")]
pub mod printer;

#[cfg_attr(docsrs, doc(cfg(feature = "articles")))]
#[cfg(feature = "articles")]
pub mod articles;

pub(crate) mod byte;

pub mod c;

/// Magic bytes of `*.spf` files
///
/// The magic bytes can be used to determine if a file is a SimplePixelFont regardless of
/// the file extension. That being said the magic bytes as u8 are are follows: `102, 115, 70`.
/// In utf8 encoding this spells out `fsF`.
pub const MAGIC_BYTES: [u8; 3] = [102, 115, 70];
