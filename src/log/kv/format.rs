//! Custom env_logger formats

use std;
use env_logger;
use log;

pub struct EnvLoggerFormatConfig {
  pub thread : bool,
  pub target : bool,
  pub file   : bool
}

/// Formats log messages as a json object string
pub fn env_logger_json_formatter (config : EnvLoggerFormatConfig) -> impl
  Fn(&mut env_logger::fmt::Formatter, &log::Record<'_>) -> std::io::Result<()>
{
  use std::io::Write;
  #[derive(Default)]
  struct KVVisitor(pub String);
  impl <'kvs> log::kv::VisitSource <'kvs> for KVVisitor {
    fn visit_pair (&mut self,
      key : log::kv::Key <'kvs>, value : log::kv::Value <'kvs>
    ) -> Result <(), log::kv::Error> {
      self.0 += format!(",\"{}\":{}", key, serde_json::to_string(&value).unwrap())
        .as_str();
      Ok(())
    }
  }
  move |buf : &mut env_logger::fmt::Formatter, record : &log::Record|{
    let thread_string = if config.thread {
      std::thread::current().name()
        .map (|name| format!(",\"thread\":\"{}\"", name))
        .unwrap_or_else (||
          format!(",\"thread\":\"{:?}\"",
            std::thread::current().id()).replace ("ThreadId", "unnamed"))
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
        record.file().unwrap(), record.line().unwrap())
    } else {
      "".to_string()
    };
    let mut kvv = KVVisitor::default();
    record.key_values().visit (&mut kvv).unwrap();
    writeln!(buf,
      "{{\"ts\":\"{}\",\"level\":\"{}\"{}{}{},\"msg\":\"{}\"{}}}",
      buf.timestamp(), record.level(), thread_string, target_string,
      file_string, record.args(), kvv.0
    )
  }
}

/// A custom `env_logger` format adding thread, target, and file information:
/// ```text
/// <ts> <level> <thread> <target> <file>: <msg> [ <key>=<value>]
/// ```
pub fn env_logger_custom_formatter (config : EnvLoggerFormatConfig) -> impl
  Fn(&mut env_logger::fmt::Formatter, &log::Record<'_>) -> std::io::Result<()>
{
  use std::io::Write;
  #[derive(Default)]
  struct KVVisitor(pub String);
  impl <'kvs> log::kv::VisitSource <'kvs> for KVVisitor {
    fn visit_pair (&mut self,
      key : log::kv::Key <'kvs>, value : log::kv::Value <'kvs>
    ) -> Result <(), log::kv::Error> {
      let mut value_string = serde_json::to_string (&value).unwrap();
      let mut value_str = value_string.as_str();
      if !value_str.contains (char::is_whitespace) {
        // if there is no whitespace we can remove outer quotes and un-escape
        // any internal quotes
        value_string = value_str.replace("\\\"", "\"");
        value_str = if value_string.chars().next() == Some('"') {
          &value_string[1..value_string.len()-1]
        } else {
          &value_string[..]
        };
      }
      self.0 += format!(" {}={}", key, value_str).as_str();
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
    if !config.file && !config.thread && !config.target {
      let level_string = format!("{}:", record.level());
      writeln!(buf,
        "{} {:6} {}{}", buf.timestamp(), level_string, record.args(), kvs)
    } else {
      let thread_string = if config.thread {
        std::thread::current().name().map (|name| format!(" {}", name))
          .unwrap_or_else (||
            format!(" {:?}",
              std::thread::current().id()).replace ("ThreadId", "unnamed"))
      } else {
        "".to_string()
      };
      let target_string = if config.target {
        format!(" {}", record.target())
      } else {
        "".to_string()
      };
      let file_string = if config.file {
        format!(" {}:{}", record.file().unwrap(), record.line().unwrap())
      } else {
        "".to_string()
      };
      writeln!(buf,
        "{} {:5}{}{}{}: {}{}",
        buf.timestamp(), record.level(), thread_string, target_string,
        file_string, record.args(), kvs)
    }
  }
}

impl Default for EnvLoggerFormatConfig {
  fn default() -> Self {
    EnvLoggerFormatConfig {
      thread: true,
      target: true,
      file:   true
    }
  }
}
