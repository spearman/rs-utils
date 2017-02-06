#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#![feature(custom_attribute)]

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use]
extern crate unwrap;

pub mod file;
