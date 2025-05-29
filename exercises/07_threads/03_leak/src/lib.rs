// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // Ref: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.leak
    // From the doc, we see that the output is: pub fn leak<'a>(self) -> &'a mut [T]
    // i.e., the output is a &[T], in this case, T = i32, so &mut[i32]
    let slice = v.leak();
    let mid = slice.len() / 2;
    let (v1, v2) = slice.split_at(mid);

    let thread1 = thread::spawn(move || v1.iter().sum::<i32>());
    let thread2 = thread::spawn(move || v2.iter().sum::<i32>());

    thread1.join().unwrap() + thread2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
