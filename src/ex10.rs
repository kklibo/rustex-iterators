//! Make the tests pass: use `std::iter::Iterator` methods.
#![allow(dead_code)]

fn is_even(x: &u32) -> bool {
    x % 2 == 0
}

const SUM_LIMIT: u32 = 100;
/// Increment the sum and return `x` IFF it won't exceed the limit.
fn check_limit(sum: &mut u32, x: &u32) -> Option<u32> {
    if *sum + x <= SUM_LIMIT {
        *sum += x;
        Some(*x)
    } else {
        None
    }
}

/// Sort these values into even and odd groups, but stop before
/// their sum would exceed the limit.
fn limited_sort(values: &[u32]) -> (Vec<u32>, Vec<u32>) {
    let mut even = Vec::new();
    let mut odd = Vec::new();

    let mut sum = 0;

    for v in values {
        if let Some(v) = check_limit(&mut sum, v) {
            if is_even(&v) { &mut even } else { &mut odd }.push(v);
        } else {
            break;
        }
    }

    (even, odd)
}

fn with_iterator(values: &[u32]) -> (Vec<u32>, Vec<u32>) {
    //a single line goes here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let values = &[];
        assert_eq!(limited_sort(values), with_iterator(values));
        let values = &[100];
        assert_eq!(limited_sort(values), with_iterator(values));
        let values = &[101];
        assert_eq!(limited_sort(values), with_iterator(values));
        let values = &[1, 2, 3, 4];
        assert_eq!(limited_sort(values), with_iterator(values));
        let values = &[1, 2, 3, 4, 100, 5, 6, 7];
        assert_eq!(limited_sort(values), with_iterator(values));
    }
}
