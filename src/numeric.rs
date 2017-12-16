use ::std;

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
