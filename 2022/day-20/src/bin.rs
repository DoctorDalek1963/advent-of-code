#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;
use tracing::instrument;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

#[instrument(skip(input))]
pub fn process_part1(input: &str) -> i32 {
    let v = dbg!(mix_list(parse_list(input)));
    let len = v.len();
    let idx = v
        .iter()
        .position(|&x| x == 0)
        .expect("0 must appear exactly once");

    dbg!(v[(idx + 1000) % len]) + dbg!(v[(idx + 2000) % len]) + dbg!(v[(idx + 3000) % len])
}

#[instrument(skip(input))]
pub fn process_part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    fn process_part1_test() {
        let _ = tracing_subscriber::fmt::try_init();
        assert_eq!(process_part1(TEST_INPUT), 3);
        //assert_eq!(process_part1(&get_input()), 1);
    }

    #[test]
    #[ignore]
    fn process_part2_test() {
        let _ = tracing_subscriber::fmt::try_init();
        assert_eq!(process_part2(TEST_INPUT), 1);
        //assert_eq!(process_part2(&get_input()), 1);
    }
}
