#![feature(array_windows)]

use std::cmp::Ordering;

pub mod bin;

pub fn get_reports(input: &str) -> Vec<Vec<i8>> {
    input
        .lines()
        .map(|line| line.split(" ").map(|x| x.parse().unwrap()).collect())
        .collect()
}

/// Check if a report is naively safe, not accounting for the problem dampener.
pub fn is_report_safe(report: &[i8]) -> bool {
    let mut pairs = report.array_windows::<2>();

    // If the sequence is strictly increasing, every pair will give 1 and the sum will be
    // the length of `pairs`. If the sequence is strictly decreasing, the sum will be the
    // negative length of `pairs`.
    let strictly_monotonic = pairs
        .map(|[a, b]| match a.cmp(b) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        })
        .sum::<i32>()
        .unsigned_abs() as usize
        == pairs.len();

    let reasonable_diffs = pairs.all(|[a, b]| (1..=3).contains(&a.abs_diff(*b)));

    strictly_monotonic && reasonable_diffs
}

/// Check if a report is safe by using [`is_report_safe`] and also using the problem dampener,
/// which can remove a single problematic level.
pub fn is_report_safe_with_problem_dampener(report: &[i8]) -> bool {
    if is_report_safe(report) {
        return true;
    }

    // We allocate a vec with len-1 slots and then try all possible variations of the report by
    // omitting one level at a time and early returning if we find one that works. This isn't very
    // efficient for large reports because it tries every option rather than trying to find the
    // problematic level, but all the reports are small in the problem input, so this is not an
    // issue in practice.
    let mut vec = Vec::with_capacity(report.len().saturating_sub(1));

    for omitted_idx in 0..report.len() {
        for (i, &level) in report.iter().enumerate() {
            if i == omitted_idx {
                continue;
            }

            vec.push(level);
        }

        if is_report_safe(&vec) {
            return true;
        }

        vec.clear();
    }

    false
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_reports() {
        assert_eq!(
            get_reports(TEST_INPUT),
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9],
            ]
        );
    }
}
