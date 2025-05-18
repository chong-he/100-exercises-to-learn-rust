// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

use std::ops::Add;

// Refer Chap4Sec11, when we have Copy, we must also always have Clone
// But the reverse is not true, we can have Clone alone without Copy
// Copy is a subtrait of Clone
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl Add for WrappingU32 {
    type Output = WrappingU32;
    fn add(self, other: WrappingU32) -> WrappingU32 {
        WrappingU32 {
            // use wrapping_add to avoid overflow
            value: self.value.wrapping_add(other.value),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        // because Copy is implemented, we don't have to explicitly do: y.clone()
        // if we remove the Copy above in the derive, we have to explicitly call y.clone()
        // With Copy, y is implicitly copied (automatically copied)
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
