use super::{Instruction, Stacks};
use nom::{bytes::complete::tag, character::complete::u16, multi::separated_list1, IResult};

pub fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("move ")(input)?;
    let (input, num) = u16(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, source) = u16(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, dest) = u16(input)?;

    Ok((
        input,
        Instruction {
            source: source.into(),
            dest: dest.into(),
            num,
        },
    ))
}

pub fn parse_all_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(tag("\n"), parse_instruction)(input)
}

fn parse_single_crate(input: &str) -> IResult<&str, Option<char>> {
    let three_chars = &input[..3];

    let char_ = if three_chars.starts_with('[') && three_chars.ends_with(']') {
        input.chars().nth(1)
    } else if three_chars == "   " {
        None
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };

    Ok((&input[3..], char_))
}

fn parse_row_of_crates(input: &str) -> IResult<&str, [Option<char>; 9]> {
    let (input, chars) = separated_list1(tag(" "), parse_single_crate)(input)?;
    let mut arr = [None; 9];
    arr.copy_from_slice(&chars[..9]);
    Ok((input, arr))
}

fn parse_all_crates(input: &str) -> IResult<&str, Stacks> {
    #[rustfmt::skip]
    let mut stacks: Stacks = [
        vec![], vec![], vec![],
        vec![], vec![], vec![],
        vec![], vec![], vec![],
    ];

    let (input, rows) = separated_list1(tag("\n"), parse_row_of_crates)(input)?;
    for char_arr in rows.iter().rev() {
        for i in 0..9 {
            match char_arr[i] {
                None => (),
                Some(c) => stacks[i].push(c),
            };
        }
    }

    Ok((input, stacks))
}

pub fn parse_whole_file(input: &str) -> IResult<&str, (Stacks, Vec<Instruction>)> {
    let (input, stacks) = parse_all_crates(input)?;
    let (input, _) = tag("\n 1   2   3   4   5   6   7   8   9 \n\n")(input)?;
    let (input, instructions) = parse_all_instructions(input)?;

    Ok((input, (stacks, instructions)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_crate_test() {
        assert_eq!(
            parse_single_crate("[A]     [B]"),
            Ok(("     [B]", Some('A')))
        );

        assert_eq!(parse_single_crate("    [D]"), Ok((" [D]", None)))
    }

    #[test]
    fn parse_row_of_crates_test() {
        assert_eq!(
            parse_row_of_crates("    [F] [T] [P] [B] [D]     [N]    "),
            Ok((
                "",
                [
                    None,
                    Some('F'),
                    Some('T'),
                    Some('P'),
                    Some('B'),
                    Some('D'),
                    None,
                    Some('N'),
                    None
                ]
            ))
        );
    }

    #[test]
    fn parse_all_crates_test() {
        const INPUT: &str = "    [G] [R]                 [P]    
    [H] [W]     [T] [P]     [H]    
    [F] [T] [P] [B] [D]     [N]    
[L] [T] [M] [Q] [L] [C]     [Z]    
[C] [C] [N] [V] [S] [H]     [V] [G]
[G] [L] [F] [D] [M] [V] [T] [J] [H]
[M] [D] [J] [F] [F] [N] [C] [S] [F]
[Q] [R] [V] [J] [N] [R] [H] [G] [Z]
 1   2   3   4   5   6   7   8   9 

...
";

        assert_eq!(
            parse_all_crates(INPUT),
            Ok((
                "\n 1   2   3   4   5   6   7   8   9 \n\n...\n",
                [
                    vec!['Q', 'M', 'G', 'C', 'L'],
                    vec!['R', 'D', 'L', 'C', 'T', 'F', 'H', 'G'],
                    vec!['V', 'J', 'F', 'N', 'M', 'T', 'W', 'R'],
                    vec!['J', 'F', 'D', 'V', 'Q', 'P'],
                    vec!['N', 'F', 'M', 'S', 'L', 'B', 'T'],
                    vec!['R', 'N', 'V', 'H', 'C', 'D', 'P'],
                    vec!['H', 'C', 'T'],
                    vec!['G', 'S', 'J', 'V', 'Z', 'N', 'H', 'P'],
                    vec!['Z', 'F', 'H', 'G'],
                ]
            ))
        );
    }

    #[test]
    fn parse_whole_file_test() {
        const INPUT: &str = "    [G] [R]                 [P]    
    [H] [W]     [T] [P]     [H]    
    [F] [T] [P] [B] [D]     [N]    
[L] [T] [M] [Q] [L] [C]     [Z]    
[C] [C] [N] [V] [S] [H]     [V] [G]
[G] [L] [F] [D] [M] [V] [T] [J] [H]
[M] [D] [J] [F] [F] [N] [C] [S] [F]
[Q] [R] [V] [J] [N] [R] [H] [G] [Z]
 1   2   3   4   5   6   7   8   9 

move 5 from 8 to 2
move 2 from 4 to 5
move 3 from 3 to 9
move 4 from 1 to 8
move 5 from 9 to 1
";

        assert_eq!(
            parse_whole_file(INPUT),
            Ok((
                "\n",
                (
                    [
                        vec!['Q', 'M', 'G', 'C', 'L'],
                        vec!['R', 'D', 'L', 'C', 'T', 'F', 'H', 'G'],
                        vec!['V', 'J', 'F', 'N', 'M', 'T', 'W', 'R'],
                        vec!['J', 'F', 'D', 'V', 'Q', 'P'],
                        vec!['N', 'F', 'M', 'S', 'L', 'B', 'T'],
                        vec!['R', 'N', 'V', 'H', 'C', 'D', 'P'],
                        vec!['H', 'C', 'T'],
                        vec!['G', 'S', 'J', 'V', 'Z', 'N', 'H', 'P'],
                        vec!['Z', 'F', 'H', 'G'],
                    ],
                    vec![
                        Instruction {
                            source: 8,
                            dest: 2,
                            num: 5
                        },
                        Instruction {
                            source: 4,
                            dest: 5,
                            num: 2
                        },
                        Instruction {
                            source: 3,
                            dest: 9,
                            num: 3
                        },
                        Instruction {
                            source: 1,
                            dest: 8,
                            num: 4
                        },
                        Instruction {
                            source: 9,
                            dest: 1,
                            num: 5
                        },
                    ]
                )
            ))
        );
    }
}
