//! Make the tests pass: use `std::iter::Iterator` methods.
#![allow(dead_code)]

/// True IFF `with_prefix` is exactly the same as `base` preceded by `prefix`.
fn is_prefixed(prefix: &[u8], base: &[u8], with_prefix: &[u8]) -> bool {
    if prefix.len() + base.len() != with_prefix.len() {
        return false;
    }

    for i in 0..prefix.len() {
        if prefix[i] != with_prefix[i] {
            return false;
        }
    }
    for i in 0..base.len() {
        if base[i] != with_prefix[prefix.len() + i] {
            return false;
        }
    }

    true
}

fn with_iterator(prefix: &[u8], base: &[u8], with_prefix: &[u8]) -> bool {
    //a single line goes here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let prefix = [];
        let base = [];
        let with_prefix = [];
        assert_eq!(
            is_prefixed(&prefix, &base, &with_prefix),
            with_iterator(&prefix, &base, &with_prefix)
        );
    }
    #[test]
    fn test2() {
        let prefix = [];
        let base = [1];
        let with_prefix = [1];
        assert_eq!(
            is_prefixed(&prefix, &base, &with_prefix),
            with_iterator(&prefix, &base, &with_prefix)
        );
    }
    #[test]
    fn test3() {
        let prefix = [1];
        let base = [];
        let with_prefix = [1];
        assert_eq!(
            is_prefixed(&prefix, &base, &with_prefix),
            with_iterator(&prefix, &base, &with_prefix)
        );
    }
    #[test]
    fn test4() {
        let prefix = [1, 2];
        let base = [3, 4];
        let with_prefix = [1, 2, 3, 4];
        assert_eq!(
            is_prefixed(&prefix, &base, &with_prefix),
            with_iterator(&prefix, &base, &with_prefix)
        );
    }
    #[test]
    fn test5() {
        let prefix = [1, 2];
        let base = [3, 4];
        let with_prefix = [1, 2, 3, 5];
        assert_eq!(
            is_prefixed(&prefix, &base, &with_prefix),
            with_iterator(&prefix, &base, &with_prefix)
        );
    }
    #[test]
    fn test6() {
        let prefix = [1, 2];
        let base = [3, 4];
        let with_prefix = [1, 2, 3];
        assert_eq!(
            is_prefixed(&prefix, &base, &with_prefix),
            with_iterator(&prefix, &base, &with_prefix)
        );
    }
    #[test]
    fn test7() {
        let prefix = [1, 2];
        let base = [3, 4];
        let with_prefix = [1, 2, 3, 4, 5];
        assert_eq!(
            is_prefixed(&prefix, &base, &with_prefix),
            with_iterator(&prefix, &base, &with_prefix)
        );
    }
}
