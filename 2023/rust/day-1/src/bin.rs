#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> u32 {
    input.lines().map(get_basic_calibration_value).sum()
}

pub fn process_part2(input: &str) -> u32 {
    input.lines().map(get_complex_calibration_value).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TEST_INPUT_1, TEST_INPUT_2};
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT_1), 142);
        assert_eq!(process_part1(&get_input()), 53974);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT_2), 281);
        assert_eq!(process_part2(&get_input()), 52840);
    }
}
