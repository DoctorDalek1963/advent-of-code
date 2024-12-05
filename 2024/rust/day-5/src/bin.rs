#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    let (rules, page_lists) = parse_input(input);

    page_lists
        .into_iter()
        .filter_map(|list| {
            let ordered = order_page_list(&rules, list.clone());

            if list == ordered {
                Some(list[list.len() / 2] as usize)
            } else {
                None
            }
        })
        .sum()
}

pub fn process_part2(input: &str) -> usize {
    let (rules, page_lists) = parse_input(input);

    page_lists
        .into_iter()
        .filter_map(|list| {
            let ordered = order_page_list(&rules, list.clone());

            if list != ordered {
                Some(ordered[ordered.len() / 2] as usize)
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
        assert_eq!(process_part1(TEST_INPUT), 143);
        assert_eq!(process_part1(&get_input()), 7365);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 123);
        assert_eq!(process_part2(&get_input()), 5770);
    }
}
