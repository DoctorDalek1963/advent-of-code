#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> u32 {
    parse_all_password_policy_pairs(input)
        .unwrap()
        .1
        .into_iter()
        .filter(|(policy, password)| is_password_valid_part_1(*policy, password))
        .count() as u32
}

pub fn process_part2(input: &str) -> u32 {
    parse_all_password_policy_pairs(input)
        .unwrap()
        .1
        .into_iter()
        .filter(|(policy, password)| is_password_valid_part_2(*policy, password))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 2);
        assert_eq!(process_part1(&get_input()), 422);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 1);
        assert_eq!(process_part2(&get_input()), 451);
    }
}
