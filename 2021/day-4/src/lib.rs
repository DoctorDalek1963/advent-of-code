pub mod bin;

use nom::{
    branch::alt, bytes::complete::tag, character::complete, multi::separated_list1, IResult,
};

pub fn parse_numbers_and_cards(input: &str) -> IResult<&str, (Vec<u8>, Vec<BingoCard>)> {
    fn parse_card(input: &str) -> IResult<&str, BingoCard> {
        let (input, card) = separated_list1(
            alt((tag("\n "), tag("\n"))),
            separated_list1(alt((tag("  "), tag(" "))), complete::u8),
        )(input)?;

        let mut numbers: [[u8; 5]; 5] = [[0; 5]; 5];

        for i in 0..5 {
            numbers[i].copy_from_slice(&card[i][..]);
        }

        Ok((input, BingoCard::from_number_arrays(numbers)))
    }

    let (input, numbers) = separated_list1(tag(","), complete::u8)(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, cards) = separated_list1(alt((tag("\n\n "), tag("\n\n"))), parse_card)(input)?;

    Ok((input, (numbers, cards)))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BingoCard {
    numbers: [[(u8, bool); 5]; 5],
    last_called_number: Option<u8>,
}

impl BingoCard {
    fn from_number_arrays(arrays: [[u8; 5]; 5]) -> Self {
        Self {
            numbers: arrays.map(|row| row.map(|n| (n, false))),
            last_called_number: None,
        }
    }

    pub fn add_number(&mut self, number: u8) {
        for x in 0..5 {
            for y in 0..5 {
                if self.numbers[x][y] == (number, false) {
                    self.numbers[x][y] = (number, true);
                    self.last_called_number = Some(number);
                    return;
                }
            }
        }
    }

    pub fn score(&self) -> u32 {
        self.numbers
            .iter()
            .flatten()
            .filter_map(|&(n, b)| if !b { Some(n as u32) } else { None })
            .sum::<u32>()
            * self.last_called_number.unwrap() as u32
    }

    pub fn has_won(&self) -> bool {
        let r0 = &self.numbers[0][..];
        let r1 = &self.numbers[1][..];
        let r2 = &self.numbers[2][..];
        let r3 = &self.numbers[3][..];
        let r4 = &self.numbers[4][..];

        let c0 = &self.numbers.iter().map(|s| s[0]).collect::<Vec<_>>()[..];
        let c1 = &self.numbers.iter().map(|s| s[1]).collect::<Vec<_>>()[..];
        let c2 = &self.numbers.iter().map(|s| s[2]).collect::<Vec<_>>()[..];
        let c3 = &self.numbers.iter().map(|s| s[3]).collect::<Vec<_>>()[..];
        let c4 = &self.numbers.iter().map(|s| s[4]).collect::<Vec<_>>()[..];

        [r0, r1, r2, r3, r4, c0, c1, c2, c3, c4]
            .iter()
            .any(|&s| s.iter().all(|&(_, b)| b))
    }
}

#[cfg(test)]
pub const TEST_INPUT: &str =
    "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers_and_cards_test() {
        assert_eq!(
            parse_numbers_and_cards(TEST_INPUT),
            Ok((
                "\n",
                (
                    vec![
                        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22,
                        18, 20, 8, 19, 3, 26, 1
                    ],
                    vec![
                        BingoCard::from_number_arrays([
                            [22, 13, 17, 11, 0],
                            [8, 2, 23, 4, 24],
                            [21, 9, 14, 16, 7],
                            [6, 10, 3, 18, 5],
                            [1, 12, 20, 15, 19],
                        ]),
                        BingoCard::from_number_arrays([
                            [3, 15, 0, 2, 22],
                            [9, 18, 13, 17, 5],
                            [19, 8, 7, 25, 23],
                            [20, 11, 10, 24, 4],
                            [14, 21, 16, 12, 6]
                        ]),
                        BingoCard::from_number_arrays([
                            [14, 21, 17, 24, 4],
                            [10, 16, 15, 9, 19],
                            [18, 8, 23, 26, 20],
                            [22, 11, 13, 6, 5],
                            [2, 0, 12, 3, 7]
                        ]),
                    ]
                )
            ))
        );
    }
}
