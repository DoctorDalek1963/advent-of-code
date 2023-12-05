#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> u64 {
    parse_almanac(input)
        .unwrap()
        .1
        .get_locations()
        .min()
        .unwrap()
}

pub fn process_part2(input: &str) -> u64 {
    parse_almanac(input)
        .unwrap()
        .1
        .reinterpret_seed_numbers_and_get_locations()
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 35);
        assert_eq!(process_part1(&get_input()), 226_172_555);
    }

    #[test]
    #[cfg_attr(
        debug_assertions,
        ignore = "this test takes ages even in release builds"
    )]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 46);
        assert_eq!(process_part2(&get_input()), 47_909_639);
    }
}
