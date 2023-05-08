//! Make the tests pass: use `std::iter::Iterator` methods.
#![allow(dead_code)]

fn imperative(numbers: &[u8]) -> Vec<bool> {
    let mut vec = Vec::new();
    for i in numbers {
        vec.push(i.is_power_of_two());
    }
    vec
}

fn with_iterator(numbers: &[u8]) -> Vec<bool> {
    //a single line goes here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let numbers = &[];
        assert_eq!(imperative(numbers), with_iterator(numbers));
        let numbers = &[1];
        assert_eq!(imperative(numbers), with_iterator(numbers));
        let numbers = &[0, 1, 2, 3, 4, 5];
        assert_eq!(imperative(numbers), with_iterator(numbers));
    }
}
