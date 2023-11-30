#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> u16 {
    input.lines().map(|line| get_seat_id(line)).max().unwrap()
}

pub fn process_part2(input: &str) -> u16 {
    let mut ids: Vec<u16> = input.lines().map(|line| get_seat_id(line)).collect();
    ids.sort();
    for &[a, b] in ids.array_windows() {
        if b == a + 2 {
            return a + 1;
        }
    }
    panic!("Should hav found seat ID");
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(&get_input()), 991);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(&get_input()), 534);
    }
}
