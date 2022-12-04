use std::{fs, ops::RangeInclusive};

pub fn get_input() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

fn parse_single_range(range: &str) -> RangeInclusive<u32> {
    let v: Vec<&str> = range.split('-').collect();
    let start: u32 = v.get(0).unwrap().parse().unwrap();
    let end: u32 = v.get(1).unwrap().parse().unwrap();

    start..=end
}

pub fn parse_ranges(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let v: Vec<&str> = line.split(',').collect();
    let s1 = *v.get(0).unwrap();
    let s2 = *v.get(1).unwrap();

    (parse_single_range(s1), parse_single_range(s2))
}

pub fn range_fully_contains_other(ranges: (RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    let (r1, r2) = ranges;
    r1.clone().into_iter().all(|e| r2.contains(&e)) || r2.into_iter().all(|e| r1.contains(&e))
}

pub fn ranges_overlap(ranges: (RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    let (r1, r2) = ranges;
    r1.clone().into_iter().any(|e| r2.contains(&e)) || r2.into_iter().any(|e| r1.contains(&e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_range_test() {
        assert_eq!(parse_single_range("2-4"), 2..=4);
        assert_eq!(parse_single_range("13-105"), 13..=105);
        assert_eq!(parse_single_range("1-3"), 1..=3);
        assert_eq!(parse_single_range("6-6"), 6..=6);
    }

    #[test]
    fn parse_ranges_test() {
        assert_eq!(parse_ranges("2-4,6-8"), (2..=4, 6..=8));
        assert_eq!(parse_ranges("2-8,3-7"), (2..=8, 3..=7));
        assert_eq!(parse_ranges("6-6,4-6"), (6..=6, 4..=6));
        assert_eq!(parse_ranges("2-6,4-8"), (2..=6, 4..=8));
    }

    #[test]
    fn range_fully_contains_other_test() {
        assert!(range_fully_contains_other((2..=8, 3..=7)));
        assert!(range_fully_contains_other((6..=6, 4..=6)));

        assert!(!range_fully_contains_other((2..=4, 6..=8)));
        assert!(!range_fully_contains_other((2..=3, 4..=5)));
        assert!(!range_fully_contains_other((2..=6, 4..=8)));
    }

    #[test]
    fn ranges_overlap_test() {
        assert!(!ranges_overlap((2..=4, 6..=8)));
        assert!(!ranges_overlap((2..=3, 4..=5)));

        assert!(ranges_overlap((5..=7, 7..=9)));
        assert!(ranges_overlap((2..=8, 3..=7)));
        assert!(ranges_overlap((6..=6, 4..=6)));
        assert!(ranges_overlap((2..=6, 4..=8)));
    }
}
