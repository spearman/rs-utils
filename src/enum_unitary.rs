extern crate num;

use ::std;

//
//  enum_unitary!
//
/// Wraps "unitary" enums (i.e.  enums where variants do not have payloads)
/// with `enum_derive` and additionally implements the `EnumUnitary` trait
/// which includes implementations of `num::Bounded`, `num::ToPrimitive`,
/// `num::FromPrimitive`, a `count_variants` trait method and a `count`
/// constant non-trait method that both return the number of variants in the
/// enum, and a trait method `iter_variants` that exposes the function provided
/// from `enum_derive`.
///
/// Currently the deriving attribute is fixed and can not be overridden, and
/// explicit discriminators are not allowed: enum variants will be numbered
/// starting from `0`.

#[macro_export]
macro_rules! enum_unitary {
  //
  //  singleton: private
  //
  (
    enum $enum:ident ($iter:ident) { $singleton:ident }
  ) => {

    macro_attr!{
      #[derive (Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd,
        IterVariants!($iter),NextVariant!,PrevVariant!)]
      enum $enum {
        $singleton=0
      }
    }

    impl num::Bounded for $enum {
      fn min_value() -> Self {
        $enum::$singleton
      }
      fn max_value() -> Self {
        $enum::$singleton
      }
    }

    impl num::FromPrimitive for $enum {
      fn from_i64 (x : i64) -> Option <Self> {
        match x {
          0 => Some ($enum::$singleton),
          _ => None
        }
      }
      fn from_u64 (x: u64) -> Option <Self> {
        match x {
          0 => Some ($enum::$singleton),
          _ => None
        }
      }
    }

    impl num::ToPrimitive for $enum {
      fn to_i64 (&self) -> Option <i64> {
        Some (self.clone() as i64)
      }
      fn to_u64 (&self) -> Option <u64> {
        Some (self.clone() as u64)
      }
    }

    impl rs_utils::EnumUnitary for $enum {
      fn count_variants() -> usize {
        Self::count()
      }
      fn iter_variants() -> Box <Iterator <Item=Self>> {
        Box::new (Self::iter_variants())
      }
    }

    impl $enum {
      const fn count() -> usize {
        1
      }
    }

  };

  //
  //  singleton: public
  //
  (
    pub enum $enum:ident ($iter:ident) { $singleton:ident }
  ) => {

    macro_attr!{
      #[derive (Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd,
        IterVariants!($iter),NextVariant!,PrevVariant!)]
      pub enum $enum {
        $singleton=0
      }
    }

    impl num::Bounded for $enum {
      fn min_value() -> Self {
        $enum::$singleton
      }
      fn max_value() -> Self {
        $enum::$singleton
      }
    }

    impl num::FromPrimitive for $enum {
      fn from_i64 (x : i64) -> Option <Self> {
        match x {
          0 => Some ($enum::$singleton),
          _ => None
        }
      }
      fn from_u64 (x: u64) -> Option <Self> {
        match x {
          0 => Some ($enum::$singleton),
          _ => None
        }
      }
    }

    impl num::ToPrimitive for $enum {
      fn to_i64 (&self) -> Option <i64> {
        Some (self.clone() as i64)
      }
      fn to_u64 (&self) -> Option <u64> {
        Some (self.clone() as u64)
      }
    }

    impl rs_utils::EnumUnitary for $enum {
      fn count_variants() -> usize {
        Self::count()
      }
      fn iter_variants() -> Box <Iterator <Item=Self>> {
        Box::new (Self::iter_variants())
      }
    }

    impl $enum {
      const fn count() -> usize {
        1
      }
    }

  };

  //
  //  2 or more variants: private
  //
  (
    enum $enum:ident ($iter:ident) { $first:ident$(, $variant:ident)+ }
  ) => {
    enum_unitary!{
      enum $enum ($iter) {$first} {$($variant),+}
    }
  };

  (
    enum $enum:ident ($iter:ident)
      {$($variant:ident),+} {$more:ident$(, $tail:ident)+}
  ) => {
    enum_unitary!{
      enum $enum ($iter) {$($variant,)+ $more} {$($tail),+}
    }
  };

  (
    enum $enum:ident ($iter:ident)
      {$min:ident$(, $variant:ident)*} {$max:ident}
  ) => {

    macro_attr!{
      #[derive (Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd,
        IterVariants!($iter),NextVariant!,PrevVariant!)]
      enum $enum {
        $min=0$(, $variant)*, $max
      }
    }

    impl num::Bounded for $enum {
      fn min_value() -> Self {
        $enum::$min
      }
      fn max_value() -> Self {
        $enum::$max
      }
    }

    impl num::FromPrimitive for $enum {
      fn from_i64 (x : i64) -> Option <Self> {
        const VARIANTS : [$enum; $enum::count()]
          = [$enum::$min$(, $enum::$variant)*, $enum::$max];
        VARIANTS.get (x as usize).cloned()
      }
      fn from_u64 (x: u64) -> Option <Self> {
        const VARIANTS : [$enum; $enum::count()]
          = [$enum::$min$(, $enum::$variant)*, $enum::$max];
        VARIANTS.get (x as usize).cloned()
      }
    }

    impl num::ToPrimitive for $enum {
      fn to_i64 (&self) -> Option <i64> {
        Some (self.clone() as i64)
      }
      fn to_u64 (&self) -> Option <u64> {
        Some (self.clone() as u64)
      }
    }

    impl rs_utils::EnumUnitary for $enum {
      fn count_variants() -> usize {
        Self::count()
      }
      fn iter_variants() -> Box <(Iterator <Item=Self>)> {
        Box::new (Self::iter_variants())
      }
    }

    impl $enum {
      const fn count() -> usize {
        $enum::$max as usize + 1
      }
    }

  };

  //
  //  2 or more variants: public
  //
  (
    pub enum $enum:ident ($iter:ident) { $first:ident$(, $variant:ident)+ }
  ) => {
    enum_unitary!{
      pub enum $enum ($iter) {$first} {$($variant),+}
    }
  };

  (
    pub enum $enum:ident ($iter:ident)
      {$($variant:ident),+} {$more:ident$(, $tail:ident)+}
  ) => {
    enum_unitary!{
      pub enum $enum ($iter) {$($variant,)+ $more} {$($tail),+}
    }
  };

  (
    pub enum $enum:ident ($iter:ident)
      {$min:ident$(, $variant:ident)*} {$max:ident}
  ) => {

    macro_attr!{
      #[derive (Clone,Copy,Debug,Eq,PartialEq,Ord,PartialOrd,
        IterVariants!($iter),NextVariant!,PrevVariant!)]
      pub enum $enum {
        $min=0$(, $variant)*, $max
      }
    }

    impl num::Bounded for $enum {
      fn min_value() -> Self {
        $enum::$min
      }
      fn max_value() -> Self {
        $enum::$max
      }
    }

    impl num::FromPrimitive for $enum {
      fn from_i64 (x : i64) -> Option <Self> {
        const VARIANTS : [$enum; $enum::count()]
          = [$enum::$min$(, $enum::$variant)*, $enum::$max];
        VARIANTS.get (x as usize).cloned()
      }
      fn from_u64 (x: u64) -> Option <Self> {
        const VARIANTS : [$enum; $enum::count()]
          = [$enum::$min$(, $enum::$variant)*, $enum::$max];
        VARIANTS.get (x as usize).cloned()
      }
    }

    impl num::ToPrimitive for $enum {
      fn to_i64 (&self) -> Option <i64> {
        Some (self.clone() as i64)
      }
      fn to_u64 (&self) -> Option <u64> {
        Some (self.clone() as u64)
      }
    }

    impl rs_utils::EnumUnitary for $enum {
      fn count_variants() -> usize {
        Self::count()
      }
      fn iter_variants() -> Box <Iterator <Item=Self>> {
        Box::new (Self::iter_variants())
      }
    }

    impl $enum {
      const fn count() -> usize {
        $enum::$max as usize + 1
      }
    }

  };

}

