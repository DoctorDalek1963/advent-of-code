#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> u16 {
    parse_games(input)
        .unwrap()
        .1
        .into_iter()
        .filter(|game| {
            game.is_possible(CubeSet {
                red: 12,
                green: 13,
                blue: 14,
            })
        })
        .map(|game| game.id)
        .sum()
}

pub fn process_part2(input: &str) -> u32 {
    parse_games(input)
        .unwrap()
        .1
        .into_iter()
        .map(|game| game.get_minimum_set().power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 8);
        assert_eq!(process_part1(&get_input()), 2541);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 2286);
        assert_eq!(process_part2(&get_input()), 66016);
    }
}
