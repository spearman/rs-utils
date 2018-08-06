//! Miscellaneous Rust utilities
//!
//! [Repository](https://github.com/spearman/rs-utils)

#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#![feature(custom_attribute)]

#[macro_use] extern crate lazy_static;

extern crate generic_array;
extern crate rand;
extern crate typenum;

#[cfg(test)] extern crate quickcheck;

#[macro_use] pub mod macros;
pub mod array;
pub mod file;
pub mod numeric;
pub mod process;
