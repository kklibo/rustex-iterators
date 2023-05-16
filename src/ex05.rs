//! Make the tests pass: use `std::iter::Iterator` methods.
#![allow(dead_code)]

/// Hash a u8
fn hash(x: &u8) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    x.hash(&mut hasher);
    hasher.finish()
}

/// Bitwise-XOR two hash values
fn xor(digest: u64, new_hash: u64) -> u64 {
    digest ^ new_hash
}

/// Create a digest (a fixed-length representation of a dataset)
/// of values that ignores reordering:
/// Hash each value, then XOR the hashes together.
/// Different groups of values will have different digests,
/// but the same groups of values will have the same digests
/// regardless of value order.
fn unordered_digest(values: &[u8]) -> Option<u64> {
    if values.is_empty() {
        return None;
    }

    let mut digest = 0;
    for x in values {
        digest = xor(digest, hash(x));
    }

    Some(digest)
}

fn with_iterator(values: &[u8]) -> Option<u64> {
    //a single line goes here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reorder_equivalence() {
        let values1 = &[1, 2, 3, 4];
        let values2 = &[2, 1, 4, 3];
        assert_eq!(unordered_digest(values1), unordered_digest(values2));
        assert_eq!(with_iterator(values1), with_iterator(values2));
    }

    #[test]
    fn test1() {
        let values = &[];
        assert_eq!(unordered_digest(values), with_iterator(values));
    }
    #[test]
    fn test2() {
        let values = &[0];
        assert_eq!(unordered_digest(values), with_iterator(values));
    }
    #[test]
    fn test3() {
        let values = &[1, 2, 3, 4];
        assert_eq!(unordered_digest(values), with_iterator(values));
    }
}
