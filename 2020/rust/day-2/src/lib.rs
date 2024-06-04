use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, multispace1},
    multi::separated_list1,
    IResult,
};

pub mod bin;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PasswordPolicy {
    minimum: u8,
    maximum: u8,
    character: char,
}

fn parse_password_policy(input: &str) -> IResult<&str, PasswordPolicy> {
    let (input, minimum) = complete::u8(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, maximum) = complete::u8(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, character) = complete::anychar(input)?;
    Ok((
        input,
        PasswordPolicy {
            minimum,
            maximum,
            character,
        },
    ))
}

fn parse_password_policy_pair(input: &str) -> IResult<&str, (PasswordPolicy, &str)> {
    let (input, policy) = parse_password_policy(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, password) = alpha1(input)?;
    Ok((input, (policy, password)))
}

pub fn parse_all_password_policy_pairs(input: &str) -> IResult<&str, Vec<(PasswordPolicy, &str)>> {
    separated_list1(multispace1, parse_password_policy_pair)(input)
}

pub fn is_password_valid_part_1(policy: PasswordPolicy, password: &str) -> bool {
    let count: u8 = password
        .chars()
        .filter(|c| *c == policy.character)
        .count()
        .try_into()
        .unwrap();
    count >= policy.minimum && count <= policy.maximum
}

pub fn is_password_valid_part_2(policy: PasswordPolicy, password: &str) -> bool {
    let pos1 = password.chars().nth(policy.minimum as usize - 1).unwrap() == policy.character;
    let pos2 = password.chars().nth(policy.maximum as usize - 1).unwrap() == policy.character;
    pos1 ^ pos2
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_all_password_policy_pairs_test() {
        assert_eq!(
            parse_all_password_policy_pairs(TEST_INPUT),
            Ok((
                "\n",
                vec![
                    (
                        PasswordPolicy {
                            minimum: 1,
                            maximum: 3,
                            character: 'a'
                        },
                        "abcde"
                    ),
                    (
                        PasswordPolicy {
                            minimum: 1,
                            maximum: 3,
                            character: 'b'
                        },
                        "cdefg"
                    ),
                    (
                        PasswordPolicy {
                            minimum: 2,
                            maximum: 9,
                            character: 'c'
                        },
                        "ccccccccc"
                    ),
                ]
            ))
        );
    }
}
