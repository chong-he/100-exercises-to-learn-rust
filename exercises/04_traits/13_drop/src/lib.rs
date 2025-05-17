// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

struct DropBomb {
    defused: bool,
}

impl Drop for DropBomb {
    // note that with impl Drop, we need to have the "drop" method defined
    // In this exercise, we define a specific behaviour of the drop method, i.e., to panic under a condition
    fn drop(&mut self) {
        // when the bomb is not defused, then we should panic
        if self.defused == false {
            panic!("panic!")
        }
    }
}

impl DropBomb {
    // this is an associated function, to create a new DropBomb struct
    fn new() -> Self {
        // By default, the bomb starts with defused = false (so it should panic when called)
        // the only way to prevent the panic is to call the defused method
        Self { defused: false }
    }

    // this is a method
    fn defuse(&mut self) {
        // when this method is called, change the defused field to true
        self.defused = true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
