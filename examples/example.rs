#[macro_use]
extern crate unwrap;

extern crate rs_utils;

fn main () {
  println!("rs_utils example main...");

  let file_name = std::path::Path::new ("examples/myfile");

  println!("file_path_incremental \"{}\": {}", file_name.display(),
    unwrap!{
      rs_utils::file::file_path_incremental (file_name)
    }.display()
  );

  println!("file_new_append_incremental \"{}\"...", file_name.display());
  unwrap!{
    rs_utils::file::file_new_append_incremental (file_name)
  };

  println!("file_path_incremental \"{}\": {}", file_name.display(),
    unwrap!{
      rs_utils::file::file_path_incremental (file_name)
    }.display()
  );

  println!("file_new_append_incremental \"{}\"...", file_name.display());
  unwrap!{
    rs_utils::file::file_new_append_incremental (file_name)
  };

  println!("!ls examples/");
  assert!{
    unwrap!{
      std::process::Command::new ("ls").arg("examples/").status()
    }.success()
  };

  println!("cleaning up...");
  unwrap!{
    std::fs::remove_dir_all (std::path::Path::new ("examples"))
  };

  println!("...rs_utils example main");
}
