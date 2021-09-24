extern crate unwrap;

extern crate tempfile;

extern crate rs_utils;

use std::path::Path;
use unwrap::unwrap;
use rs_utils::*;

fn main () {
  println!("{:?} main...", app::exe_name());

  // options
  let opts = {
    let opts = clap::App::new ("RsUtils Example App");
    app::clap_arg_log_level (opts).get_matches()
  };
  // log init
  let log_level = app::clap_opt_log_level (&opts).unwrap()
    .unwrap_or (simplelog::LevelFilter::Debug);
  app::init_simple_termlogger (log_level);
  log::info!("terminal logger initialized");
  // incremental files example
  log::info!("example: incrementally named files");
  let temp_dir  = unwrap!(
    tempfile::Builder::new().prefix ("tmp").tempdir_in ("examples"));
  let file_path = temp_dir.path().join (Path::new ("myfile"));
  let path      = file_path.as_path();
  log::debug!("file_path_incremental\n  \"{}\":\n  {}",
    path.display(), unwrap!(file::file_path_incremental (path)).display());
  log::debug!("file_new_append_incremental \"{}\"...", path.display());
  unwrap!(file::file_new_append_incremental (path));
  log::debug!("file_path_incremental\n  \"{}\":\n  {}",
    path.display(), unwrap!(file::file_path_incremental (path)).display());
  log::debug!("file_new_append_incremental \"{}\"...", path.display());
  unwrap!(file::file_new_append_incremental (path));
  log::debug!("file_path_incremental\n  \"{}\":\n  {}",
    path.display(), unwrap!(file::file_path_incremental (path)).display());
  log::debug!("file_path_incremental and then file_new_append\n  \"{}\"...",
    path.display());
  // file_new_append_incremental is equivalent to:
  unwrap!(file::file_path_incremental (path)
    .and_then (|path_buf| file::file_new_append (path_buf.as_path())));
  log::info!("!ls {}", temp_dir.path().display());
  assert!(unwrap!(
    std::process::Command::new ("ls")
      .arg (temp_dir.path().as_os_str())
      .status()
  ).success());

  println!("...{:?} main", app::exe_name());
}
