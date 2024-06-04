#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    simulate_lanternfish_with_vec(parse_lanternfish(input).unwrap().1, 80)
}

pub fn process_part2(input: &str) -> usize {
    simulate_lanternfish_with_map(parse_lanternfish(input).unwrap().1, 256)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 5934);
        //assert_eq!(process_part1(&get_input()), 1);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 26984457539);
        //assert_eq!(process_part2(&get_input()), 1);
    }
}
