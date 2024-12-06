#![allow(unused_imports)]
#![allow(unused_variables)]

use rayon::iter::{ParallelBridge, ParallelIterator};
use std::collections::HashSet;

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    let (bounds, obstacles, guard) = parse_map(input);
    get_all_visited_coords_assuming_no_loop(&bounds, &obstacles, guard).len()
}

pub fn process_part2(input: &str) -> usize {
    let (bounds, obstacles, guard) = parse_map(input);
    let all_visited_coords = get_all_visited_coords_assuming_no_loop(&bounds, &obstacles, guard);

    (0..bounds.0)
        .flat_map(|x| std::iter::repeat(x).zip(0..bounds.1))
        .par_bridge()
        .filter(|&(x, y)| {
            // Placing a new obstacle in a position that the guard never encounters would do nothing,
            // so we only consider positions that the guard would visit in the original map. And
            // we're not allowed to put an obstacle in the guard's starting position.
            all_visited_coords.contains(&Coord(x, y)) && guard.position != Coord(x, y)
        })
        .filter(|&(x, y)| {
            let mut new_obstacles = Vec::with_capacity(obstacles.len() + 1);
            new_obstacles.extend(obstacles.iter());
            new_obstacles.push(Coord(x, y));

            map_has_loop(&bounds, &new_obstacles, guard)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 41);
        assert_eq!(process_part1(&get_input()), 4602);
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "part 2 is slow in debug builds")]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 6);
        assert_eq!(process_part2(&get_input()), 1703);
    }
}
