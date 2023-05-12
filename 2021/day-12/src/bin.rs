#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    Graph::parse(input)
        .all_dfs_paths_visiting_small_caves_once()
        .len()
}

pub fn process_part2(input: &str) -> usize {
    Graph::parse(input)
        .all_dfs_paths_visiting_small_caves_twice()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TEST_INPUT_1, TEST_INPUT_2, TEST_INPUT_3};

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT_1), 10);
        assert_eq!(process_part1(TEST_INPUT_2), 19);
        assert_eq!(process_part1(TEST_INPUT_3), 226);
        assert_eq!(process_part1(&get_input()), 3679);
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT_1), 36);
        assert_eq!(process_part2(TEST_INPUT_2), 103);
        assert_eq!(process_part2(TEST_INPUT_3), 3509);
        //assert_eq!(process_part2(&get_input()), 1);
    }
}
