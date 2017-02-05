extern crate std;

//
//  file_new_append_incremental
//
/// Calls `file_new_append` on the path returned by feeding the file path
/// to `file_path_incremental`

/// # Errors
///
/// - Invalid unicode (see [`is_file`](fn.is_file.html))
/// - Not a file:
///
/// ```
/// # use std::path::Path; use std::io::ErrorKind;
/// # use rs_utils::file::file_new_append_incremental;
/// assert_eq!(
///   file_new_append_incremental (Path::new ("somedir/")).unwrap_err().kind(),
///   ErrorKind::InvalidInput);
/// ```
///

pub fn file_new_append_incremental (file_path : &std::path::Path)
  -> Result <std::fs::File, std::io::Error>
{
  let file_name = try!{ file_path_incremental (file_path) };
  file_new_append (file_name.as_path())
}

//
//  file_new_append
//
/// Opens a new file at specified path for writing in append mode,
/// recursively creating parent directories

/// # Errors
///
/// - Invalid unicode (see [`is_file`](fn.is_file.html))
/// - Not a file:
///
/// ```
/// # use std::path::Path; use std::io::ErrorKind;
/// # use rs_utils::file::file_new_append;
/// assert_eq!(
///   file_new_append (Path::new ("somedir/")).unwrap_err().kind(),
///   ErrorKind::InvalidInput);
/// ```
///
/// - File already exists:
///
/// ```
/// # use std::path::Path; use std::io::ErrorKind;
/// # use rs_utils::file::file_new_append;
/// let file_path = Path::new ("somedir/somefile");
/// assert! (!file_path.exists());
/// file_new_append (file_path).unwrap();
/// assert_eq!(
///   file_new_append (file_path).unwrap_err().kind(),
///   ErrorKind::AlreadyExists);
/// # std::fs::remove_dir_all (Path::new ("somedir")).unwrap()
/// ```

pub fn file_new_append (file_path : &std::path::Path)
  -> Result <std::fs::File, std::io::Error>
{
  if !try!{ is_file (file_path) } {
    return Err (std::io::Error::new (std::io::ErrorKind::InvalidInput,
      "no file specified".to_string()))
  }

  if file_path.exists() {
    return Err (std::io::Error::new (std::io::ErrorKind::AlreadyExists,
      "file already exists".to_string()))
  }

  let dir = file_path.parent().unwrap_or (std::path::Path::new (""));
  try!{ std::fs::create_dir_all (dir) };

  std::fs::File::create (file_path)
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
/// assert_eq!(
///   file_path_incremental (Path::new ("somedir/somefile")).unwrap().as_path().to_str().unwrap(),
///   "somedir/somefile.0"
/// );
/// ```

/// # Errors
///
/// - Invalid unicode (see [`is_file`](fn.is_file.html))
/// - Not a file:
///
/// ```
/// # use std::io::ErrorKind; use std::path::Path;
/// # use rs_utils::file::file_path_incremental;
/// assert_eq!(
///   file_path_incremental (Path::new ("somepath/")).unwrap_err().kind(),
///   ErrorKind::InvalidInput);
/// assert_eq!(
///   file_path_incremental (Path::new (".")).unwrap_err().kind(),
///   ErrorKind::InvalidInput);
/// ```

pub fn file_path_incremental (file_path : &std::path::Path)
  -> Result <std::path::PathBuf, std::io::Error>
{
  if !try!{ is_file (file_path) } {
    return Err (std::io::Error::new (
      std::io::ErrorKind::InvalidInput,
      "no file specified".to_string()))
  }

  // unwrap failure should have been caught by `is_file` test
  let file_name = file_path.file_name().unwrap_or_else (
    || panic!("fatal: path should be a valid file")
  ).to_str().unwrap_or_else (
    || panic!("fatal: `file_path.file_name()` \
      returned invalid os str: {:?}", file_path.file_name()));
  let dir = file_path.parent().unwrap_or (
    std::path::Path::new (""));
  for i in 0.. {
    let name = String::from (file_name) + &format!(".{}", i);
    let fp   = dir.join (name);
    if !fp.exists() {
      return Ok(fp)
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
/// # use std::path::Path; use std::ffi::OsStr; use std::io::ErrorKind;
/// # use rs_utils::file::is_file;
/// use std::os::unix::ffi::OsStrExt;
/// let garbage = [192u8, 192u8, 192u8, 192u8];
/// let garbage_path = Path::new (OsStr::from_bytes (&garbage));
/// assert_eq!(
///   is_file (&garbage_path).unwrap_err().kind(),
///   ErrorKind::InvalidInput
/// );
/// ```

pub fn is_file (file_path : &std::path::Path)
  -> Result <bool, std::io::Error>
{
  let s = try!{
    file_path.to_str().ok_or (std::io::Error::new (
      std::io::ErrorKind::InvalidInput,
      "not valid unicode".to_string()))
  };

  if s.ends_with (std::path::MAIN_SEPARATOR) {
    return Ok(false)
  }

  if let None = std::path::Path::new (file_path).file_name() {
    return Ok(false)
  }

  Ok(true)
} //  end is_file

//
//  tests
//
#[cfg(test)]
mod tests {
  extern crate std;

  //
  //  test_file_new_append
  //
  #[test]
  fn test_file_new_append () {
    use ::file::file_new_append;
    {
      let file_path = std::path::Path::new (
        "test_file_new_append/myfile");
      file_new_append (file_path).unwrap();
      assert!(file_path.exists());
      assert!(file_path.is_file());
    }
    // cleanup
    std::fs::remove_dir_all (
      std::path::Path::new ("test_file_new_append")
    ).unwrap()
  }

  //
  //  test_file_path_incremental
  //
  #[test]
  fn test_file_path_incremental () {
    use ::file::file_path_incremental;
    assert_eq!(
      "test_file_path_incremental/myfile.0",
      file_path_incremental (
        std::path::Path::new ("test_file_path_incremental/myfile")
      ).unwrap().as_path().to_str().unwrap()
    );
  }

  //
  //  test_is_file
  //
  #[test]
  fn test_is_file () {
    use ::file::is_file;

    // ok: is file
    assert!(
      is_file (std::path::Path::new ("path/to/file")).unwrap()
    );
    // ok: is directory
    assert!(
      !is_file (std::path::Path::new ("path/to/directory/")).unwrap()
    );
    assert!(
      !is_file (std::path::Path::new ("..")).unwrap()
    );
    assert!(
      !is_file (std::path::Path::new (".")).unwrap()
    );
    // err: invalid input
    use std::os::unix::ffi::OsStrExt;
    let garbage = [192u8, 192u8, 192u8, 192u8];
    let garbage_path = std::path::Path::new (
      std::ffi::OsStr::from_bytes (&garbage));
    assert_eq!(
      is_file (&garbage_path).unwrap_err().kind(),
      std::io::ErrorKind::InvalidInput
    );

    // TODO: test that is_file() implies std::fs::File::create will
    // not give an "is a directory" error
  }

}
