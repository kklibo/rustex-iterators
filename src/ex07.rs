//! Make the tests pass: use `std::iter::Iterator` methods.
#![allow(dead_code, clippy::is_digit_ascii_radix)]

/// Read through `strs` in order and return the first
/// character that is a valid hex digit.
fn first_hex(strs: &[&str]) -> Option<char> {
    for s in strs {
        for c in s.chars() {
            if c.is_digit(16) {
                return Some(c);
            }
        }
    }
    None
}

fn with_iterator(strs: &[&str]) -> Option<char> {
    //a single line goes here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let strs = &[];
        assert_eq!(first_hex(strs), with_iterator(strs));
    }
    #[test]
    fn test2() {
        let strs = &[""];
        assert_eq!(first_hex(strs), with_iterator(strs));
    }
    #[test]
    fn test3() {
        let strs = &["s"];
        assert_eq!(first_hex(strs), with_iterator(strs));
    }
    #[test]
    fn test4() {
        let strs = &["a"];
        assert_eq!(first_hex(strs), with_iterator(strs));
    }
    #[test]
    fn test5() {
        let strs = &["sssA"];
        assert_eq!(first_hex(strs), with_iterator(strs));
    }
    #[test]
    fn test6() {
        let strs = &["sss", "ssss"];
        assert_eq!(first_hex(strs), with_iterator(strs));
    }
    #[test]
    fn test7() {
        let strs = &["ssss", "sss0sss"];
        assert_eq!(first_hex(strs), with_iterator(strs));
    }
}
