//! Miscellaneous Rust utilities
//!
//! [Repository](https://github.com/spearman/rs-utils)

#![feature(decl_macro)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod file;
pub mod log;
pub mod numeric;

pub mod macros;
pub use self::macros::*;
