#![feature(array_windows)]

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

pub mod bin;

pub fn get_next_number(sequence: &[i64]) -> i64 {
    if sequence.iter().all(|&x| x == 0) {
        0
    } else {
        let diffs: Vec<_> = sequence.array_windows().map(|&[a, b]| b - a).collect();
        sequence.last().unwrap() + get_next_number(&diffs)
    }
}

pub fn get_previous_number(sequence: &[i64]) -> i64 {
    if sequence.iter().all(|&x| x == 0) {
        0
    } else {
        let diffs: Vec<_> = sequence.array_windows().map(|&[a, b]| b - a).collect();
        sequence.first().unwrap() - get_previous_number(&diffs)
    }
}

pub fn parse(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(newline, separated_list1(tag(" "), complete::i64))(input)
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_test() {
        assert_eq!(
            parse(TEST_INPUT),
            Ok((
                "\n",
                vec![
                    vec![0, 3, 6, 9, 12, 15],
                    vec![1, 3, 6, 10, 15, 21],
                    vec![10, 13, 16, 21, 30, 45],
                ]
            ))
        );
    }

    #[test]
    fn get_next_number_test() {
        assert_eq!(get_next_number(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(get_next_number(&[1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(get_next_number(&[10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn get_previous_number_test() {
        assert_eq!(get_previous_number(&[0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(get_previous_number(&[1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(get_previous_number(&[10, 13, 16, 21, 30, 45]), 5);
    }
}
