extern crate std;

lazy_static! {
  pub static ref FILE_NAME : String = {
    let path_buf = std::path::PathBuf::from (
      unwrap!{ std::env::args().next() }
    );
    let file_name = unwrap!{ unwrap!{ path_buf.file_name() }.to_str() };
    file_name.to_string()
  };
}
