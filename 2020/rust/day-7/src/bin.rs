#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    how_many_can_contain(&parse_rules(input).unwrap().1, "shiny gold")
}

pub fn process_part2(input: &str) -> usize {
    how_many_inside(&parse_rules(input).unwrap().1, "shiny gold")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 4);
        //assert_eq!(process_part1(&get_input()), 355);
    }

    #[test]
    fn process_part2_test() {
        const OTHER_INPUT: &str = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"#;

        assert_eq!(process_part2(TEST_INPUT), 32);
        assert_eq!(process_part2(OTHER_INPUT), 126);
        assert_eq!(process_part2(&get_input()), 5312);
    }
}
