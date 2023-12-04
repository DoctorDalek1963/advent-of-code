#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;
use std::collections::HashMap;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    parse_scratchcards(input)
        .unwrap()
        .1
        .into_iter()
        .map(|card| {
            let count = card.count_winning_numbers() as u32;
            if count == 0 {
                0
            } else {
                2usize.pow(count - 1)
            }
        })
        .sum()
}

pub fn process_part2(input: &str) -> usize {
    // Map from the card ID to a tuple `(copies, winning_numbers)`, which is how many copies we
    // have of that card, and how many winning numbers it has
    let mut map: HashMap<u8, (usize, usize)> = parse_scratchcards(input)
        .unwrap()
        .1
        .into_iter()
        .map(|card| (card.id, (1, card.count_winning_numbers())))
        .collect();

    // Map from card ID to winning numbers. We iterate this one and mutate `map`
    let win_map: Vec<(u8, usize)> = {
        let mut v: Vec<_> = map
            .iter()
            .map(|(&id, &(_copies, winning_numbers))| (id, winning_numbers))
            .collect();
        v.sort_by_key(|&(id, _)| id);
        v
    };

    for (id, winning_numbers) in win_map {
        let copies_of_this_card = map.get(&id).unwrap().0;
        for offset in 1..=winning_numbers {
            map.get_mut(&(id + offset as u8)).unwrap().0 += copies_of_this_card;
        }
    }

    dbg!(map).into_values().map(|(copies, _)| copies).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 13);
        assert_eq!(process_part1(&get_input()), 20_107);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 30);
        assert_eq!(process_part2(&get_input()), 8_172_507);
    }
}
