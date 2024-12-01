#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;
use itertools::Itertools;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    let (mut left_list, mut right_list) = get_two_lists(input);

    left_list.sort();
    right_list.sort();

    left_list
        .into_iter()
        .zip(right_list)
        .map(|(a, b)| a.abs_diff(b) as usize)
        .sum()
}

pub fn process_part2(input: &str) -> usize {
    let (left_list, right_list) = get_two_lists(input);

    let right_list_counts = right_list.into_iter().counts();

    left_list
        .into_iter()
        .map(|x| x as usize * right_list_counts.get(&x).copied().unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 11);
        assert_eq!(process_part1(&get_input()), 2_580_760);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 31);
        assert_eq!(process_part2(&get_input()), 25_358_365);
    }
}
