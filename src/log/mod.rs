//! Logging macros

pub mod kv;

pub use log::{log, Level, LevelFilter};

/// Log trace message that includes the enclosing function name in the target
pub macro trace {
  ($($args:tt)*) => {
    $crate::log_::trace!(target: $crate::stdext::function_name!(), $($args)*)
  }
}

/// Log debug message that includes the enclosing function name in the target
pub macro debug {
  ($($args:tt)*) => {
    $crate::log_::debug!(target: $crate::stdext::function_name!(), $($args)*)
  }
}

/// Log info message that includes the enclosing function name in the target
pub macro info {
  ($($args:tt)*) => {
    $crate::log_::info!(target: $crate::stdext::function_name!(), $($args)*)
  }
}

/// Log warn message that includes the enclosing function name in the target
pub macro warn {
  ($($args:tt)*) => {
    $crate::log_::warn!(target: $crate::stdext::function_name!(), $($args)*)
  }
}

/// Log error message that includes the enclosing function name in the target
pub macro error {
  ($($args:tt)*) => {
    $crate::log_::error!(target: $crate::stdext::function_name!(), $($args)*)
  }
}
