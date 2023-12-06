#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    parse_races(input)
        .unwrap()
        .1
        .into_iter()
        .map(|race| race.ways_to_win().count())
        .product()
}

pub fn process_part2(input: &str) -> usize {
    let races = parse_races(&input.replace(" ", "")).unwrap().1;

    debug_assert_eq!(
        races.len(),
        1,
        "There should only be one race after removing all spaces"
    );

    let race = races[0];
    race.ways_to_win().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 288);
        assert_eq!(process_part1(&get_input()), 2_756_160);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 71503);
        assert_eq!(process_part2(&get_input()), 34_788_142);
    }
}
