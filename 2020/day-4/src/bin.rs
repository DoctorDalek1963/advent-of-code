#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    Passport::from_text(input)
        .into_iter()
        .filter(|passport| passport.is_basic_valid_no_country_id())
        .count()
}

pub fn process_part2(input: &str) -> usize {
    Passport::from_text(input)
        .into_iter()
        .filter(|passport| passport.is_proper_valid_no_country_id())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 2);
        assert_eq!(process_part1(&get_input()), 204);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 2);
        assert_eq!(process_part2(&get_input()) - 1, 179);
    }
}
