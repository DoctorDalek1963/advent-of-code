#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    let mut map = RockMap::from_lines(parse_lines(input).unwrap().1);
    while let Some(_) = map.drop_sand_with_abyss(true) {}
    map.sand_points.len()
}

pub fn process_part2(input: &str) -> usize {
    let mut map = RockMap::from_lines(parse_lines(input).unwrap().1);
    while let Some(_) = map.drop_sand_with_abyss(false) {}
    map.sand_points.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 24);
        assert_eq!(process_part1(&get_input()), 858);
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "very slow in debug mode")]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 93);
        assert_eq!(process_part2(&get_input()), 26845);
    }
}
