#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    let (bounds, symbol_map) = parse_map(input);

    symbol_map
        .values()
        .map(|coords| find_all_simple_antinodes_in_bounds_for_one_symbol(bounds, coords))
        .fold(HashSet::new(), |acc, set| {
            acc.union(&set).copied().collect::<HashSet<Coord>>()
        })
        .len()
}

pub fn process_part2(input: &str) -> usize {
    let (bounds, symbol_map) = parse_map(input);

    symbol_map
        .values()
        .map(|coords| find_all_complex_antinodes_in_bounds_for_one_symbol(bounds, coords))
        .fold(HashSet::new(), |acc, set| {
            acc.union(&set).copied().collect::<HashSet<Coord>>()
        })
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 14);
        assert_eq!(process_part1(&get_input()), 313);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 34);
        assert_eq!(process_part2(&get_input()), 1064);
    }
}
