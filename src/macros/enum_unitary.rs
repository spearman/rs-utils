//! Exports a macro `enum_unitary!` that wraps "unitary" enums (i.e.  enums
//! where variants do not have payloads) with `enum_derive` and additionally
//! provides a few more conveniences:
//!
//! - a constant function `count()` that returns the number of variants in the
//!   enum
//! - a `num::Bounded` implementation (provides `min_value()`, `max_value()`
//!   functions)
//! - a `num::FromPrimitive` implementation (provides `from_*` functions for
//!   signed and unsigned integers)
//!
//! Attributes provided are passed through to the inner invocation of
//! `macro_attr!` used to define the enum where the following macros are
//! available for deriving (see `enum_derive` crate for details):
//!
//! - `EnumDisplay!`
//! - `EnumFromInner!`
//! - `EnumFromStr!`
//! - `EnumInnerAsTrait!`
//! - `IterVariantNames!`
//! - `IterVariants!`
//! - `NextVariant!`
//! - `PrevVariant!`
//!
//! Currently explicit discriminators are not allowed, enum variants will be
//! numbered starting from `0`.

#[macro_export]
macro_rules! enum_unitary {
  //
  //  singleton
  //
  ( #$attrs:tt
    enum $enum:ident { $singleton:ident }
  ) => {

    macro_attr!{
      #$attrs
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

    impl $enum {
      const fn count() -> usize {
        1
      }
    }

  };

  ( #$attrs:tt
    pub enum $enum:ident { $singleton:ident }
  ) => {

    macro_attr!{
      #$attrs
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

    impl $enum {
      const fn count() -> usize {
        1
      }
    }

  };

  //
  //  2 or more variants: minimal syntax (;)
  //
  ( #$attrs:tt
    enum $enum:ident { $min:ident$(, $variant:ident)*; $max:ident }
  ) => {

    macro_attr!{
      #$attrs
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

    impl $enum {
      const fn count() -> usize {
        $enum::$max as usize + 1
      }
    }

  };

  ( #$attrs:tt
    pub enum $enum:ident { $min:ident$(, $variant:ident)*; $max:ident }
    ) => {

    macro_attr!{
      #$attrs
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

    impl $enum {
      const fn count() -> usize {
        $enum::$max as usize + 1
      }
    }

  };

}

#[cfg(test)]
mod tests {
  extern crate num;

  #[test]
  fn test_unit() {
    use self::num::Bounded;
    use self::num::FromPrimitive;

    // private enum
    enum_unitary!{
      #[derive(Copy,Clone,Debug,Eq,PartialEq,NextVariant!,PrevVariant!,
        IterVariants!(Myenum1Variants))]
      enum Myenum1 {
        A, B; C
      }
    }
    assert_eq!(Myenum1::count(), 3);
    assert_eq!(Myenum1::A as usize, 0);
    assert_eq!(Myenum1::B as usize, 1);
    assert_eq!(Myenum1::C as usize, 2);
    assert_eq!(Some (Myenum1::A), Myenum1::from_usize (0));
    assert_eq!(Some (Myenum1::B), Myenum1::from_usize (1));
    assert_eq!(Some (Myenum1::C), Myenum1::from_usize (2));
    assert_eq!(None, Myenum1::from_usize (3));
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
      #[derive(Copy,Clone,Debug,Eq,PartialEq,NextVariant!,PrevVariant!,
        IterVariants!(Myenum2Variants))]
      pub enum Myenum2 {
        A, B; C
      }
    }
    assert_eq!(Myenum2::count(), 3);
    assert_eq!(Myenum2::A as usize, 0);
    assert_eq!(Myenum2::B as usize, 1);
    assert_eq!(Myenum2::C as usize, 2);
    assert_eq!(Some (Myenum2::A), Myenum2::from_usize (0));
    assert_eq!(Some (Myenum2::B), Myenum2::from_usize (1));
    assert_eq!(Some (Myenum2::C), Myenum2::from_usize (2));
    assert_eq!(None, Myenum2::from_usize (3));
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
      #[derive(Copy,Clone,Debug,Eq,PartialEq,NextVariant!,PrevVariant!,
        IterVariants!(Myenum3Variants))]
      enum Myenum3 {
        X
      }
    }
    assert_eq!(Myenum3::count(), 1);
    assert_eq!(Myenum3::X as usize, 0);
    assert_eq!(Some (Myenum3::X), Myenum3::from_usize (0));
    assert_eq!(None, Myenum3::from_usize (1));
    assert_eq!(Myenum3::min_value(), Myenum3::X);
    assert_eq!(Myenum3::max_value(), Myenum3::X);
    let mut i = Myenum3::iter_variants();
    assert_eq!(i.next(), Some (Myenum3::X));
    assert_eq!(i.next(), None);
    assert_eq!(Myenum3::X.next_variant(), None);
    assert_eq!(Myenum3::X.prev_variant(), None);

    // public singleton enum
    enum_unitary!{
      #[derive(Copy,Clone,Debug,Eq,PartialEq,NextVariant!,PrevVariant!,
        IterVariants!(Myenum4Variants))]
      pub enum Myenum4 {
        X
      }
    }
    assert_eq!(Myenum4::count(), 1);
    assert_eq!(Myenum4::X as usize, 0);
    assert_eq!(Some (Myenum4::X), Myenum4::from_usize (0));
    assert_eq!(None, Myenum4::from_usize (1));
    assert_eq!(Myenum4::min_value(), Myenum4::X);
    assert_eq!(Myenum4::max_value(), Myenum4::X);
    let mut i = Myenum4::iter_variants();
    assert_eq!(i.next(), Some (Myenum4::X));
    assert_eq!(i.next(), None);
    assert_eq!(Myenum4::X.next_variant(), None);
    assert_eq!(Myenum4::X.prev_variant(), None);
  }
}
