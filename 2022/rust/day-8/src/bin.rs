use crate::*;

pub fn process_part1<const N: usize>(s: &str) -> usize {
    count_visible_trees(parse_height_grid::<N>(s))
}

pub fn process_part2<const N: usize>(s: &str) -> u32 {
    find_best_scenic_score(parse_height_grid::<N>(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1::<5>(INPUT), 21);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2::<5>(INPUT), 8);
    }
}
