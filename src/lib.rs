//! Miscellaneous Rust utilities
//!
//! [Repository](https://github.com/spearman/rs-utils)

#![feature(decl_macro)]
#![cfg_attr(feature = "app", feature(pattern))]

extern crate generic_array;
extern crate typenum;

#[cfg(test)] extern crate quickcheck;
#[cfg(test)] extern crate quickcheck_macros;
#[cfg(test)] extern crate tempfile;

pub mod array;
pub mod file;
pub mod numeric;

pub mod macros;
pub use self::macros::*;

#[cfg(feature = "app")] pub mod app;
#[cfg(feature = "app")] pub use self::app::App;
