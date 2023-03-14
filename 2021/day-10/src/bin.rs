#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> u32 {
    input.lines().map(find_corruption_score).sum()
}

pub fn process_part2(input: &str) -> u64 {
    let mut scores: Vec<_> = input
        .lines()
        .filter(|line| find_corruption_score(line) == 0)
        .map(find_autocomplete_score)
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 26397);
        assert_eq!(process_part1(&get_input()), 469755);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 288957);
        assert_eq!(process_part2(&get_input()), 2762335572);
    }
}
