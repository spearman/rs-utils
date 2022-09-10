//! Application utilities

use std;
use std::error::Error;
use clap;
use simplelog;

/// Derived argument parser
///
/// Usage:
/// ```text
/// use clap::Parser;
/// let app = rs_utils::App::parse();
/// ```
#[derive(Debug, clap::Parser)]
#[clap(about = "rs-utils app", version)]
pub struct App {
  #[clap(flatten)]
  pub args : Args
}

/// Derived clap arguments
#[derive(Debug, clap::Args)]
pub struct Args {
  #[clap(short = 'l', help = "Global log level filter")]
  pub log_level     : Option <simplelog::LevelFilter>,
  #[clap(short = 'm',
    name = "MODULE=LEVEL",
    value_parser = parse_key_val::<String, simplelog::LevelFilter>,
    help = "Module log filters")]
  pub module_filters : Vec <(String, simplelog::LevelFilter)>
}

/// Initialize `simplelog` terminal logger to stdout with the given log level.
///
/// This sets the "target level" to `Error` (show module paths for all levels)
/// and sets the "thread level" to `Off` (don't show thread numbers for any
/// levels).
pub fn init_simple_termlogger (log_level : simplelog::LevelFilter) {
  simplelog::TermLogger::init (
    log_level,
    base_simplelog_config_builder().build(),
    simplelog::TerminalMode::Stdout,
    simplelog::ColorChoice::Auto
  ).unwrap();
}

/// Initialize a `simplelog` terminal logger to stdout with a given global log
/// level and list of module filters.
///
/// This sets the "target level" to `Error` (show module paths for all levels)
/// and sets the "thread level" to `Off` (don't show thread numbers for any
/// levels).
pub fn init_combined_termlogger (
  global_log_level  : simplelog::LevelFilter,
  module_log_levels : Vec <(String, simplelog::LevelFilter)>
) {
  let global_config = {
    let mut builder = base_simplelog_config_builder();
    for (module, _) in module_log_levels.iter().cloned() {
      builder.add_filter_ignore (module);
    }
    builder.build()
  };
  let mut loggers : Vec <Box <dyn simplelog::SharedLogger>> = vec![
    simplelog::TermLogger::new (
      global_log_level,
      global_config,
      simplelog::TerminalMode::Stdout,
      simplelog::ColorChoice::Auto
    )
  ];
  for (module, level_filter) in module_log_levels {
    let mut builder = base_simplelog_config_builder();
    builder.add_filter_allow (module);
    loggers.push (simplelog::TermLogger::new (
      level_filter,
      builder.build(),
      simplelog::TerminalMode::Stdout,
      simplelog::ColorChoice::Auto
    ));
  }
  simplelog::CombinedLogger::init (loggers).unwrap();
}

/// Gets the `file_name()` of the `std::env::current_exe()`
#[inline]
pub fn exe_name() -> std::ffi::OsString {
  std::env::current_exe().unwrap().file_name().unwrap().to_owned()
}

//
//  private
//

/// `simplelog` config builder with "target level" to `Error` (show module paths
/// for all levels) and sets the "thread level" to `Off` (don't show thread
/// numbers for any levels).
fn base_simplelog_config_builder() -> simplelog::ConfigBuilder {
  let mut builder = simplelog::ConfigBuilder::new();
  builder.set_target_level (simplelog::LevelFilter::Error); // module path
  builder.set_thread_level (simplelog::LevelFilter::Off);   // no thread numbers
  builder
}

fn parse_key_val <T, U> (s : &str)
  -> Result <(T, U), Box <dyn Error + Send + Sync + 'static>>
where
  T      : std::str::FromStr,
  T::Err : Error + Send + Sync + 'static,
  U      : std::str::FromStr,
  U::Err : Error + Send + Sync + 'static,
{
  let pos = s.find ('=')
    .ok_or_else (|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
  Ok ((s[..pos].parse()?, s[pos + 1..].parse()?))
}

////////////////////////////////////////////////////////////////////////////////
//  impls
////////////////////////////////////////////////////////////////////////////////

impl Args {
  pub fn init_termlogger (&self) {
  }
}
