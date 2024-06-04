#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> i64 {
    parse(input)
        .unwrap()
        .1
        .into_iter()
        .map(|seq| get_next_number(&seq))
        .sum()
}

pub fn process_part2(input: &str) -> i64 {
    parse(input)
        .unwrap()
        .1
        .into_iter()
        .map(|seq| get_previous_number(&seq))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 114);
        assert_eq!(process_part1(&get_input()), 1_647_269_739);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 2);
        assert_eq!(process_part2(&get_input()), 864);
    }
}
