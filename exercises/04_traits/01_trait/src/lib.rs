// Define a trait named `IsEven` that has a method `is_even` that returns a `true` if `self` is
// even, otherwise `false`.
//
// Then implement the trait for `u32` and `i32`.

trait IsEven {
    fn is_even(&self) -> bool;
}

// u32 is a primitive type is Rust, so no need to define a struct/type for it
// with the impl IsEven for u32
// now u32 has IsEven trait implemented on it
// so u32 can now call the .is_even() method (it can't without this implementation)
// the same goes for i32
// we can't call is_even e.g., on u8 or u64, because we didn't implement IsEven for the type
// e.g., change the 42u32 in the test to 42u64, then we get error:
// error[E0599]: no method named `is_even` found for type `u64` in the current scope
impl IsEven for u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_is_even() {
        assert!(42u64.is_even());
        assert!(!43u32.is_even());
    }

    #[test]
    fn test_i32_is_even() {
        assert!(42i32.is_even());
        assert!(!43i32.is_even());
        assert!(0i32.is_even());
        assert!(!(-1i32).is_even());
    }
}
