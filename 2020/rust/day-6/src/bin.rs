#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    parse_groups(input)
        .unwrap()
        .1
        .into_iter()
        .map(|group| collect_responses(&group).len())
        .sum()
}

pub fn process_part2(input: &str) -> usize {
    parse_groups(input)
        .unwrap()
        .1
        .into_iter()
        .map(|group| {
            let group_size = group.len();
            collect_and_count_responses(&group)
                .values()
                .filter(|&&count| count == group_size)
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 11);
        assert_eq!(process_part1(&get_input()), 6335);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 6);
        assert_eq!(process_part2(&get_input()), 3392);
    }
}
