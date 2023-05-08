//! Make the tests pass: use `std::iter::Iterator` methods.
#![allow(dead_code)]

fn is_start(s: &str) -> bool {
    s == "start"
}
fn is_end(s: &str) -> bool {
    s == "end"
}
fn try_add(sum: u32, s: &&str) -> Result<u32, ()> {
    if *s == "+1" {
        Ok(sum + 1)
    } else {
        Err(())
    }
}

/// Parse a series of strings and return a sum value.
/// 1. Ignore all entries until a start string is found
/// 2. After this, until an end string is found:
///     1. count the "+1" strings
///     2. Any string other than "+1" is invalid: return an error
/// 3. Ignore all entries after the end string
///
/// Examples:
/// ```text
/// assert_eq!(parse_series(&["+1", "start", "+1", "end", "+1"]), Ok(1));
/// assert_eq!(parse_series(&["start", "foo", "end"]), Err(()));
/// ```
fn parse_series(strs: &[&str]) -> Result<u32, ()> {
    let mut started = false;
    let mut ended = false;
    let mut sum = 0;

    for s in strs {
        if !started && is_start(s) {
            started = true;
            continue;
        }
        if !ended && is_end(s) {
            ended = true;
            continue;
        }
        if started && !ended {
            sum = try_add(sum, s)?;
        }
    }
    Ok(sum)
}

fn with_iterator(strs: &[&str]) -> Result<u32, ()> {
    //a single statement goes here
    strs.iter()
        .skip_while(|s| !is_start(s))
        .skip(1)
        .take_while(|s| !is_end(s))
        .try_fold(0, try_add)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tester(f: impl Fn(&[&str]) -> Result<u32, ()>) {
        let strs = &[];
        assert_eq!(f(strs), Ok(0));
        let strs = &["+1"];
        assert_eq!(f(strs), Ok(0));
        let strs = &["start"];
        assert_eq!(f(strs), Ok(0));
        let strs = &["+1", "start"];
        assert_eq!(f(strs), Ok(0));
        let strs = &["end"];
        assert_eq!(f(strs), Ok(0));
        let strs = &["end", "+1"];
        assert_eq!(f(strs), Ok(0));
        let strs = &["+1", "end"];
        assert_eq!(f(strs), Ok(0));
        let strs = &["start", "end"];
        assert_eq!(f(strs), Ok(0));
        let strs = &["+1", "start", "end", "+1"];
        assert_eq!(f(strs), Ok(0));

        let strs = &["start", "+1", "end"];
        assert_eq!(f(strs), Ok(1));
        let strs = &["+1", "start", "+1", "+1", "end", "+1"];
        assert_eq!(f(strs), Ok(2));
        let strs = &["+1", "start", "+1", "+1", "+1"];
        assert_eq!(f(strs), Ok(3));

        let strs = &["invalid", "start", "+1", "end", "invalid"];
        assert_eq!(f(strs), Ok(1));
        let strs = &["invalid", "+1", "end", "invalid"];
        assert_eq!(f(strs), Ok(0));
        let strs = &["start", "invalid", "+1", "end"];
        assert_eq!(f(strs), Err(()));

        let strs = &["start", "+1", "end", "+1", "end"];
        assert_eq!(f(strs), Ok(1));
        let strs = &["start", "+1", "start", "+1", "end"];
        assert_eq!(f(strs), Err(()));
    }

    #[test]
    fn test1() {
        tester(parse_series);
        tester(with_iterator);
    }
}
