//! Miscellaneous Rust utilities
//!
//! [Repository](https://github.com/spearman/rs-utils)

#![feature(decl_macro)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[expect(unused_extern_crates)]
extern crate log as log_;
#[expect(unused_imports)]
use stdext;

pub mod file;
pub mod log;
pub mod numeric;

pub mod macros;
pub use self::macros::*;
