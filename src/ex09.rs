//! Make the tests pass:
//! For each provided function, match its behavior with a more concise function.
//!
//! Can you do all of them before you try running the tests?
#![allow(dead_code, clippy::option_filter_map, clippy::map_flatten)]

fn func00(s: &str) -> bool {
    s.chars().fold(true, |b, c| c.is_numeric() && b)
}
fn func00_concise(s: &str) -> bool {
    // Replace `fold` with a more concise `std::iter::Iterator` method
    s.chars().all(char::is_numeric)
}

fn func01(s: &str) -> bool {
    s.chars().fold(false, |b, c| c.is_numeric() || b)
}
fn func01_concise(s: &str) -> bool {
    // Replace `fold` with a more concise `std::iter::Iterator` method
    s.chars().any(char::is_numeric)
}

/// Returns an *`Iterator`* that skips the first element.
fn skip_first(values: Vec<u8>) -> impl Iterator<Item = u8> {
    values.into_iter().skip(1)
}

fn func02(values: Vec<Vec<u8>>) -> Vec<u8> {
    values.into_iter().map(skip_first).flatten().collect()
}
fn func02_concise(values: Vec<Vec<u8>>) -> Vec<u8> {
    // Replace two `Iterator` methods with one
    values.into_iter().flat_map(skip_first).collect()
}

/// If `x` is a valid hex digit (less than 16),
/// return its character (e.g.: 10 -> 'a').
fn hexify(x: &u8) -> Option<char> {
    char::from_digit(*x as u32, 16)
}

fn func03(values: &[u8]) -> String {
    values
        .iter()
        .map(hexify)
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
}
fn func03_concise(values: &[u8]) -> String {
    // Replace three `Iterator` methods with one
    values.iter().filter_map(hexify).collect()
}

fn func04(values: &[u8]) -> Option<char> {
    values
        .iter()
        .map(hexify)
        .filter(Option::is_some)
        .map(Option::unwrap)
        .next()
}
fn func04_concise(values: &[u8]) -> Option<char> {
    // Replace four `Iterator` methods with one
    values.iter().find_map(hexify)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test00() {
        let s = "";
        assert_eq!(func00(s), func00_concise(s));
        let s = "a";
        assert_eq!(func00(s), func00_concise(s));
        let s = "0";
        assert_eq!(func00(s), func00_concise(s));
        let s = "aaaa1a";
        assert_eq!(func00(s), func00_concise(s));
        let s = "111111";
        assert_eq!(func00(s), func00_concise(s));
    }
    #[test]
    fn test01() {
        let s = "";
        assert_eq!(func01(s), func01_concise(s));
        let s = "a";
        assert_eq!(func01(s), func01_concise(s));
        let s = "0";
        assert_eq!(func01(s), func01_concise(s));
        let s = "aaaa1a";
        assert_eq!(func01(s), func01_concise(s));
        let s = "aaaaaa";
        assert_eq!(func01(s), func01_concise(s));
    }
    #[test]
    fn test02() {
        let values = vec![];
        assert_eq!(func02(values.clone()), func02_concise(values));
        let values = vec![vec![]];
        assert_eq!(func02(values.clone()), func02_concise(values));
        let values = vec![vec![0]];
        assert_eq!(func02(values.clone()), func02_concise(values));
        let values = vec![vec![0], vec![1]];
        assert_eq!(func02(values.clone()), func02_concise(values));
        let values = vec![vec![0, 1], vec![], vec![2, 3, 4]];
        assert_eq!(func02(values.clone()), func02_concise(values));
    }
    #[test]
    fn test03() {
        let values = &[];
        assert_eq!(func03(values), func03_concise(values));
        let values = &[0];
        assert_eq!(func03(values), func03_concise(values));
        let values = &[20];
        assert_eq!(func03(values), func03_concise(values));
        let values = &[0, 1, 2, 3];
        assert_eq!(func03(values), func03_concise(values));
        let values = &[14, 15, 16, 17];
        assert_eq!(func03(values), func03_concise(values));
        let values = &[18, 19, 20, 21];
        assert_eq!(func03(values), func03_concise(values));
    }
    #[test]
    fn test04() {
        let values = &[];
        assert_eq!(func04(values), func04_concise(values));
        let values = &[0];
        assert_eq!(func04(values), func04_concise(values));
        let values = &[20];
        assert_eq!(func04(values), func04_concise(values));
        let values = &[14, 15, 16, 17];
        assert_eq!(func04(values), func04_concise(values));
        let values = &[16, 17, 14, 15];
        assert_eq!(func04(values), func04_concise(values));
        let values = &[18, 19, 20, 21];
        assert_eq!(func04(values), func04_concise(values));
    }
}
