//! Utility macros

/// Expands into a series of for loops with the given block of code which should
/// treat each type uniformly.
///
/// This is to avoid dynamic dispatch when chaining iterators over generic
/// collections when the types are known.
#[macro_export]
macro_rules! for_sequence {
  (
    $pattern:pat in $($collection:ident),* $do:block
  ) => {
    $(for $pattern in $collection $do)+
  }
}

/// Print the stringified expression followed by its debug formatting
#[macro_export]
macro_rules! show {
  ($e:expr) => {
    println!("{}: {:?}", stringify!($e), $e);
  }
}

/// Print the stringified expression followed by its pretty debug formatting
#[macro_export]
macro_rules! pretty {
  ($e:expr) => {
    println!("{}: {:#?}", stringify!($e), $e);
  }
}

/// Print the stringified expression followed by its display formatting
#[macro_export]
macro_rules! display {
  ($e:expr) => {
    println!("{}: {}", stringify!($e), $e);
  }
}

/// Print the stringified expression followed by its bitstring formatting
#[macro_export]
macro_rules! bits {
  ($e:expr) => {
    println!("{}: {:b}", stringify!($e), $e);
  }
}

/// Print the stringified expression followed by its hexadecimal formatting
#[macro_export]
macro_rules! hex {
  ($e:expr) => {
    println!("{}: {:x}", stringify!($e), $e);
  }
}

/// Print the stringified expression followed by its pointer formatting
#[macro_export]
macro_rules! address {
  ($e:expr) => {
    println!("{}: {:p}", stringify!($e), $e);
  }
}
