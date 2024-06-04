#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> u32 {
    let (a, b) = find_pair(&parse_nums(input));
    a * b
}

pub fn process_part2(input: &str) -> u32 {
    let (a, b, c) = find_triplet(&parse_nums(input));
    a * b * c
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 514579);
        assert_eq!(process_part1(&get_input()), 928896);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 241861950);
        assert_eq!(process_part2(&get_input()), 295668576);
    }
}
