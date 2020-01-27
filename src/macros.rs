//! Utility macros

/// Expands into a series of for loops with the given block of code which should
/// treat each type uniformly.
///
/// This is to avoid dynamic dispatch when chaining iterators over generic
/// collections when the types are known.
pub macro for_sequence {
  ( $pattern:pat in ($($iter:expr),+) $do:block
  ) => {
    $(for $pattern in $iter $do)+
  }
}

/// Print the stringified expression followed by its debug formatting
pub macro show {
  ($e:expr) => {
    println!("{}: {:?}", stringify!($e), $e);
  }
}

/// Print the stringified expression followed by its pretty debug formatting
pub macro pretty {
  ($e:expr) => {
    println!("{}: {:#?}", stringify!($e), $e);
  }
}

/// Print the stringified expression followed by its display formatting
pub macro display {
  ($e:expr) => {
    println!("{}: {}", stringify!($e), $e);
  }
}

/// Print the stringified expression followed by its bitstring formatting
pub macro bits {
  ($e:expr) => {
    let e = $e;
    println!("{}: {:02$b}", stringify!($e), e, 8 * std::mem::size_of_val (&e));
  }
}

/// Print the stringified expression followed by its hexadecimal formatting
pub macro hex {
  ($e:expr) => {
    println!("{}: {:x}", stringify!($e), $e);
  }
}

/// Print the stringified expression followed by its pointer formatting
pub macro address {
  ($e:expr) => {
    println!("{}: {:p}", stringify!($e), $e);
  }
}
