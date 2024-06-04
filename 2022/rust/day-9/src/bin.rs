use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

fn process_tail_n<const N: usize>(s: &str) -> usize {
    let mut board = BoardState::<N>::default();
    let directions = parse_directions(s).unwrap().1;
    board.move_many_directions(directions);
    board.tail_visited_points.len()
}

pub fn process_part1(s: &str) -> usize {
    process_tail_n::<1>(s)
}

pub fn process_part2(s: &str) -> usize {
    process_tail_n::<9>(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part1_test() {
        const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

        assert_eq!(process_part1(INPUT), 13);
        assert_eq!(process_part1(&get_input()), 5683);
    }

    #[test]
    fn process_part2_test() {
        const INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

        assert_eq!(process_part2(INPUT), 36);
        assert_eq!(process_part2(&get_input()), 2372);
    }
}
