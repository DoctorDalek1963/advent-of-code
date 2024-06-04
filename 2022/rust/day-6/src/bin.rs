use crate::*;

pub fn process_part1(s: &str) -> usize {
    find_unique_slice::<4>(s)
}

pub fn process_part2(s: &str) -> usize {
    find_unique_slice::<14>(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(INPUT), 7);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(INPUT), 19);
    }
}
