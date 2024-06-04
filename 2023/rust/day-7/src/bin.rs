#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    parse_hands_and_bids(input, false)
        .unwrap()
        .1
        .into_iter()
        .map(|(hand, bid)| (hand, hand.get_type(), bid))
        .sorted_by_key(|&(hand, hand_type, _bid)| (hand_type, hand))
        .enumerate()
        .map(|(rank_minus_one, (_hand, _hand_type, bid))| bid as usize * (rank_minus_one + 1))
        .sum()
}

pub fn process_part2(input: &str) -> usize {
    parse_hands_and_bids(input, true)
        .unwrap()
        .1
        .into_iter()
        .map(|(hand, bid)| (hand, hand.get_type_with_joker_rule(), bid))
        .sorted_by_key(|&(hand, hand_type, _bid)| (hand_type, hand))
        .enumerate()
        .map(|(rank_minus_one, (_hand, _hand_type, bid))| bid as usize * (rank_minus_one + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 6440);
        assert_eq!(process_part1(&get_input()), 253_205_868);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 5905);
        assert_eq!(process_part2(&get_input()), 253_907_829);
    }
}
