#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

fn loop_chamber(input: &str, num: usize) -> u64 {
    let mut chamber = Chamber::new(parse_directions(input));
    for i in 0..num {
        chamber.drop_rock();
        if i % 1_000_000 == 0 {
            eprintln!("i: {i:?}");
        }
    }
    chamber.max_height().unwrap() + 1
}

pub fn process_part1(input: &str) -> u64 {
    loop_chamber(input, 2022)
}

pub fn process_part2(input: &str) -> u64 {
    loop_chamber(input, 1_000_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 3068);
        assert_eq!(process_part1(&get_input()), 3235);
    }

    #[test]
    #[ignore = "takes forever (apparently over 2 months)"]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 1514285714288);
        //assert_eq!(process_part2(&get_input()), 1);
    }
}
