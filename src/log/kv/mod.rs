//! The usual key-value format for log macros is to put the keys before the
//! message, the macros in this module reverses this order so that the log
//! message comes before the key/value pairs.

#[cfg(feature="env-logger-format")]
pub mod format;
#[cfg(feature="env-logger-format")]
pub use self::format::*;

pub use log::LevelFilter;

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
