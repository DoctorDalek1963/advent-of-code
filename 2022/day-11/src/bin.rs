use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(s: &str) -> u128 {
    let mut group = parse_monkey_group(s).unwrap().1;
    for _ in 0..20 {
        group.do_round(true);
    }
    group.get_monkey_business()
}

pub fn process_part2(s: &str) -> u128 {
    let mut group = parse_monkey_group(s).unwrap().1;
    for _ in 0..10_000 {
        group.do_round(false);
    }
    group.get_monkey_business()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 10605);
        assert_eq!(process_part1(&get_input()), 55216);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 2713310158);
        assert_eq!(process_part2(&get_input()), 12848882750);
    }
}
