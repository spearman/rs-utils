//! Logging macros

#[doc(hidden)] extern crate log;
#[doc(hidden)] extern crate stdext;

#[cfg(feature="env-logger-format")]
pub use self::kv::{env_logger_custom_format, env_logger_json_format};

/// Log trace message that includes the enclosing function name in the target
pub macro trace {
  ($($args:tt)*) => {
    log::trace!(target: stdext::function_name!(), $($args)*)
  }
}

/// Log debug message that includes the enclosing function name in the target
pub macro debug {
  ($($args:tt)*) => {
    log::debug!(target: stdext::function_name!(), $($args)*)
  }
}

/// Log info message that includes the enclosing function name in the target
pub macro info {
  ($($args:tt)*) => {
    log::info!(target: stdext::function_name!(), $($args)*)
  }
}

/// Log warn message that includes the enclosing function name in the target
pub macro warn {
  ($($args:tt)*) => {
    log::warn!(target: stdext::function_name!(), $($args)*)
  }
}

/// Log error message that includes the enclosing function name in the target
pub macro error {
  ($($args:tt)*) => {
    log::error!(target: stdext::function_name!(), $($args)*)
  }
}

pub mod kv {
  //! The usual key-value format for log macros is to put the keys before the
  //! message, the macros in this module reverses this order so that the log
  //! message comes before the key/value pairs.

  pub macro trace {
    ($fmtstring:expr$(, $fmtarg:expr)*$(; $($kvargs:tt)*)?) => {
      super::trace!($($($kvargs)*;)? $fmtstring$(, $fmtarg)*)
    }
  }
  pub macro debug {
    ($fmtstring:expr$(, $fmtarg:expr)*$(; $($kvargs:tt)*)?) => {
      super::debug!($($($kvargs)*;)? $fmtstring$(, $fmtarg)*)
    }
  }
  pub macro info {
    ($fmtstring:expr$(, $fmtarg:expr)*$(; $($kvargs:tt)*)?) => {
      super::info!($($($kvargs)*;)? $fmtstring$(, $fmtarg)*)
    }
  }
  pub macro warn {
    ($fmtstring:expr$(, $fmtarg:expr)*$(; $($kvargs:tt)*)?) => {
      super::warn!($($($kvargs)*;)? $fmtstring$(, $fmtarg)*)
    }
  }
  pub macro error {
    ($fmtstring:expr$(, $fmtarg:expr)*$(; $($kvargs:tt)*)?) => {
      super::error!($($($kvargs)*;)? $fmtstring$(, $fmtarg)*)
    }
  }

  #[cfg(feature="env-logger-format")]
  /// Formats log messages as a json object string
  pub fn env_logger_json_format (
    buf    : &mut env_logger::fmt::Formatter,
    record : &log::Record
  ) -> std::io::Result<()> {
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
    let mut kvv = KVVisitor::default();
    record.key_values().visit (&mut kvv).unwrap();
    let thread = std::thread::current().name().map (str::to_string)
      .unwrap_or_else (||
        format!("{:?}",
          std::thread::current().id()).replace ("ThreadId", "unnamed"));
    writeln!(buf,
      "{{\"ts\":\"{}\",\"level\":\"{}\",\"thread\":\"{}\",\
        \"target\":\"{}\",\"file\":\"{}:{}\",\"msg\":\"{}\"{}}}",
      buf.timestamp(), record.level(), thread, record.target(),
      record.file().unwrap(), record.line().unwrap(), record.args(), kvv.0
    )
  }

  #[cfg(feature="env-logger-format")]
  /// A custom `env_logger` format adding thread, target, and file information:
  /// ```text
  /// <ts> <level> <thread> <target> <file>: <msg> [ <key>=<value>]
  /// ```
  pub fn env_logger_custom_format (
    buf    : &mut env_logger::fmt::Formatter,
    record : &log::Record
  ) -> std::io::Result<()> {
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
          // if there is now whitespace we can remove outer quotes and un-escape
          // any internal quotes
          value_string = value_str.replace("\\\"", "\"");
          value_str = &value_string[1..value_string.len()-1];
        }
        self.0 += format!(" {}={}", key, value_str).as_str();
        Ok(())
      }
    }
    let mut kvv = KVVisitor::default();
    record.key_values().visit (&mut kvv).unwrap();
    let thread = std::thread::current().name().map (str::to_string)
      .unwrap_or_else (||
        format!("{:?}",
          std::thread::current().id()).replace ("ThreadId", "unnamed"));
    writeln!(buf,
      "{} {:5} {} {} {}:{}: {} {}",
      buf.timestamp(), record.level(), thread, record.target(),
      record.file().unwrap(), record.line().unwrap(), record.args(), kvv.0
    )
  }
}
