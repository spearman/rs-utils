#[macro_use] extern crate unwrap;

extern crate rand;
extern crate tempdir;

extern crate rs_utils;

use rs_utils::*;

fn main () {
  let exe_name = std::env::current_exe().unwrap().file_name().unwrap()
    .to_owned();
  println!("{:?} main...", exe_name);

  println!("{}", String::from_utf8 (vec![b'#'; 80]).unwrap());
  println!("example: incrementally named files");
  let temp_dir = unwrap!{ tempdir::TempDir::new_in ("examples", "tmp") };
  let file_path = temp_dir.path().join (
    std::path::Path::new ("myfile"));
  let file_path = file_path.as_path();
  println!("file_path_incremental\n  \"{}\":\n  {}", file_path.display(),
    unwrap!{ file::file_path_incremental (file_path) }.display());
  println!("file_new_append_incremental \"{}\"...", file_path.display());
  unwrap!{ file::file_new_append_incremental (file_path) };
  println!("file_path_incremental\n  \"{}\":\n  {}", file_path.display(),
    unwrap!{ file::file_path_incremental (file_path) }.display());
  println!("file_new_append_incremental \"{}\"...", file_path.display());
  unwrap!{ file::file_new_append_incremental (file_path) };
  println!("file_path_incremental\n  \"{}\":\n  {}", file_path.display(),
    unwrap!{ file::file_path_incremental (file_path) }.display());
  println!("file_path_incremental and then file_new_append\n  \"{}\"...",
    file_path.display());
  unwrap!{
    // file_new_append_incremental is equivalent to:
    file::file_path_incremental (file_path).and_then (
      |path_buf| file::file_new_append (path_buf.as_path())
    )
  };
  println!("!ls {}", temp_dir.path().display());
  assert!{
    unwrap!{
      std::process::Command::new ("ls").arg (temp_dir.path().as_os_str())
        .status()
    }.success()
  };

  println!("{}", String::from_utf8 (vec![b'#'; 80]).unwrap());
  println!("example: default xorshift rng");
  use rand::Rng;
  let mut xorshiftrng = numeric::xorshift_rng_unseeded();
  let b = xorshiftrng.gen::<u8>();
  println!("b: {:?}", b);

  println!("...{:?} main", exe_name);
}
