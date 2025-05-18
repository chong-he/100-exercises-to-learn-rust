// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

// Copy is required because the same variables used multiple times in the integration.rs test
// When Copy is required, Clone also required
// PartialEq is required because got assert_eq called on this type (SaturatingU16) for comparison
// Without this PartialEq, Rust wouldn't know how to compare SaturatingU16 to u16
// Debug is required so that it can be formatted using debug {:?} if the test failed
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct SaturatingU16 {
    value: u16,
}

// We still need the below impl, why?
// The above PartialEq trait implementation on SaturatingU16 only works for comparing between 2 instances of the same type
// i.e., 2 instances of SaturatingU16
// What if we want to compare SaturatingU16 to a different type, say u16?
// because the test got this: assert_eq!(a + a, 20u16);
// where a is SaturatingU16, and 20 is u16, so how can we compare? We can't
// Therefore, we need to implement PartialEq<u16> for SaturatingU16 type, i.e., compare 2 different types
// https://doc.rust-lang.org/std/cmp/trait.PartialEq.html#how-can-i-compare-two-different-types
// we see the "other" input here is of type u16, and we implement for SaturatingU16
// so that we can now compare SaturatingU16 to u16
impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

// How to know we need to have impl From trait?
// From the integration.rs, we see .into() being called, so into and From are like together
// Once we implement From, then we can call .into()
// The above says: It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
// so we need to impl From these 4 types
impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 {
            value: value as u16,
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16 { value: *value }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 {
            value: *value as u16,
        }
    }
}

// Implement Add for the same type, no need to have Rhs
impl Add for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: SaturatingU16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(other.value),
        }
    }
}

// Implement Add for 2 types, see Rustbook Sec 20.2
// how we can add 2 different types
// The Output type is always SaturatingU16, meaning after the Add operation, we get back SaturatingU16 type
impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: u16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(other),
        }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &u16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(*other),
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &SaturatingU16) -> SaturatingU16 {
        SaturatingU16 {
            value: self.value.saturating_add(other.value),
        }
    }
}
