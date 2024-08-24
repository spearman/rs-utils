//! `rs-utils` example

use std::path::Path;
use env_logger;
use log::LevelFilter;
use tempfile;

// log conflicts with the log crate
use rs_utils::{log as log_, file};

fn main () {
  println!("main...");
  env_logger::Builder::new()
    .filter_level (LevelFilter::Debug)
    .parse_default_env()
    //.format (log_::env_logger_json_format)
    .format (log_::env_logger_custom_format)
    .init();
  // test kv string
  log_::kv::info!("test kv string"; a=r#"hello"world""#);
  log_::kv::info!("test kv string"; a=r#"hello "world""#);
  // incremental files example
  log_::info!("example: incrementally named files");
  let temp_dir  =
    tempfile::Builder::new().prefix ("tmp").tempdir_in ("examples").unwrap();
  let file_path = temp_dir.path().join (Path::new ("myfile"));
  let path      = file_path.as_path();
  log_::kv::debug!("file path incremental";
    target_path=path.display().to_string().as_str(),
    incremental_path=file::file_path_incremental (path).unwrap().display()
      .to_string().as_str());
  log_::kv::debug!("file new append incremental";
    target_path=path.display().to_string().as_str());
  file::file_new_append_incremental (path).unwrap();
  log_::kv::debug!("file path incremental";
    target_path=path.display().to_string().as_str(),
    incremental_path=file::file_path_incremental (path).unwrap().display()
      .to_string().as_str());
  log_::kv::debug!("file new append incremental";
    target_path=path.display().to_string().as_str());
  file::file_new_append_incremental (path).unwrap();
  log_::kv::debug!("file path incremental";
    target_path=path.display().to_string().as_str(),
    incremental_path=file::file_path_incremental (path).unwrap().display()
      .to_string().as_str());
  log_::kv::debug!("file path incremental and then file new append";
    target_path=path.display().to_string().as_str());
  // file_new_append_incremental is equivalent to:
  file::file_path_incremental (path)
    .and_then (|path_buf| file::file_new_append (path_buf.as_path())).unwrap();
  log_::info!("!ls {}", temp_dir.path().display());
  assert!(std::process::Command::new ("ls")
    .arg (temp_dir.path().as_os_str())
    .status().unwrap().success());

  println!("...main");
}
