#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> u32 {
    let mut octupuses = Octopodes::parse(input);
    for _ in 0..100 {
        octupuses.run_step();
    }
    octupuses.total_flashes()
}

pub fn process_part2(input: &str) -> u32 {
    let mut octupuses = Octopodes::parse(input);

    let mut step_num = 0;
    loop {
        step_num += 1;
        if octupuses.run_step() {
            break;
        }
    }
    step_num
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 1656);
        assert_eq!(process_part1(&get_input()), 1665);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 195);
        assert_eq!(process_part2(&get_input()), 235);
    }
}
