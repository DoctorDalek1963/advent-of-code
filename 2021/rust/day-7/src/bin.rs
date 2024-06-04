#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> u32 {
    brute_force_optimal_fuel_cost(parse_crab_positions(input).unwrap().1)
}

pub fn process_part2(input: &str) -> u32 {
    brute_force_optimal_fuel_cost_part_2(parse_crab_positions(input).unwrap().1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 37);
        assert_eq!(process_part1(&get_input()), 354129);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 168);
        assert_eq!(process_part2(&get_input()), 98905973);
    }
}
