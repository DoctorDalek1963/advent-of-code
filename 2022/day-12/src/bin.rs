#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1<const W: usize, const H: usize>(s: &str) -> usize {
    let map = parse_map::<W, H>(s);
    map.find_minimum_steps_from_start()
}

pub fn process_part2<const W: usize, const H: usize>(s: &str) -> usize {
    let map = parse_map::<W, H>(s);
    map.find_minimum_steps_from_anywhere()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1::<8, 5>(INPUT), 31);
        assert_eq!(process_part1::<81, 41>(&get_input()), 350);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2::<8, 5>(INPUT), 29);

        // This test is slow, so only run it in release mode
        if !cfg!(debug_assertions) {
            assert_eq!(process_part2::<81, 41>(&get_input()), 349);
        }
    }
}
