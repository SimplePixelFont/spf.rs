#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_playground_url = "https://play.rust-lang.org")]
#![doc(html_logo_url = "https://github.com/The-Nice-One/spf.rs/blob/main/res/spf.rs.png")]

pub mod core;

#[cfg_attr(docsrs, doc(cfg(feature = "log")))]
#[cfg(feature = "log")]
pub mod log;

#[cfg_attr(docsrs, doc(cfg(feature = "cache")))]
#[cfg(feature = "cache")]
pub mod cache;

#[cfg_attr(docsrs, doc(cfg(feature = "printer")))]
#[cfg(feature = "printer")]
pub mod printer;

#[cfg_attr(docsrs, doc(cfg(feature = "ergonomics")))]
#[cfg(feature = "ergonomics")]
pub mod ergonomics;

#[cfg_attr(docsrs, doc(cfg(feature = "ffi")))]
#[cfg(feature = "ffi")]
pub mod c;

#[cfg_attr(docsrs, doc(cfg(feature = "articles")))]
#[cfg(feature = "articles")]
pub mod articles;
