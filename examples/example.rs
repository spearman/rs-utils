//! `rs-utils` example

use std::path::Path;
use env_logger;
use tempfile;

use rs_utils::file;
use rs_utils::log::{LevelFilter, kv as log};

fn main () {
  println!("main...");
  env_logger::Builder::new()
    .filter_level (LevelFilter::Debug)
    .parse_default_env()
    .format (log::env_logger_custom_formatter (log::EnvLoggerFormatConfig::default()))
    //.format (log::env_logger_json_formatter (log::EnvLoggerFormatConfig::default()))
    .init();
  // test kv string
  log::info!("test kv string"; a=r#"hello"world""#);
  log::info!("test kv string"; a=r#"hello "world""#);
  // incremental files example
  log::info!("example: incrementally named files");
  let temp_dir  = tempfile::Builder::new().prefix ("tmp").tempdir_in ("examples")
    .unwrap();
  let file_path = temp_dir.path().join (Path::new ("myfile"));
  let path      = file_path.as_path();
  log::debug!("file path incremental";
    target_path=path.display().to_string().as_str(),
    incremental_path=file::file_path_incremental (path)
      .unwrap().display().to_string().as_str());
  log::debug!("file new append incremental";
    target_path=path.display().to_string().as_str());
  file::file_new_append_incremental (path).unwrap();
  log::debug!("file path incremental";
    target_path=path.display().to_string().as_str(),
    incremental_path=file::file_path_incremental (path)
      .unwrap().display().to_string().as_str());
  log::debug!("file new append incremental";
    target_path=path.display().to_string().as_str());
  file::file_new_append_incremental (path).unwrap();
  log::debug!("file path incremental";
    target_path=path.display().to_string().as_str(),
    incremental_path=file::file_path_incremental (path)
      .unwrap().display().to_string().as_str());
  log::debug!("file path incremental and then file new append";
    target_path=path.display().to_string().as_str());
  // file_new_append_incremental is equivalent to:
  file::file_path_incremental (path)
    .and_then (|path_buf| file::file_new_append (path_buf.as_path())).unwrap();
  log::info!("!ls {}", temp_dir.path().display());
  assert!(std::process::Command::new ("ls").arg (temp_dir.path().as_os_str())
    .status().unwrap().success());

  println!("...main");
}
