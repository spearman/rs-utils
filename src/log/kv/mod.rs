//! The usual key-value format for log macros is to put the keys before the message, the
//! macros in this module reverse this order so that the log message comes before the
//! key/value pairs.

#[cfg(feature="env-logger-format")]
#[cfg_attr(docsrs, doc(cfg(feature="env-logger-format")))]
pub mod format;
#[cfg(feature="env-logger-format")]
#[cfg_attr(docsrs, doc(cfg(feature="env-logger-format")))]
pub use self::format::*;

pub use log::LevelFilter;

/// Log trace message with key values after the message instead of before
pub macro trace {
  ($fmtstring:expr$(, $fmtarg:expr)*$(; $($kvargs:tt)*)?) => {
    super::trace!($($($kvargs)*;)? $fmtstring$(, $fmtarg)*)
  }
}
/// Log debug message with key values after the message instead of before
pub macro debug {
  ($fmtstring:expr$(, $fmtarg:expr)*$(; $($kvargs:tt)*)?) => {
    super::debug!($($($kvargs)*;)? $fmtstring$(, $fmtarg)*)
  }
}
/// Log info message with key values after the message instead of before
pub macro info {
  ($fmtstring:expr$(, $fmtarg:expr)*$(; $($kvargs:tt)*)?) => {
    super::info!($($($kvargs)*;)? $fmtstring$(, $fmtarg)*)
  }
}
/// Log warn message with key values after the message instead of before
pub macro warn {
  ($fmtstring:expr$(, $fmtarg:expr)*$(; $($kvargs:tt)*)?) => {
    super::warn!($($($kvargs)*;)? $fmtstring$(, $fmtarg)*)
  }
}
/// Log error message with key values after the message instead of before
pub macro error {
  ($fmtstring:expr$(, $fmtarg:expr)*$(; $($kvargs:tt)*)?) => {
    super::error!($($($kvargs)*;)? $fmtstring$(, $fmtarg)*)
  }
}
