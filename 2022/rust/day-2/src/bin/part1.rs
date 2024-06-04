use day_2::HandShape;
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fs,
};

fn process_game(line: &str) -> u32 {
    let elf_shape: HandShape = line.chars().nth(0).unwrap().into();
    let player_shape: HandShape = line.chars().nth(2).unwrap().into();

    player_shape.raw_score()
        + match player_shape.cmp(&elf_shape) {
            Greater => 6,
            Equal => 3,
            Less => 0,
        }
}

fn main() {
    let file_contents = fs::read_to_string("./input.txt").unwrap();

    let total_score = file_contents
        .lines()
        .map(|line| process_game(line))
        .sum::<u32>();

    println!("{}", total_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_game_test() {
        assert_eq!(process_game("A Y"), 8);
        assert_eq!(process_game("B X"), 1);
        assert_eq!(process_game("C Z"), 6);
    }
}
