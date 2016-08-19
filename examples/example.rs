use std::path::Path;

extern crate rs_utils;
use rs_utils::{
  inc_file_name};

fn main () {
  println!("rs_utils example main...");

  println!("inc_file_name \"myfile\": {}",
    inc_file_name (Path::new ("myfile")).unwrap().display());

  println!("...rs_utils example main");
}
