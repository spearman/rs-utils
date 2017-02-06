extern crate tempdir;
#[macro_use]
extern crate unwrap;

extern crate rs_utils;

fn main () {
  println!("example rs_utils::file main...");

  let temp_dir = unwrap!{ tempdir::TempDir::new_in ("examples", "tmp") };

  let file_path = temp_dir.path().join (
    std::path::Path::new ("myfile"));
  let file_path = file_path.as_path();

  println!("file_path_incremental \"{}\": {}", file_path.display(),
    unwrap!{
      rs_utils::file::file_path_incremental (file_path)
    }.display()
  );
  println!("file_new_append_incremental \"{}\"...", file_path.display());
  unwrap!{
    rs_utils::file::file_new_append_incremental (file_path)
  };

  println!("file_path_incremental \"{}\": {}", file_path.display(),
    unwrap!{
      rs_utils::file::file_path_incremental (file_path)
    }.display()
  );
  println!("file_new_append_incremental \"{}\"...", file_path.display());
  unwrap!{
    rs_utils::file::file_new_append_incremental (file_path)
  };

  println!("file_path_incremental \"{}\": {}", file_path.display(),
    unwrap!{
      rs_utils::file::file_path_incremental (file_path)
    }.display()
  );
  println!("file_path_incremental and then file_new_append \"{}\"...",
    file_path.display()
  );
  unwrap!{
    // file_new_append_incremental is equivalent to:
    rs_utils::file::file_path_incremental (file_path).and_then (
      |path_buf| rs_utils::file::file_new_append (path_buf.as_path())
    )
  };

  println!("!ls {}", temp_dir.path().display());
  assert!{
    unwrap!{
      std::process::Command::new ("ls").arg (temp_dir.path().as_os_str())
        .status()
    }.success()
  };

  println!("...example rs_utils::file main");
}