//
//  trait EnumUnitary
//
// TODO: expose more constraints ?
pub trait EnumUnitary : Copy + Clone + Eq + Ord + PartialEq + PartialOrd
  + Send + Sync + std::fmt::Debug
  + num::Bounded + num::ToPrimitive + num::FromPrimitive
{
  fn count_variants() -> usize;
  fn iter_variants() -> Box <Iterator <Item=Self>>;
  // TODO: expose more methods from enum_derive ?
}

//
//  mod tests
//
#[cfg(test)]
mod tests {
  extern crate num;
  use ::enum_unitary as rs_utils;

  #[test]
  fn test_unit() {
    use super::EnumUnitary;
    use self::num::Bounded;
    use self::num::{FromPrimitive,ToPrimitive};

    // private enum
    enum_unitary!{
      enum Myenum1 (Myenum1Variants) {
        A, B, C
      }
    }
    assert_eq!(Myenum1::count(), 3);
    assert_eq!(Myenum1::count_variants(), 3);
    assert_eq!(Myenum1::A as usize, 0);
    assert_eq!(Myenum1::B as usize, 1);
    assert_eq!(Myenum1::C as usize, 2);
    assert_eq!(Some (Myenum1::A), Myenum1::from_usize (0));
    assert_eq!(Some (Myenum1::B), Myenum1::from_usize (1));
    assert_eq!(Some (Myenum1::C), Myenum1::from_usize (2));
    assert_eq!(None, Myenum1::from_usize (3));
    assert_eq!(Some (0), Myenum1::A.to_usize());
    assert_eq!(Some (1), Myenum1::B.to_usize());
    assert_eq!(Some (2), Myenum1::C.to_usize());
    assert_eq!(Myenum1::min_value(), Myenum1::A);
    assert_eq!(Myenum1::max_value(), Myenum1::C);
    let mut i = Myenum1::iter_variants();
    assert_eq!(i.next(), Some (Myenum1::A));
    assert_eq!(i.next(), Some (Myenum1::B));
    assert_eq!(i.next(), Some (Myenum1::C));
    assert_eq!(i.next(), None);
    assert_eq!(Myenum1::A.next_variant(), Some (Myenum1::B));
    assert_eq!(Myenum1::A.prev_variant(), None);
    assert_eq!(Myenum1::B.next_variant(), Some (Myenum1::C));
    assert_eq!(Myenum1::B.prev_variant(), Some (Myenum1::A));
    assert_eq!(Myenum1::C.next_variant(), None);
    assert_eq!(Myenum1::C.prev_variant(), Some (Myenum1::B));

    // public enum
    enum_unitary!{
      enum Myenum2 (Myenum2Variants) {
        A, B, C
      }
    }
    assert_eq!(Myenum2::count(), 3);
    assert_eq!(Myenum2::count_variants(), 3);
    assert_eq!(Myenum2::A as usize, 0);
    assert_eq!(Myenum2::B as usize, 1);
    assert_eq!(Myenum2::C as usize, 2);
    assert_eq!(Some (Myenum2::A), Myenum2::from_usize (0));
    assert_eq!(Some (Myenum2::B), Myenum2::from_usize (1));
    assert_eq!(Some (Myenum2::C), Myenum2::from_usize (2));
    assert_eq!(None, Myenum2::from_usize (3));
    assert_eq!(Some (0), Myenum2::A.to_usize());
    assert_eq!(Some (1), Myenum2::B.to_usize());
    assert_eq!(Some (2), Myenum2::C.to_usize());
    assert_eq!(Myenum2::min_value(), Myenum2::A);
    assert_eq!(Myenum2::max_value(), Myenum2::C);
    let mut i = Myenum2::iter_variants();
    assert_eq!(i.next(), Some (Myenum2::A));
    assert_eq!(i.next(), Some (Myenum2::B));
    assert_eq!(i.next(), Some (Myenum2::C));
    assert_eq!(i.next(), None);
    assert_eq!(Myenum2::A.next_variant(), Some (Myenum2::B));
    assert_eq!(Myenum2::A.prev_variant(), None);
    assert_eq!(Myenum2::B.next_variant(), Some (Myenum2::C));
    assert_eq!(Myenum2::B.prev_variant(), Some (Myenum2::A));
    assert_eq!(Myenum2::C.next_variant(), None);
    assert_eq!(Myenum2::C.prev_variant(), Some (Myenum2::B));

    // private singleton enum
    enum_unitary!{
      enum Myenum3 (Myenum3Variants) {
        X
      }
    }
    assert_eq!(Myenum3::count(), 1);
    assert_eq!(Myenum3::count_variants(), 1);
    assert_eq!(Myenum3::X as usize, 0);
    assert_eq!(Some (Myenum3::X), Myenum3::from_usize (0));
    assert_eq!(None, Myenum3::from_usize (1));
    assert_eq!(Some (0), Myenum3::X.to_usize());
    assert_eq!(Myenum3::min_value(), Myenum3::X);
    assert_eq!(Myenum3::max_value(), Myenum3::X);
    let mut i = Myenum3::iter_variants();
    assert_eq!(i.next(), Some (Myenum3::X));
    assert_eq!(i.next(), None);
    assert_eq!(Myenum3::X.next_variant(), None);
    assert_eq!(Myenum3::X.prev_variant(), None);

    // public singleton enum
    enum_unitary!{
      enum Myenum4 (Myenum4Variants) {
        X
      }
    }
    assert_eq!(Myenum4::count(), 1);
    assert_eq!(Myenum4::count_variants(), 1);
    assert_eq!(Myenum4::X as usize, 0);
    assert_eq!(Some (Myenum4::X), Myenum4::from_usize (0));
    assert_eq!(None, Myenum4::from_usize (1));
    assert_eq!(Some (0), Myenum4::X.to_usize());
    assert_eq!(Myenum4::min_value(), Myenum4::X);
    assert_eq!(Myenum4::max_value(), Myenum4::X);
    let mut i = Myenum4::iter_variants();
    assert_eq!(i.next(), Some (Myenum4::X));
    assert_eq!(i.next(), None);
    assert_eq!(Myenum4::X.next_variant(), None);
    assert_eq!(Myenum4::X.prev_variant(), None);
  }
}
