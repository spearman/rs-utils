#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#![feature(custom_attribute)]

#[macro_use] extern crate unwrap;
#[macro_use] extern crate lazy_static;

#[cfg(test)] extern crate quickcheck;

pub mod file;
pub mod process;
