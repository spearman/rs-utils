//! `rs-utils` example

extern crate tempfile;
extern crate rs_utils;

use std::path::Path;
use clap::Parser;
use rs_utils::*;

fn main () {
  println!("{:?} main...", app::exe_name());

  let app = App::parse();
  let log_level = app.args.log_level.unwrap_or (simplelog::LevelFilter::Debug);
  app::init_combined_termlogger (log_level, app.args.module_filters);
  log::info!("terminal logger initialized");
  // incremental files example
  log::info!("example: incrementally named files");
  let temp_dir  =
    tempfile::Builder::new().prefix ("tmp").tempdir_in ("examples").unwrap();
  let file_path = temp_dir.path().join (Path::new ("myfile"));
  let path      = file_path.as_path();
  log::debug!("file_path_incremental\n  \"{}\":\n  {}",
    path.display(), file::file_path_incremental (path).unwrap().display());
  log::debug!("file_new_append_incremental \"{}\"...", path.display());
  file::file_new_append_incremental (path).unwrap();
  log::debug!("file_path_incremental\n  \"{}\":\n  {}",
    path.display(), file::file_path_incremental (path).unwrap().display());
  log::debug!("file_new_append_incremental \"{}\"...", path.display());
  file::file_new_append_incremental (path).unwrap();
  log::debug!("file_path_incremental\n  \"{}\":\n  {}",
    path.display(), file::file_path_incremental (path).unwrap().display());
  log::debug!("file_path_incremental and then file_new_append\n  \"{}\"...",
    path.display());
  // file_new_append_incremental is equivalent to:
  file::file_path_incremental (path)
    .and_then (|path_buf| file::file_new_append (path_buf.as_path())).unwrap();
  log::info!("!ls {}", temp_dir.path().display());
  assert!(std::process::Command::new ("ls")
    .arg (temp_dir.path().as_os_str())
    .status().unwrap().success());

  println!("...{:?} main", app::exe_name());
}
