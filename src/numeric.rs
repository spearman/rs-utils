//! Numeric utilities

use {std, rand};

/// Create an XorShiftRng with a default seed.
///
/// Previously this was possible with the deprecated `XorShiftRng::new_unseeded`
/// method, this function uses the same seed as was used previously.
/// ```
/// # extern crate rand;
/// # extern crate rs_utils;
/// # fn main() {
/// # use rs_utils::numeric::xorshift_rng_unseeded;
/// use rand::RngCore;
/// let mut xorshift = xorshift_rng_unseeded();
/// let mut fill_bytes = [0u8; 16];
/// xorshift.fill_bytes (&mut fill_bytes[..]);
/// assert_eq!(fill_bytes, [
///   0x8e, 0x69, 0x7d, 0xd6,
///   0x0e, 0x7d, 0x58, 0x1a,
///   0xe9, 0x57, 0x3b, 0x6b,
///   0x2c, 0x96, 0xc6, 0xe9
/// ]);
/// # }
/// ```
pub fn xorshift_rng_unseeded() -> rand::prng::XorShiftRng {
  use rand::SeedableRng;
  rand::prng::XorShiftRng::from_seed ([
    0x19, 0x3a, 0x67, 0x54,
    0xa8, 0xa7, 0xd4, 0x69,
    0x97, 0x83, 0x0e, 0x05,
    0x11, 0x3b, 0xa7, 0xbb
  ])
}

/// Returns the minimum of two partially ordered values, returning the rhs when
/// they are incomparable.
///
/// This follows the convention of `f32::min` and `f32::max`, but the opposite
/// convention is used internally by the `collision` crate.
pub fn min_partial <S : PartialOrd + Copy> (lhs : S, rhs : S) -> S {
  match lhs.partial_cmp (&rhs) {
    Some (std::cmp::Ordering::Less) | Some (std::cmp::Ordering::Equal)
      => lhs,
    _ => rhs
  }
}

/// Returns the maximum of two partially ordered values, returning the rhs when
/// they are incomparable.
///
/// This follows the convention of `f32::min` and `f32::max`, but the opposite
/// convention is used internally by the `collision` crate.
pub fn max_partial <S : PartialOrd + Copy> (lhs : S, rhs : S) -> S {
  match lhs.partial_cmp (&rhs) {
    Some (std::cmp::Ordering::Greater) | Some (std::cmp::Ordering::Equal)
      => lhs,
    _ => rhs
  }
}
