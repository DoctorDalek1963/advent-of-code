#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

fn process_general(input: &str, is_smudged: bool) -> usize {
    let scans = parse_scans(input);
    scans
        .iter()
        .map(|scan| -> usize {
            let columns: Vec<_> = find_lines_of_symmetry(scan, true, is_smudged).collect();
            let rows: Vec<_> = find_lines_of_symmetry(scan, false, is_smudged).collect();

            debug_assert!(
                (columns.len() == 1 && rows.len() == 0)
                    || (columns.len() == 0 && rows.len() == 1)
                    || (columns.len() == 0 && rows.len() == 0)
            );

            columns.into_iter().sum::<usize>() + 100 * rows.into_iter().sum::<usize>()
        })
        .sum()
}

pub fn process_part1(input: &str) -> usize {
    process_general(input, false)
}

pub fn process_part2(input: &str) -> usize {
    process_general(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 405);
        assert_eq!(process_part1(&get_input()), 29213);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 400);
        assert_eq!(process_part2(&get_input()), 37453);
    }
}
