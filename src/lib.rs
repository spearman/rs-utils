#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#![feature(custom_attribute)]
#![feature(const_fn)]

#[macro_use] extern crate unwrap;
#[macro_use] extern crate lazy_static;

#[cfg(test)]
#[macro_use] extern crate macro_attr;
#[cfg(test)]
#[macro_use] extern crate enum_derive;

#[cfg(test)] extern crate quickcheck;
#[cfg(test)] extern crate num;

pub mod enum_unitary;
pub mod file;
pub mod process;

pub use enum_unitary::EnumUnitary;
