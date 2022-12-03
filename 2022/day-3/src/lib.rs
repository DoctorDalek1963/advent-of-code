use std::fs;

pub fn get_input() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

fn get_types_from_rucksack(rucksack: &str) -> (Vec<char>, Vec<char>) {
    let length = rucksack.len() / 2;
    let first = &rucksack[..length];
    let last = &rucksack[length..];

    (first.chars().collect(), last.chars().collect())
}

fn get_mismatched_type(types: (Vec<char>, Vec<char>)) -> Option<char> {
    let (first, last) = types;
    first.iter().filter(|&c| last.contains(c)).copied().next()
}

#[inline]
pub fn get_mismatched_type_from_rucksack(rucksack: &str) -> Option<char> {
    get_mismatched_type(get_types_from_rucksack(rucksack))
}

pub fn char_to_priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - 96,
        'A'..='Z' => (c as u32) - 38,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_types_from_rucksack_test() {
        assert_eq!(
            get_types_from_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp"),
            (
                vec!['v', 'J', 'r', 'w', 'p', 'W', 't', 'w', 'J', 'g', 'W', 'r',],
                vec!['h', 'c', 's', 'F', 'M', 'M', 'f', 'F', 'F', 'h', 'F', 'p',]
            )
        );

        assert_eq!(
            get_types_from_rucksack("ttgJtRGJQctTZtZT"),
            (
                vec!['t', 't', 'g', 'J', 't', 'R', 'G', 'J',],
                vec!['Q', 'c', 't', 'T', 'Z', 't', 'Z', 'T',]
            )
        );
    }

    #[rustfmt::skip]
    #[test]
    fn get_mismatched_type_test() {
        assert_eq!(get_mismatched_type(get_types_from_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp")).unwrap(), 'p');
        assert_eq!(get_mismatched_type(get_types_from_rucksack("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")).unwrap(), 'L');
        assert_eq!(get_mismatched_type(get_types_from_rucksack("PmmdzqPrVvPwwTWBwg")).unwrap(), 'P');
        assert_eq!(get_mismatched_type(get_types_from_rucksack("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")).unwrap(), 'v');
        assert_eq!(get_mismatched_type(get_types_from_rucksack("ttgJtRGJQctTZtZT")).unwrap(), 't');
        assert_eq!(get_mismatched_type(get_types_from_rucksack("CrZsJsPPZsGzwwsLwLmpwMDw")).unwrap(), 's');
    }

    #[test]
    fn char_to_priority_test() {
        assert_eq!(char_to_priority('a'), 1);
        assert_eq!(char_to_priority('z'), 26);
        assert_eq!(char_to_priority('A'), 27);
        assert_eq!(char_to_priority('Z'), 52);
        assert_eq!(char_to_priority('m'), 13);
        assert_eq!(char_to_priority('F'), 32);
    }
}
