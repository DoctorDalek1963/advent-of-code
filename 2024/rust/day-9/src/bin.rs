#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    compute_checksum(&compact_fs_id_list(filesystem_to_id_list(&parse_disk_map(
        input,
    ))))
}

pub fn process_part2(input: &str) -> usize {
    compute_checksum(&defrag_fs(parse_disk_map(input)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    #[cfg_attr(debug_assertions, ignore = "part 1 is very slow in debug builds")]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 1928);
        assert_eq!(process_part1(&get_input()), 6_435_922_584_968);
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore = "part 2 is slow in debug builds")]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 2858);
        assert_eq!(process_part2(&get_input()), 6_469_636_832_766);
    }
}
