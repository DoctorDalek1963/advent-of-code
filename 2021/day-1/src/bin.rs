use crate::*;

pub fn process_part1(s: &str) -> u32 {
    count_increasing_depth_differences(s)
}

pub fn process_part2(s: &str) -> u32 {
    count_increasing_window_sums(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "199
200
208
210
200
207
240
269
260
263
";

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(INPUT), 7);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(INPUT), 5);
    }
}
