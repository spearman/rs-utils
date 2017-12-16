//! Process related utilities.

use ::std;

lazy_static! {
  /// A lazy static string loaded from the first element of `std::env::args()`
  /// (the name of the executable file currently being run).
  pub static ref EXE_FILE_NAME : String = {
    let path_buf  = std::path::PathBuf::from (std::env::args().next().unwrap());
    let file_name = path_buf.file_name().unwrap().to_str().unwrap();
    file_name.to_string()
  };
}
