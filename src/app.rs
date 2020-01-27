//! Application utilities

use std;
use clap;
use log;
use simplelog;

/// Adds a `clap` argument named `"log-level"` with short name `"l"`,
/// and value name `"LOG_LEVEL"`, suitable for passing a log level filter:
///
/// ```ignore
/// ./myapp -l Debug
/// ```
pub fn clap_arg_log_level <'a, 'b> (app : clap::App <'a, 'b>)
  -> clap::App <'a, 'b>
{
  app.arg (clap::Arg::with_name ("log-level")
    .short ("l")
    .value_name ("LOG_LEVEL")
    .help ("Log level filter"))
}

/// Get the value of the `LOG_LEVEL` option and attempt to parse it as
/// a `simplelog::LevelFilter`
pub fn clap_opt_log_level <'a> (opts : &clap::ArgMatches <'a>)
  -> Result <Option <simplelog::LevelFilter>, log::ParseLevelError>
{
  use std::str::FromStr;
  use simplelog::LevelFilter;
  match opts.value_of ("log-level") {
    None => Ok (None),
    Some (log_level) => {
      let log_level = LevelFilter::from_str (log_level)?;
      Ok (Some (log_level))
    }
  }
}

/// Initialize `simplelog` terminal logger to stdout with the given log level.
///
/// This sets the "target level" to `Error` (show module paths for all levels)
/// and sets the "thread level" to `Off` (don't show thread numbers for any
/// levels).
pub fn init_simple_termlogger (log_level : simplelog::LevelFilter) {
  simplelog::TermLogger::init (
    log_level,
    simplelog::ConfigBuilder::new()
      .set_target_level (simplelog::LevelFilter::Error) // module path
      .set_thread_level (simplelog::LevelFilter::Off)   // no thread numbers
      .build(),
    simplelog::TerminalMode::Stdout
  ).unwrap();
}

/// Gets the `file_name()` of the `std::env::current_exe()`
#[inline]
pub fn exe_name() -> std::ffi::OsString {
  std::env::current_exe().unwrap().file_name().unwrap().to_owned()
}
