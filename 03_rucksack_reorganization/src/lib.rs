use std::collections::HashSet;

// Give the priority of an item type.
pub fn priority(item: &char) -> u32 {
    // use ASCII codes
    const LOWER_START: u32 = 97;
    const UPPER_START: u32 = 65;
    if item.is_lowercase() {
        (*item as u32) - LOWER_START + 1
    } else {
        (*item as u32) - UPPER_START + 26 + 1
    }
}

// Find items appearing in both compartments of a rucksack
pub fn rucksack_repeats(rucksack: &str) -> HashSet<char> {
    let total_items = rucksack.chars().count();
    let compartment_contents = rucksack.chars()
        .take(total_items / 2)
        .collect::<HashSet<char>>();
    rucksack.chars()
        .rev()
        .take(total_items / 2)
        .filter(move |c| compartment_contents.contains(c))
        .collect::<HashSet<char>>()
}

/// Find the repeated characters between the given strings
pub fn shared_chars(strings: Vec<&str>) -> HashSet<char> {
    if strings.len() < 2 {
        return HashSet::new();
    }
    strings
        .iter()
        .map(|s| s.chars().collect::<HashSet<char>>())
        .reduce(|a, b| a.intersection(&b).cloned().collect::<HashSet<char>>())
        .unwrap_or(HashSet::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priorty_lower() {
        assert_eq!(7, priority(&'g'));
    }

    #[test]
    fn priorty_upper() {
        assert_eq!(48, priority(&'V'));
    }

    #[test]
    fn rucksack_repeats_one() {
        let repeats = rucksack_repeats("abcAbC");
        assert_eq!(1, repeats.len());
        assert!(repeats.contains(&'b'));
    }

    #[test]
    fn rucksack_repeats_none() {
        let repeats = rucksack_repeats("helloworldHELLOWORLD");
        assert_eq!(0, repeats.len());
    }

    #[test]
    fn rucksack_repeats_many() {
        let repeats = rucksack_repeats("aBcDeabcde");
        assert_eq!(3, repeats.len());
        for c in ['a', 'c', 'e'] {
            assert!(repeats.contains(&c));
        }
    }

    #[test]
    fn rucksack_repeats_none_regression() {
        let repeats = rucksack_repeats("aaBBBB");
        assert_eq!(1, repeats.len());
        assert!(repeats.contains(&'B'));
    }

    #[test]
    fn shared_chars_one() {
        let shared = shared_chars(vec![
            "aaa",
            "aaa",
            "aaa",
        ]);
        assert_eq!(1, shared.len());
        assert!(shared.contains(&'a'));
    }

    #[test]
    fn shared_chars_none() {
        let shared = shared_chars(vec![
            "abc",
            "ABC",
            "dEf",
        ]);
        assert_eq!(0, shared.len());
    }

    #[test]
    fn shared_chars_many() {
        let shared = shared_chars(vec![
            "abcd",
            "bcde",
            "dcoab",
        ]);
        assert_eq!(3, shared.len());
        for c in ['b', 'c', 'd'] {
            assert!(shared.contains(&c));
        }
    }

    #[test]
    fn shared_chars_single_string() {
        let shared = shared_chars(vec![
            "aaa",
        ]);
        assert_eq!(0, shared.len());
    }

}

