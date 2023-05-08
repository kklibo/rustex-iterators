//! Make the tests pass: use `std::iter::Iterator` methods.
#![allow(dead_code)]

fn imperative(numbers: &[u8], count: usize) -> u8 {
    let mut i = 0;
    let mut result = 0;
    for _ in 0..count {
        if let Some(&number) = numbers.get(i) {
            result += number;
        }
        i += 1;
        if i == numbers.len() {
            i = 0;
        }
    }
    result
}

fn with_iterator(numbers: &[u8], count: usize) -> u8 {
    //a single line goes here
    numbers.iter().cycle().take(count).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let numbers = &[];
        let count = 10;
        assert_eq!(imperative(numbers, count), with_iterator(numbers, count));
    }

    #[test]
    fn test2() {
        let numbers = &[1];
        let count = 10;
        assert_eq!(imperative(numbers, count), with_iterator(numbers, count));
    }

    #[test]
    fn test3() {
        let numbers = &[0, 1, 2, 3];
        let count = 10;
        assert_eq!(imperative(numbers, count), with_iterator(numbers, count));
    }

    #[test]
    fn test4() {
        let numbers = &[0, 1, 2, 3];
        let count = 2;
        assert_eq!(imperative(numbers, count), with_iterator(numbers, count));
    }
}
