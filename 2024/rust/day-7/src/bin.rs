#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> i64 {
    parse_equations(input)
        .into_iter()
        .filter_map(|(target, nums)| {
            if ways_to_solve_simple(target, &nums) > 0 {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

pub fn process_part2(input: &str) -> i64 {
    parse_equations(input)
        .into_iter()
        .filter_map(|(target, nums)| {
            if ways_to_solve_with_concat(target, &nums) > 0 {
                Some(target)
            } else {
                None
            }
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
        assert_eq!(process_part1(TEST_INPUT), 3749);
        assert_eq!(process_part1(&get_input()), 21_572_148_763_543);
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "part 2 is slow in debug builds")]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 11_387);
        assert_eq!(process_part2(&get_input()), 581_941_094_529_163);
    }
}
