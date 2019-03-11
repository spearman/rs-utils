//! Miscellaneous Rust utilities
//!
//! [Repository](https://github.com/spearman/rs-utils)

#![feature(custom_attribute)]

extern crate generic_array;
extern crate rand_core;
extern crate rand_xorshift;
extern crate typenum;

#[cfg(test)] extern crate quickcheck;
#[cfg(test)] extern crate quickcheck_macros;
#[cfg(test)] extern crate tempdir;

#[macro_use] pub mod macros;
pub mod array;
pub mod file;
pub mod numeric;
