//! Make the tests pass: use `std::iter::Iterator` methods.
#![allow(dead_code)]

use std::ops::Range;

fn func(vals: &(usize, usize)) -> bool {
    (vals.0 + vals.1).is_power_of_two()
}

fn imperative(a: Range<usize>, b: Range<usize>) -> usize {
    let mut count = 0;
    let mut a = a;
    let mut b = b;

    loop {
        if let Some(a) = a.next() {
            if let Some(b) = b.next() {
                if func(&(a, b)) {
                    count += 1;
                }
                continue;
            }
        }
        break;
    }
    count
}

fn with_iterator(a: Range<usize>, b: Range<usize>) -> usize {
    //a single line goes here
    a.zip(b).filter(func).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = 0..0;
        let b = 0..0;
        assert_eq!(imperative(a.clone(), b.clone()), with_iterator(a, b));
    }
    #[test]
    fn test2() {
        let a = 0..1;
        let b = 2..3;
        assert_eq!(imperative(a.clone(), b.clone()), with_iterator(a, b));
    }
    #[test]
    fn test3() {
        let a = 0..4;
        let b = 2..6;
        assert_eq!(imperative(a.clone(), b.clone()), with_iterator(a, b));
    }
    #[test]
    fn test4() {
        let a = 0..4;
        let b = 2..20;
        assert_eq!(imperative(a.clone(), b.clone()), with_iterator(a, b));
    }
    #[test]
    fn test5() {
        let a = 0..20;
        let b = 2..6;
        assert_eq!(imperative(a.clone(), b.clone()), with_iterator(a, b));
    }
}
