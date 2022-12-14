#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> u32 {
    let (numbers, mut cards) = parse_numbers_and_cards(input).unwrap().1;
    for n in numbers {
        for c in &mut cards {
            c.add_number(n);

            if c.has_won() {
                return c.score();
            }
        }
    }

    unreachable!("at least one card should win");
}

pub fn process_part2(input: &str) -> u32 {
    let (numbers, mut cards) = parse_numbers_and_cards(input).unwrap().1;
    let mut all_other_cards_won = false;

    for n in numbers {
        if !all_other_cards_won {
            for c in &mut cards {
                c.add_number(n);
            }

            all_other_cards_won = cards.iter().filter(|card| !card.has_won()).count() == 1;
        } else {
            let last_card_left = cards.iter_mut().find(|card| !card.has_won()).unwrap();

            last_card_left.add_number(n);
            if last_card_left.has_won() {
                return last_card_left.score();
            }
        }
    }

    unreachable!("there should be a final winner");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 4512);
        assert_eq!(process_part1(&get_input()), 44088);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 1924);
        assert_eq!(process_part2(&get_input()), 23670);
    }
}
