#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    count_collisions(&parse_map(input).unwrap().1, (3, 1))
}

pub fn process_part2(input: &str) -> usize {
    let map = parse_map(input).unwrap().1;
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|slope| count_collisions(&map, slope))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 7);
        assert_eq!(process_part1(&get_input()), 274);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 336);
        assert_eq!(process_part2(&get_input()), 6_050_183_040);
    }
}
