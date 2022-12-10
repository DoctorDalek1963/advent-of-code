use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1<const N: usize>(s: &str) -> u32 {
    find_power_consumption(parse_bit_arrays::<N>(s).unwrap().1)
}

pub fn process_part2<const N: usize>(s: &str) -> u32 {
    find_life_support_rating(parse_bit_arrays::<N>(s).unwrap().1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1::<5>(INPUT), 198);
        assert_eq!(process_part1::<12>(&get_input()), 2003336);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2::<5>(INPUT), 230);
        assert_eq!(process_part2::<12>(&get_input()), 1877139);
    }
}
