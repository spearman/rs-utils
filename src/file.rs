extern crate std;

//
//  file_new_append_incremental
//
/// Calls `file_new_append` on the path returned by feeding the file path
/// to `file_path_incremental`.

/// # Errors
///
/// - Invalid unicode (&#x261e; see [`is_file`](fn.is_file.html))
/// - Not a file (&#x261e; see [`file_path_incremental`](fn.file_path_incremental.html))

pub fn file_new_append_incremental (file_path : &std::path::Path)
  -> Result <std::fs::File, std::io::Error>
{
  let file_name = try!(file_path_incremental (file_path));
  file_new_append (file_name.as_path())
}

//
//  file_new_append
//
/// Opens a new file at specified path for writing in append mode,
/// recursively creating parent directories

/// # Errors
///
/// - Invalid unicode (&#x261e; see [`is_file`](fn.is_file.html))
/// - Not a file:
///
/// ```
/// # use std::error::Error; use std::io::ErrorKind; use std::path::Path;
/// # use rs_utils::file::file_new_append;
/// let e = file_new_append (Path::new ("somepath/")).err().unwrap();
/// assert_eq!(e.kind(), ErrorKind::InvalidInput);
/// assert_eq!(e.description(), "not a file");
/// ```
///
/// - File already exists:
///
/// ```
/// extern crate tempdir;
/// # extern crate rs_utils;
/// # use std::error::Error; use std::io::ErrorKind; use std::path::Path;
/// # use rs_utils::file::file_new_append;
/// # fn main () {
///
/// let temp_dir = tempdir::TempDir::new_in ("", "tmp").unwrap();
/// let file_path = temp_dir.path().join (Path::new ("somefile"));
/// let file_path = file_path.as_path();
/// assert! (!file_path.exists());
/// file_new_append (file_path).unwrap();
/// let e = file_new_append (file_path).err().unwrap();
/// assert_eq!(e.kind(), ErrorKind::AlreadyExists);
/// assert_eq!(e.description(), "entity already exists");
/// # }
/// ```

pub fn file_new_append (file_path : &std::path::Path)
  -> Result <std::fs::File, std::io::Error>
{
  if !try!(is_file (file_path)) {
    return Err (std::io::Error::new (std::io::ErrorKind::InvalidInput,
      "not a file".to_string()))
  }

  let dir = file_path.parent().unwrap_or_else (|| std::path::Path::new (""));
  try!(std::fs::create_dir_all (dir));

  std::fs::OpenOptions::new().append (true).create_new (true).open (file_path)
} // end file_new_append

//
//  file_path_incremental
//
/// Returns the file path appended with suffix `.N` where `N` gives
/// the first available non-pre-existing filename starting from `0`.
///
/// This function only queries for the next available filename, no
/// directories or files are created.

/// # Examples
///
/// ```
/// # use std::path::Path;
/// # use rs_utils::file::file_path_incremental;
/// let file_path = Path::new ("somedir/somefile");
/// assert_eq!(
///   file_path_incremental (file_path).unwrap().to_str().unwrap(),
///   "somedir/somefile.0"
/// );
/// ```

/// # Errors
///
/// - Invalid unicode (&#x261e; see [`is_file`](fn.is_file.html))
/// - Not a file:
///
/// ```
/// # use std::error::Error; use std::io::ErrorKind; use std::path::Path;
/// # use rs_utils::file::file_path_incremental;
/// let e = file_path_incremental (Path::new ("somepath/")).err().unwrap();
/// assert_eq!(e.kind(), ErrorKind::InvalidInput);
/// assert_eq!(e.description(), "not a file");
/// ```

pub fn file_path_incremental (file_path : &std::path::Path)
  -> Result <std::path::PathBuf, std::io::Error>
{
  if !try!(is_file (file_path)) {
    return Err (std::io::Error::new (
      std::io::ErrorKind::InvalidInput, "not a file".to_string()))
  }

  // unwrap failure should have been caught by `is_file` test
  let file_name = file_path.file_name().unwrap_or_else (
    || panic!("fatal: path should be a valid file")
  ).to_str().unwrap_or_else (
    || panic!("fatal: `file_path.file_name()` \
      returned invalid os str: {:?}", file_path.file_name()));
  let dir = file_path.parent().unwrap_or_else (|| std::path::Path::new (""));
  for i in 0.. {
    let name = String::from (file_name) + &format!(".{}", i);
    let fp   = dir.join (name);
    if !fp.exists() {
      return Ok (fp)
    }
  }
  unreachable!("fatal: incremental file name loop should have returned")
} // end file_path_incremental

//
//  is_file
//
/// If this returns true then `std::fs::File::create` will not fail
/// with "is a directory" error.
///
/// This is not the same as `std::path::Path::is_file` which also
/// tests whether the file actually exists.
///
/// # Examples
///
/// ```
/// # use std::path::Path; use rs_utils::file::is_file;
/// assert!(is_file (Path::new ("path/to/file")).unwrap());
/// assert!(!is_file (Path::new ("path/to/directory/")).unwrap());
/// assert!(!is_file (Path::new ("..")).unwrap());
/// ```
///
/// # Errors
///
/// - Invalid unicode:
///
/// ```
/// # use std::error::Error; use std::io::ErrorKind;
/// # use std::path::Path; use std::ffi::OsStr;
/// # use rs_utils::file::is_file;
/// use std::os::unix::ffi::OsStrExt;
/// let garbage = [192u8, 192u8, 192u8, 192u8];
/// let garbage_path = Path::new (OsStr::from_bytes (&garbage));
/// let e = is_file (&garbage_path).err().unwrap();
/// assert_eq!(e.kind(), ErrorKind::InvalidInput);
/// assert_eq!(e.description(), "not valid unicode");
/// ```

pub fn is_file (file_path : &std::path::Path) -> Result <bool, std::io::Error> {
  let s = try!{
    file_path.to_str().ok_or (std::io::Error::new (
      std::io::ErrorKind::InvalidInput, "not valid unicode".to_string()))
  };

  if s.ends_with (std::path::MAIN_SEPARATOR) {
    return Ok (false)
  }

  if let None = std::path::Path::new (file_path).file_name() {
    return Ok (false)
  }

  Ok (true)
} //  end is_file

//
//  tests
//
#[cfg(test)]
mod tests {
  use ::file::*;
  extern crate std;
  extern crate quickcheck;
  extern crate tempdir;

  //
  //  test_is_file
  //
  // test that is_file() implies file creation will not give an "is a
  // directory" error: as of Rust 1.16 (2017-01-23) this error is
  // simply indicated by an ErrorKind::Other (other os error)
  #[ignore]
  #[quickcheck]
  fn prop_is_file_implies_not_directory (file_path : String)
    -> quickcheck::TestResult
  {
    let file_path = std::path::Path::new (file_path.as_str());
    if !is_file (file_path).unwrap() {
      return quickcheck::TestResult::discard()
    }
    if let Some (s) = file_path.parent() {
      if !s.to_str().unwrap().is_empty() {
        return quickcheck::TestResult::discard()
      }
    }
    let temp_dir = tempdir::TempDir::new_in ("", "tmp").unwrap();
    let file_path = temp_dir.path().join (file_path);
    quickcheck::TestResult::from_bool (
      if let Err(e) = std::fs::OpenOptions::new()
        .append (true).create (true).open (file_path.clone())
      {
        e.kind() != std::io::ErrorKind::Other
      } else {
        true
      }
    )
  }

} // end mod tests
