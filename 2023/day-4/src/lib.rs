use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1, newline},
    multi::separated_list1,
    IResult,
};
use std::collections::HashSet;

pub mod bin;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scratchcard {
    id: u8,
    winning_numbers: Vec<u8>,
    numbers_we_have: Vec<u8>,
}

impl Scratchcard {
    pub fn count_winning_numbers(&self) -> usize {
        let winning: HashSet<u8> = self.winning_numbers.iter().copied().collect();
        let we_have: HashSet<u8> = self.numbers_we_have.iter().copied().collect();

        winning.intersection(&we_have).count()
    }
}

pub fn parse_scratchcards(input: &str) -> IResult<&str, Vec<Scratchcard>> {
    separated_list1(newline, parse_scratchcard)(input)
}

fn parse_scratchcard(input: &str) -> IResult<&str, Scratchcard> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, id) = complete::u8(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, winning_numbers) = separated_list1(multispace1, complete::u8)(input)?;
    let (input, _) = tag(" |")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, numbers_we_have) = separated_list1(multispace1, complete::u8)(input)?;

    Ok((
        input,
        Scratchcard {
            id,
            winning_numbers,
            numbers_we_have,
        },
    ))
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_scratchcards_test() {
        assert_eq!(
            parse_scratchcards(TEST_INPUT),
            Ok((
                "\n",
                vec![
                    Scratchcard {
                        id: 1,
                        winning_numbers: vec![41, 48, 83, 86, 17,],
                        numbers_we_have: vec![83, 86, 6, 31, 17, 9, 48, 53,],
                    },
                    Scratchcard {
                        id: 2,
                        winning_numbers: vec![13, 32, 20, 16, 61,],
                        numbers_we_have: vec![61, 30, 68, 82, 17, 32, 24, 19,],
                    },
                    Scratchcard {
                        id: 3,
                        winning_numbers: vec![1, 21, 53, 59, 44,],
                        numbers_we_have: vec![69, 82, 63, 72, 16, 21, 14, 1,],
                    },
                    Scratchcard {
                        id: 4,
                        winning_numbers: vec![41, 92, 73, 84, 69,],
                        numbers_we_have: vec![59, 84, 76, 51, 58, 5, 54, 83,],
                    },
                    Scratchcard {
                        id: 5,
                        winning_numbers: vec![87, 83, 26, 28, 32,],
                        numbers_we_have: vec![88, 30, 70, 12, 93, 22, 82, 36,],
                    },
                    Scratchcard {
                        id: 6,
                        winning_numbers: vec![31, 18, 13, 56, 72,],
                        numbers_we_have: vec![74, 77, 10, 23, 35, 67, 36, 11,],
                    },
                ]
            ))
        );
    }
}
