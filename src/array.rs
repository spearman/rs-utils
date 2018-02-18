//! Array utilities

use ::{generic_array, typenum};

use typenum::consts::*;
use generic_array::GenericArray;

/// Create a new array from the first elements of an array of pairs of the same
/// length as the first array.
///
/// ```
/// # extern crate generic_array;
/// # extern crate typenum;
/// # extern crate rs_utils;
/// # fn main() {
/// # use rs_utils::array::fst;
/// use generic_array::GenericArray;
/// use typenum::consts::*;
/// let xs : [(u8, f32); 2]        = [(5, 1.0), (100, -1.0)];
/// let ys : GenericArray <u8, U2> = fst (&xs.into());
/// assert_eq!(ys.as_slice(), [5, 100]);
/// # }
/// ```
///
/// A compilation error will result if the lengths of the arrays do not match:
///
/// ```compile_fail
/// # extern crate generic_array;
/// # extern crate typenum;
/// # extern crate rs_utils;
/// # fn main() {
/// # use rs_utils::array::fst;
/// use generic_array::GenericArray;
/// use typenum::consts::*;
/// let xs : [(u8, f32); 2]        = [(5, 1.0), (100, -1.0)];
/// let ys : GenericArray <u8, U1> = fst (&xs.into());
/// # }
/// ```

pub fn fst <A, B, N> (from : &GenericArray <(A,B), N>) -> GenericArray <A, N>
where
  A : Clone,
  N : generic_array::ArrayLength <(A,B)> + generic_array::ArrayLength <A>,
{
  GenericArray::generate (|i| from[i].0.clone())
}

/// Create a new array from the first elements of an array of pairs or the
/// default if the destination array length is longer than the input array
/// length.
///
/// ```
/// # extern crate generic_array;
/// # extern crate typenum;
/// # extern crate rs_utils;
/// # fn main() {
/// # use rs_utils::array::fst_default;
/// use generic_array::GenericArray;
/// use typenum::consts::*;
/// let xs : [(u8, f32); 2]        = [(5, 1.0), (100, -1.0)];
/// let ys : GenericArray <u8, U3> = fst_default (&xs.into());
/// assert_eq!(ys.as_slice(), [5, 100, 0]);
/// # }
/// ```

pub fn fst_default <A, B, N, M> (from : &GenericArray <(A,B), N>)
  -> GenericArray <A, M>
where
  A : Clone + Default,
  N : generic_array::ArrayLength <(A,B)>,
  M : generic_array::ArrayLength <A>
{
  GenericArray::generate (|i| if i < N::to_usize() {
    from[i].0.clone()
  } else {
    A::default()
  })
}

/// Create a new array from the first elements of an array of pairs of equal
/// or greater length than the first.
///
/// ```
/// # extern crate generic_array;
/// # extern crate typenum;
/// # extern crate rs_utils;
/// # fn main() {
/// # use rs_utils::array::fst_initial;
/// use generic_array::GenericArray;
/// use typenum::consts::*;
/// let xs : [(u8, f32); 2]        = [(5, 1.0), (100, -1.0)];
/// let ys : GenericArray <u8, U1> = fst_initial (&xs.into());
/// assert_eq!(ys.as_slice(), [5]);
/// # }
/// ```
///
/// A compilation error will result if the length of the input array is
/// shorter than the length of the destination array:
///
/// ```compile_fail
/// # extern crate generic_array;
/// # extern crate typenum;
/// # extern crate rs_utils;
/// # fn main() {
/// # use rs_utils::array::fst_initial;
/// use generic_array::GenericArray;
/// use typenum::consts::*;
/// let xs : [(u8, f32); 2]        = [(5, 1.0), (100, -1.0)];
/// let ys : GenericArray <u8, U3> = fst_initial (&xs.into());
/// # }
/// ```

pub fn fst_initial <A, B, N, M> (from : &GenericArray <(A,B), N>)
  -> GenericArray <A, M>
where
  A : Clone,
  N : generic_array::ArrayLength <(A,B)>,
  M : generic_array::ArrayLength <A> + typenum::IsLessOrEqual <N, Output=B1>
{
  GenericArray::generate (|i| from[i].0.clone())
}

