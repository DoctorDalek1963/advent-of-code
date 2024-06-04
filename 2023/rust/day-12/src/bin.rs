#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    parse_spring_states_and_group_lengths(input)
        .unwrap()
        .1
        .into_par_iter()
        .map(|(spring_states, group_lengths): (Vec<_>, Vec<_>)| {
            count_possible_arrangements(&spring_states, &group_lengths)
        })
        .sum()
}

pub fn process_part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    #[ignore]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 21);
        //assert_eq!(process_part1(&get_input()), 1);
    }

    #[test]
    #[ignore]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 1);
        //assert_eq!(process_part2(&get_input()), 1);
    }
}
