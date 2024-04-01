#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;
use std::collections::HashSet;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    let coords_map: HashMap<Point, (Connection, bool)> = get_coords_map(parse_map(input))
        .into_iter()
        .map(|(point, conn)| (point, (conn, false)))
        .collect();
    dbg!(coords_map);

    let mut loops = vec![];
    let mut current_loop = vec![];
    let mut prev_point = None;
    let mut current_point = (1, 1);

    while let Some(starting_point) = coords_map
        .iter()
        .find_map(|(&point, (_conn, visited))| (!visited).then_some(point))
    {
        current_loop.clear();
        current_point = starting_point;
        current_loop.push(current_point);
    }

    todo!()
}

pub fn process_part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TEST_INPUT_LARGE_CLUTTERED, TEST_INPUT_SMALL_CLUTTERED};
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT_SMALL_CLUTTERED), 4);
        assert_eq!(process_part1(TEST_INPUT_LARGE_CLUTTERED), 8);
        //assert_eq!(process_part1(&get_input()), 1);
    }

    #[test]
    #[ignore]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT_SMALL_CLUTTERED), 1);
        //assert_eq!(process_part2(&get_input()), 1);
    }
}
