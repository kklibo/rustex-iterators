//! Make the tests pass: use a `std::iter::Iterator` method.
//! Note: `std::ops::Range` implements `Iterator`.
#![allow(dead_code)]

use std::ops::Range;

fn imperative(range: Range<u8>) -> Vec<u8> {
    let mut vec = Vec::new();
    for i in range {
        vec.push(i);
    }
    vec
}

/// Complete this function and run `cargo test` to confirm that
/// the tests pass.
///
/// When you're done, uncomment the next exercise in `lib.rs` to
/// advance to the next level.
fn with_iterator(range: Range<u8>) -> Vec<u8> {
    //a single line goes here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise01() {
        assert_eq!(imperative(0..0), with_iterator(0..0));
        assert_eq!(imperative(0..1), with_iterator(0..1));
        assert_eq!(imperative(0..10), with_iterator(0..10));
    }
}
