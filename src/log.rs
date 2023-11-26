//! Logging macros

#[doc(hidden)] extern crate log;
#[doc(hidden)] extern crate stdext;

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
