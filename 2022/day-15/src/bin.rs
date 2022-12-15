#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str, y_level: i32) -> usize {
    count_occupied_cells_at_y_level(parse_sensor_beacon_pairs(input).unwrap().1, y_level)
}

pub fn process_part2(input: &str, max: i32) -> u64 {
    get_tuning_frequency_of_beacon_pos(parse_sensor_beacon_pairs(input).unwrap().1, max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT, 10), 26);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT, 20), 56_000_011);
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "only run in release mode")]
    fn real_input_test() {
        assert_eq!(process_part1(&get_input(), 2_000_000), 4_873_353);
        assert_eq!(process_part2(&get_input(), 4_000_000), 11_600_823_139_120);
    }
}
