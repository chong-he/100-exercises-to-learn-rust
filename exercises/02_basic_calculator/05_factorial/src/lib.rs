// Define a function named `factorial` that, given a non-negative integer `n`,
// returns `n!`, the factorial of `n`.
//
// The factorial of `n` is defined as the product of all positive integers up to `n`.
// For example, `5!` (read "five factorial") is `5 * 4 * 3 * 2 * 1`, which is `120`.
// `0!` is defined to be `1`.
//
// We expect `factorial(0)` to return `1`, `factorial(1)` to return `1`,
// `factorial(2)` to return `2`, and so on.
//
// Use only what you learned! No loops yet, so you'll have to use recursion!
fn factorial(n: i32) -> i32 {
    if n == 0 {
        1
    } else if n == 1 {
        1
    } else {
        n * factorial(n - 1)
        // e.g., n =5, then it is 5*factorial(4). What is factorial(4)?
        // it is 4*factorial(3), so the original equation becomes: 5*4*factorial(3)
        // and so on, until it becomes 5*4*3*2*factorial(1)
        // what is factorial(1)? It's 1, as defined in the function
        // so in the end, when n = 5, it becomes: 5*4*3*2*1 = 120
        // because it hits the arm of factorial(1) (when n = 1), so it stops there
        // i.e., the loop doesn't go to factorial(0) or factorial(-1) and so on
    }
}
#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
