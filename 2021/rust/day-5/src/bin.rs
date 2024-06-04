#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;
use itertools::Itertools;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

fn process_part<F>(input: &str, func: F) -> usize
where
    F: Fn(Point, Point) -> Option<Vec<Point>>,
{
    parse_lines(input)
        .unwrap()
        .1
        .iter()
        .map(|&(p, q)| func(p, q))
        .flatten()
        .flatten()
        .counts()
        .into_iter()
        .filter(|&(point, count)| count > 1)
        .count()
}

pub fn process_part1(input: &str) -> usize {
    process_part(input, get_points_on_straight_line)
}

pub fn process_part2(input: &str) -> usize {
    process_part(input, get_points_on_straight_or_diagonal_lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 5);
        assert_eq!(process_part1(&get_input()), 7468);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 12);
        assert_eq!(process_part2(&get_input()), 22_364);
    }
}
