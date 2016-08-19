use std::fs::{
  File,
  OpenOptions,
  create_dir_all};

use std::io::{
  Error,
  ErrorKind};

use std::path::{
  MAIN_SEPARATOR,
  Path,
  PathBuf};

use std::string::String;

/// opens a new file at specified path for writing in append mode,
/// recursively creating parent directories
///
/// # Errors
///
/// - file already exists
/// - not a file

pub fn dump_file (file_path : &Path) -> Result <File,Error> {
  if !try! { is_file (file_path) } {
    return Err (Error::new (ErrorKind::InvalidInput,
      "no file specified".to_string()))
  }

  if file_path.exists() {
    return Err (Error::new (ErrorKind::AlreadyExists,
      "file already exists".to_string()))
  }

  let dir = file_path.parent().unwrap_or (Path::new (""));
  try! { create_dir_all (dir) };

  OpenOptions::new()
    .append     (true)
    .create_new (true)
    .open       (file_path)
}

//
//  rs_utils::inc_file_name
//
/// returns the next non-existing incrementally-numbered file
/// `path/to/filename.N` where `N` gives the first available filename
/// starting from `0`
///
/// this function only queries the next available filename, no
/// directories or files are created

/// # Errors
///
/// not a file:
///
/// ```
/// # use std::io::{Error,ErrorKind}; use std::path::Path;
/// # use rs_utils::inc_file_name;
/// assert_eq!(
///   inc_file_name (Path::new ("somepath/")).unwrap_err().kind(),
///   ErrorKind::InvalidInput);
/// assert_eq!(
///   inc_file_name (Path::new (".")).unwrap_err().kind(),
///   ErrorKind::InvalidInput);
/// ```

pub fn inc_file_name (file_path : &Path) -> Result <PathBuf,Error> {
  if !try!{ is_file (file_path) } {
    return Err (Error::new (
      ErrorKind::InvalidInput,
      "no file specified".to_string()))
  }

  // unwrap failure would have been caught by `is_file` test
  let file_name = file_path.file_name().unwrap().to_str().unwrap();
  let dir       = file_path.parent().unwrap_or (Path::new (""));
  for i in 0.. {
    let name = String::from (file_name) + &format!(".{}", i);
    let fp   = dir.join (name);
    if !fp.exists() {
      return Ok(fp)
    }
  }
  unreachable!("error: incremental file name loop should have returned")
}

//
//  rs_utils::is_file
//
/// if this returns true then `std::fs::File::create` will not fail
/// with "is a directory" error
///
/// ```
/// # use std::path::Path; use rs_utils::is_file;
/// assert_eq!(true,  is_file (Path::new ("path/to/file")).unwrap());
/// assert_eq!(false, is_file (Path::new ("path/to/notfile/")).unwrap());
/// assert_eq!(false, is_file (Path::new ("..")).unwrap());
/// ```

// FIXME: tests?

pub fn is_file (file_path : &Path)
  -> Result <bool,Error>
{
  let s = try! {
    file_path.to_str().ok_or (Error::new (
      ErrorKind::InvalidInput,
      "not valid unicode".to_string()))
  };

  if s.ends_with (MAIN_SEPARATOR) {
    return Ok(false)
  }

  if let None = Path::new (file_path).file_name() {
    return Ok(false)
  }

  Ok(true)
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}
