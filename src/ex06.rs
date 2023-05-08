//! Make the tests pass: use `std::iter::Iterator` methods.
#![allow(dead_code)]

/// Go through these values in reverse, skipping `skip` values after each one selected.
/// Return the reverse-order indices of the selected values and the values themselves.
fn reverse_and_skip_through(values: &[u8], skip: usize) -> (Vec<usize>, Vec<u8>) {
    let mut selected_indices = Vec::new();
    let mut selected_values = Vec::new();

    if !values.is_empty() {
        let values_end = values.len() - 1;
        for i in 0..=values_end {
            if i % (skip + 1) == 0 {
                selected_indices.push(i);
                selected_values.push(values[values_end - i]);
            }
        }
    }

    (selected_indices, selected_values)
}

fn with_iterator(values: &[u8], skip: usize) -> (Vec<usize>, Vec<u8>) {
    //a single line goes here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let values = &[];
        let skip = 1;
        assert_eq!(
            reverse_and_skip_through(values, skip),
            with_iterator(values, skip)
        );
    }
    #[test]
    fn test2() {
        let values = &[1];
        let skip = 1;
        assert_eq!(
            reverse_and_skip_through(values, skip),
            with_iterator(values, skip)
        );
    }
    #[test]
    fn test3() {
        let values = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
        let skip = 0;
        assert_eq!(
            reverse_and_skip_through(values, skip),
            with_iterator(values, skip)
        );
    }
    #[test]
    fn test4() {
        let values = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
        let skip = 1;
        assert_eq!(
            reverse_and_skip_through(values, skip),
            with_iterator(values, skip)
        );
    }
    #[test]
    fn test5() {
        let values = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
        let skip = 2;
        assert_eq!(
            reverse_and_skip_through(values, skip),
            with_iterator(values, skip)
        );
    }
    #[test]
    fn test6() {
        let values = &[1, 2, 3, 4, 5, 6, 7, 8, 9];
        let skip = 3;
        assert_eq!(
            reverse_and_skip_through(values, skip),
            with_iterator(values, skip)
        );
    }
}
