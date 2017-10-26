extern crate std;

lazy_static! {
  pub static ref FILE_NAME : String = {
    let path_buf = std::path::PathBuf::from (std::env::args().next().unwrap());
    let file_name = path_buf.file_name().unwrap().to_str().unwrap();
    file_name.to_string()
  };
}