/// Create a new array from the second elements of an array of pairs of the same
/// length as the second array.
///
/// ```
/// # extern crate generic_array;
/// # extern crate typenum;
/// # extern crate rs_utils;
/// # fn main() {
/// # use rs_utils::array::snd;
/// use generic_array::GenericArray;
/// use typenum::consts::*;
/// let xs : [(u8, f32); 2]        = [(5, 1.0), (100, -1.0)];
/// let ys : GenericArray <f32, U2> = snd (&xs.into());
/// assert_eq!(ys.as_slice(), [1.0, -1.0]);
/// # }
/// ```
///
/// A compilation error will result if the lengths of the arrays do not match:
///
/// ```compile_fail
/// # extern crate generic_array;
/// # extern crate typenum;
/// # extern crate rs_utils;
/// # fn main() {
/// # use rs_utils::array::snd;
/// use generic_array::GenericArray;
/// use typenum::consts::*;
/// let xs : [(u8, f32); 2]        = [(5, 1.0), (100, -1.0)];
/// let ys : GenericArray <f32, U1> = snd (&xs.into());
/// # }
/// ```

pub fn snd <A, B, N> (from : &GenericArray <(A,B), N>) -> GenericArray <B, N>
where
  B : Clone,
  N : generic_array::ArrayLength <(A,B)> + generic_array::ArrayLength <B>,
{
  GenericArray::generate (|i| from[i].1.clone())
}

/// Create a new array from the second elements of an array of pairs or the
/// default if the destination array length is longer than the input array
/// length.
///
/// ```
/// # extern crate generic_array;
/// # extern crate typenum;
/// # extern crate rs_utils;
/// # fn main() {
/// # use rs_utils::array::snd_default;
/// use generic_array::GenericArray;
/// use typenum::consts::*;
/// let xs : [(u8, f32); 2]        = [(5, 1.0), (100, -1.0)];
/// let ys : GenericArray <f32, U3> = snd_default (&xs.into());
/// assert_eq!(ys.as_slice(), [1.0, -1.0, 0.0]);
/// # }
/// ```

pub fn snd_default <A, B, N, M> (from : &GenericArray <(A,B), N>)
  -> GenericArray <B, M>
where
  B : Clone + Default,
  N : generic_array::ArrayLength <(A,B)>,
  M : generic_array::ArrayLength <B>
{
  GenericArray::generate (|i| if i < N::to_usize() {
    from[i].1.clone()
  } else {
    B::default()
  })
}

/// Create a new array from the second elements of an array of pairs of equal
/// or greater length than the second.
///
/// ```
/// # extern crate generic_array;
/// # extern crate typenum;
/// # extern crate rs_utils;
/// # fn main() {
/// # use rs_utils::array::snd_initial;
/// use generic_array::GenericArray;
/// use typenum::consts::*;
/// let xs : [(u8, f32); 2]        = [(5, 1.0), (100, -1.0)];
/// let ys : GenericArray <f32, U1> = snd_initial (&xs.into());
/// assert_eq!(ys.as_slice(), [1.0]);
/// # }
/// ```
///
/// A compilation error will result if the length of the input array is
/// shorter than the length of the destination array:
///
/// ```compile_fail
/// # extern crate generic_array;
/// # extern crate typenum;
/// # extern crate rs_utils;
/// # fn main() {
/// # use rs_utils::array::snd_initial;
/// use generic_array::GenericArray;
/// use typenum::consts::*;
/// let xs : [(u8, f32); 2]        = [(5, 1.0), (100, -1.0)];
/// let ys : GenericArray <f32, U3> = snd_initial (&xs.into());
/// # }
/// ```

pub fn snd_initial <A, B, N, M> (from : &GenericArray <(A,B), N>)
  -> GenericArray <B, M>
where
  B : Clone,
  N : generic_array::ArrayLength <(A,B)>,
  M : generic_array::ArrayLength <B> + typenum::IsLessOrEqual <N, Output=B1>
{
  GenericArray::generate (|i| from[i].1.clone())
}
