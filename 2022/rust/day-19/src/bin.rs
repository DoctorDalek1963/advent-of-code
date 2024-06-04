#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> u32 {
    parse_blueprint_list(input)
        .unwrap()
        .1
        .par_iter()
        //.par_iter()
        .map(|blueprint| blueprint.quality_level())
        .sum()
}

pub fn process_part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    #[ignore = "I never finished this day"]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 33);
        //assert_eq!(process_part1(&get_input()), 1);
    }

    #[test]
    #[ignore]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 1);
        //assert_eq!(process_part2(&get_input()), 1);
    }
}
