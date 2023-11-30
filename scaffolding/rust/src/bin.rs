#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    0
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
        assert_eq!(process_part1(TEST_INPUT), 1);
        //assert_eq!(process_part1(&get_input()), 1);
    }

    #[test]
    #[ignore]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 1);
        //assert_eq!(process_part2(&get_input()), 1);
    }
}
