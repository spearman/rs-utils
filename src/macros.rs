//! Utility macros

/// Print the stringified expression followed by its debug formatting
pub macro show {
  ($e:expr) => {
    println!("{}: {:?}", stringify!($e), $e);
  }
}

/// Print the stringified expression to stderr followed by its debug formatting
pub macro eshow {
  ($e:expr) => {
    eprintln!("{}: {:?}", stringify!($e), $e);
  }
}

/// Print the stringified expression followed by its pretty debug formatting
pub macro pretty {
  ($e:expr) => {
    println!("{}: {:#?}", stringify!($e), $e);
  }
}

/// Print the stringified expression to stderr followed by its pretty debug formatting
pub macro epretty {
  ($e:expr) => {
    eprintln!("{}: {:#?}", stringify!($e), $e);
  }
}

/// Print the stringified expression followed by its display formatting
pub macro display {
  ($e:expr) => {
    println!("{}: {}", stringify!($e), $e);
  }
}

/// Print the stringified expression to stderr followed by its display formatting
pub macro edisplay {
  ($e:expr) => {
    eprintln!("{}: {}", stringify!($e), $e);
  }
}

/// Print the stringified expression followed by its bitstring formatting
pub macro bits {
  ($e:expr) => {
    let e = $e;
    println!("{}: {:02$b}", stringify!($e), e, 8 * core::mem::size_of_val (&e));
  }
}

/// Print the stringified expression to stderr followed by its bitstring formatting
pub macro ebits {
  ($e:expr) => {
    let e = $e;
    eprintln!("{}: {:02$b}", stringify!($e), e, 8 * core::mem::size_of_val (&e));
  }
}

/// Print the stringified expression followed by its hexadecimal formatting
pub macro hex {
  ($e:expr) => {
    println!("{}: {:x}", stringify!($e), $e);
  }
}

/// Print the stringified expression to stderr followed by its hexadecimal formatting
pub macro ehex {
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

/// Print the stringified expression to stderr followed by its pointer formatting
pub macro eaddress {
  ($e:expr) => {
    eprintln!("{}: {:p}", stringify!($e), $e);
  }
}
