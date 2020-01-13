//! Miscellaneous Rust utilities
//!
//! [Repository](https://github.com/spearman/rs-utils)

extern crate generic_array;
extern crate typenum;

#[cfg(test)] extern crate quickcheck;
#[cfg(test)] extern crate quickcheck_macros;
#[cfg(test)] extern crate tempdir;

#[macro_use] pub mod macros;
pub mod array;
pub mod file;
pub mod numeric;
