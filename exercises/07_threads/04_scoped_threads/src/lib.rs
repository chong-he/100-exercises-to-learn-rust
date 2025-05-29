// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let midpoint = v.len() / 2;

    thread::scope(|scope| {
        // sum1 is a JoinHandle (not the actual sum of the elements of the vector)
        // this only runs the background to compute the sum
        let sum1 = scope.spawn(|| {
            let v1 = &v[0..midpoint];
            v1.iter().sum()
        });
        // need to call .join() to become sum
        // is like calling .join() to "redeem" the actual result, which is the sum
        let t1: i32 = sum1.join().unwrap();

        let sum2 = scope.spawn(|| {
            let v2 = &v[midpoint..];
            v2.iter().sum()
        });
        let t2: i32 = sum2.join().unwrap();
        t1 + t2
    })
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
