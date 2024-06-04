use crate::*;

pub fn process_part1(s: &str) -> i32 {
    let mut position = Position::default();

    for command in parse_commands(s).unwrap().1 {
        position.execute_command_part1(command);
    }

    position.position_product()
}

pub fn process_part2(s: &str) -> i32 {
    let mut position = Position::default();

    for command in parse_commands(s).unwrap().1 {
        position.execute_command_part2(command);
    }

    position.position_product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(INPUT), 150);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(INPUT), 900);
    }
}
