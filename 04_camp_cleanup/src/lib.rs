pub mod parse;

/// Given two bounds-inclusive ranges, determine whether one completely
/// contains the other.
pub fn has_total_overlap(a1: (i32, i32), a2: (i32, i32)) -> bool {
    range_contains(a1, a2) || range_contains(a2, a1)
}

fn range_contains(sup: (i32, i32), sub: (i32, i32)) -> bool {
    sup.0 <= sub.0 && sup.1 >= sub.1
}

/// Given two bounds-inclusive ranges, determine whether they have any overlap.
pub fn has_overlap(a1: (i32, i32), a2: (i32, i32)) -> bool {
    a1.0 <= a2.1 && a2.0 <= a1.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_contains_true() {
        let sup = (1, 20);
        let sub = (4, 20);
        assert!(range_contains(sup, sub));
    }

    #[test]
    fn range_contains_false() {
        let sup = (1, 20);
        let sub = (8, 29);
        assert!(!range_contains(sup, sub));
    }

    #[test]
    fn has_overlap_total() {
        let (a1, a2) = ((5, 7), (3, 9));
        assert!(has_overlap(a1, a2));
    }

    #[test]
    fn has_overlap_partial() {
        let (a1, a2) = ((3, 8), (1, 3));
        assert!(has_overlap(a1, a2));
    }

    #[test]
    fn has_overlap_false() {
        let (a1, a2) = ((17, 18), (19, 20));
        assert!(!has_overlap(a1, a2));
    }
}


