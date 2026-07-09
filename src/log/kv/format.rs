//! Custom `env_logger` formats

use std::{io, thread};
use env_logger;
use log;

#[derive(Clone, Copy, Debug)]
pub struct EnvLoggerFormatConfig {
  pub thread              : bool,
  pub target              : bool,
  pub file                : bool,
  pub timestamp_precision : env_logger::TimestampPrecision
}

/// Formats log messages as a json object string
pub fn env_logger_json_formatter (config : EnvLoggerFormatConfig)
  -> impl Fn(&mut env_logger::fmt::Formatter, &log::Record<'_>) -> io::Result<()>
{
  use io::Write;
  #[derive(Default)]
  struct KVVisitor(pub String);
  impl <'kvs> log::kv::VisitSource <'kvs> for KVVisitor {
    fn visit_pair (&mut self, key : log::kv::Key <'kvs>, value : log::kv::Value <'kvs>)
      -> Result <(), log::kv::Error>
    {
      self.0 += format!(",\"{}\":{}", key, serde_json::to_string (&value).unwrap())
        .as_str();
      Ok(())
    }
  }
  move |buf : &mut env_logger::fmt::Formatter, record : &log::Record|{
    let thread_string = if config.thread {
      thread::current().name().map_or_else (
        || format!(",\"thread\":\"{:?}\"", thread::current().id())
          .replace ("ThreadId", "unnamed"),
        |name| format!(",\"thread\":\"{name}\""))
    } else {
      "".to_string()
    };
    let target_string = if config.target {
      format!(",\"target\":\"{}\"", record.target())
    } else {
      "".to_string()
    };
    let file_string = if config.file {
      format!(",\"file\":\"{}:{}\"",
        record.file().unwrap_or ("<unknown>"), record.line().unwrap_or (0))
    } else {
      "".to_string()
    };
    let mut kvv = KVVisitor::default();
    record.key_values().visit (&mut kvv).unwrap();
    let ts = match config.timestamp_precision {
      env_logger::TimestampPrecision::Seconds => buf.timestamp_seconds(),
      env_logger::TimestampPrecision::Millis  => buf.timestamp_millis(),
      env_logger::TimestampPrecision::Micros  => buf.timestamp_micros(),
      env_logger::TimestampPrecision::Nanos   => buf.timestamp_nanos()
    };
    writeln!(buf, "{{\"ts\":\"{ts}\",\"level\":\"{}\"{}{}{},\"msg\":\"{}\"{}}}",
      record.level(), thread_string, target_string, file_string, record.args(), kvv.0
    )
  }
}

/// A custom `env_logger` format adding thread, target, and file information:
/// ```text
/// <ts> <level> <thread> <target> <file>: <msg> [ <key>=<value>]
/// ```
pub fn env_logger_custom_formatter (config : EnvLoggerFormatConfig)
  -> impl Fn(&mut env_logger::fmt::Formatter, &log::Record<'_>) -> io::Result<()>
{
  use io::Write;
  #[derive(Default)]
  struct KVVisitor(pub String);
  impl <'kvs> log::kv::VisitSource <'kvs> for KVVisitor {
    fn visit_pair (&mut self, key : log::kv::Key <'kvs>, value : log::kv::Value <'kvs>)
      -> Result <(), log::kv::Error>
    {
      let mut value_string = serde_json::to_string (&value).unwrap();
      let mut value_str = value_string.as_str();
      if !value_str.contains (char::is_whitespace) {
        // if there is no whitespace we can remove outer quotes and un-escape
        // any internal quotes
        value_string = value_str.replace ("\\\"", "\"");
        value_str = if value_string.starts_with ('"') {
          &value_string[1..value_string.len()-1]
        } else {
          &value_string[..]
        };
      }
      self.0 += format!(" {key}={value_str}").as_str();
      Ok(())
    }
  }
  move |buf : &mut env_logger::fmt::Formatter, record : &log::Record|{
    let mut kvv = KVVisitor::default();
    record.key_values().visit (&mut kvv).unwrap();
    let kvs = if kvv.0.is_empty() {
      "".to_string()
    } else {
      format!(" {}", kvv.0)
    };
    let ts = match config.timestamp_precision {
      env_logger::TimestampPrecision::Seconds => buf.timestamp_seconds(),
      env_logger::TimestampPrecision::Millis  => buf.timestamp_millis(),
      env_logger::TimestampPrecision::Micros  => buf.timestamp_micros(),
      env_logger::TimestampPrecision::Nanos   => buf.timestamp_nanos()
    };
    if !config.file && !config.thread && !config.target {
      let level_string = format!("{}:", record.level());
      writeln!(buf, "{ts} {level_string:6} {}{kvs}", record.args())
    } else {
      let thread_string = if config.thread {
        thread::current().name().map_or_else (
          || format!(" {:?}", thread::current().id())
            .replace ("ThreadId", "unnamed"),
          |name| format!(" {name}"))
      } else {
        "".to_string()
      };
      let target_string = if config.target {
        format!(" {}", record.target())
      } else {
        "".to_string()
      };
      let file_string = if config.file {
        format!(" {}:{}",
          record.file().unwrap_or ("<unknown>"), record.line().unwrap_or (0))
      } else {
        "".to_string()
      };
      writeln!(buf, "{ts} {:5}{}{}{}: {}{kvs}",
        record.level(), thread_string, target_string, file_string, record.args())
    }
  }
}

impl EnvLoggerFormatConfig {
  pub const fn thread (&mut self, thread : bool) -> &mut Self {
    self.thread = thread;
    self
  }

  pub const fn target (&mut self, target : bool) -> &mut Self {
    self.target = target;
    self
  }

  pub const fn file (&mut self, file : bool) -> &mut Self {
    self.file = file;
    self
  }

  pub const fn timestamp_precision (&mut self, precision : env_logger::TimestampPrecision)
    -> &mut Self
  {
    self.timestamp_precision = precision;
    self
  }

  pub const fn build (&mut self) -> Self {
    *self
  }
}

impl Default for EnvLoggerFormatConfig {
  fn default() -> Self {
    EnvLoggerFormatConfig {
      thread: true,
      target: true,
      file:   true,
      timestamp_precision: env_logger::TimestampPrecision::Seconds
    }
  }
}
