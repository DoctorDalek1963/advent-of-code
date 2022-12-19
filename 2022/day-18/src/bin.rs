#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    find_total_surface_area(parse_points(input).unwrap().1)
}

pub fn process_part2(input: &str) -> usize {
    find_exterior_surface_area(parse_points(input).unwrap().1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 64);
        assert_eq!(process_part1(&get_input()), 3586);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 58);
        assert_eq!(process_part2(&get_input()), 2072);
    }
}
