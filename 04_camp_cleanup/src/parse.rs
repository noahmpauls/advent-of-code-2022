use lazy_static::lazy_static;
use regex::Regex;

type Assignment = (i32, i32);
type Pair = (Assignment, Assignment);

/// Parse group assignments from a string.
pub fn group_assignment_pairs(s: &str) -> Pair {
    // Using a regex is not the best way. Instead, just parse by splitting on
    // commas and dashes.
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    let mut nums = RE.find_iter(s).map(|m| m.as_str().parse::<i32>().unwrap());
    ((
        nums.next().unwrap(),
        nums.next().unwrap(),
    ),(
        nums.next().unwrap(),
        nums.next().unwrap(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_groups_onedigit() {
        let s = "1-2,3-4";
        let expected = ((1, 2), (3, 4));
        assert_eq!(expected, group_assignment_pairs(s));
    }

    #[test]
    fn parse_groups_manydigits() {
        let s = "1-25,23-4500";
        let expected = ((1, 25), (23, 4500));
        assert_eq!(expected, group_assignment_pairs(s));
    }
}

