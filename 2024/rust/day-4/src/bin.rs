#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    find_all_xmases_in_grid(&parse_grid(input)).len()
}

pub fn process_part2(input: &str) -> usize {
    find_all_x_mases_in_grid(&parse_grid(input)).len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 18);
        assert_eq!(process_part1(&get_input()), 2517);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 9);
        assert_eq!(process_part2(&get_input()), 1960);
    }
}
