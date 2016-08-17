use std::path::{
  MAIN_SEPARATOR,
  Path,
  PathBuf};

use std::string::String;

//
//  rs_utils::Error
//
#[derive(Debug,PartialEq)]
pub enum Error {
  NoFileSpecified,
  NotValidUnicode
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
/// # use rs_utils::{Error, inc_file_name};
/// assert_eq!(inc_file_name ("somepath/"),
///   Err(Error::NoFileSpecified));
/// assert_eq!(inc_file_name ("."),
///   Err(Error::NoFileSpecified));
/// ```

pub fn inc_file_name (file_path : &str) -> Result <PathBuf,Error> {
  if file_path.ends_with (MAIN_SEPARATOR) {
    return Err(Error::NoFileSpecified)
  }
  let p         = Path::new (file_path);
  let osstr     = try! {
    p.file_name() .ok_or (Error::NoFileSpecified)
  };
  let file_name = try! {
    osstr.to_str().ok_or (Error::NotValidUnicode)
  };
  let dir       = p.parent().unwrap_or (Path::new (""));
  for i in 0.. {
    let name = String::from (file_name) + &format!(".{}", i);
    let fp   = dir.join (name);
    if !fp.exists() {
      return Ok(fp)
    }
  }
  unreachable!("error: incremental file name loop should have returned")
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}
