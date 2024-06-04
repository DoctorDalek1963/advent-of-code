use lazy_static::lazy_static;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use nom_regex::str::re_find;
use regex::Regex;

pub mod bin;

lazy_static! {
    static ref SPRING_STATE_CHARS: Regex = Regex::new(r"^[.#?]+").unwrap();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

fn parse_spring_states(input: &str) -> IResult<&str, Vec<SpringState>> {
    let (input, state_str) = re_find(SPRING_STATE_CHARS.clone())(input)?;
    let states = state_str
        .chars()
        .map(|c| match c {
            '.' => SpringState::Operational,
            '#' => SpringState::Damaged,
            '?' => SpringState::Unknown,
            _ => unreachable!("We can only match the covered characters"),
        })
        .collect();
    Ok((input, states))
}

pub fn parse_spring_states_and_group_lengths(
    input: &str,
) -> IResult<&str, Vec<(Vec<SpringState>, Vec<u8>)>> {
    separated_list1(
        newline,
        separated_pair(
            parse_spring_states,
            tag(" "),
            separated_list1(tag(","), complete::u8),
        ),
    )(input)
}

pub fn count_possible_arrangements(spring_states: &[SpringState], group_lengths: &[u8]) -> usize {
    dbg!(2usize.pow(
        spring_states
            .iter()
            .filter(|s| s == &&SpringState::Unknown)
            .count() as u32
    ));
    1
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_spring_states_and_group_lengths_test() {
        use SpringState::*;

        assert_eq!(
            parse_spring_states_and_group_lengths(TEST_INPUT),
            Ok((
                "\n",
                vec![
                    (
                        vec![
                            Unknown,
                            Unknown,
                            Unknown,
                            Operational,
                            Damaged,
                            Damaged,
                            Damaged,
                        ],
                        vec![1, 1, 3],
                    ),
                    (
                        vec![
                            Operational,
                            Unknown,
                            Unknown,
                            Operational,
                            Operational,
                            Unknown,
                            Unknown,
                            Operational,
                            Operational,
                            Operational,
                            Unknown,
                            Damaged,
                            Damaged,
                            Operational,
                        ],
                        vec![1, 1, 3],
                    ),
                    (
                        vec![
                            Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown, Damaged,
                            Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown,
                        ],
                        vec![1, 3, 1, 6],
                    ),
                    (
                        vec![
                            Unknown,
                            Unknown,
                            Unknown,
                            Unknown,
                            Operational,
                            Damaged,
                            Operational,
                            Operational,
                            Operational,
                            Damaged,
                            Operational,
                            Operational,
                            Operational,
                        ],
                        vec![4, 1, 1],
                    ),
                    (
                        vec![
                            Unknown,
                            Unknown,
                            Unknown,
                            Unknown,
                            Operational,
                            Damaged,
                            Damaged,
                            Damaged,
                            Damaged,
                            Damaged,
                            Damaged,
                            Operational,
                            Operational,
                            Damaged,
                            Damaged,
                            Damaged,
                            Damaged,
                            Damaged,
                            Operational,
                        ],
                        vec![1, 6, 5],
                    ),
                    (
                        vec![
                            Unknown, Damaged, Damaged, Damaged, Unknown, Unknown, Unknown, Unknown,
                            Unknown, Unknown, Unknown, Unknown,
                        ],
                        vec![3, 2, 1],
                    ),
                ],
            ))
        );
    }

    #[test]
    #[ignore = "I abandoned this day's solution"]
    fn count_possible_arrangements_test() {
        use SpringState::*;

        // ???.### 1,1,3
        assert_eq!(
            count_possible_arrangements(
                &[
                    Unknown,
                    Unknown,
                    Unknown,
                    Operational,
                    Damaged,
                    Damaged,
                    Damaged,
                ],
                &[1, 1, 3]
            ),
            1
        );

        // .??..??...?##. 1,1,3
        assert_eq!(
            count_possible_arrangements(
                &[
                    Operational,
                    Unknown,
                    Unknown,
                    Operational,
                    Operational,
                    Unknown,
                    Unknown,
                    Operational,
                    Operational,
                    Operational,
                    Unknown,
                    Damaged,
                    Damaged,
                    Operational,
                ],
                &[1, 1, 3]
            ),
            4
        );

        // ?#?#?#?#?#?#?#? 1,3,1,6
        assert_eq!(
            count_possible_arrangements(
                &[
                    Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown, Damaged,
                    Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown,
                ],
                &[1, 3, 1, 6]
            ),
            1
        );

        // ????.#...#... 4,1,1
        assert_eq!(
            count_possible_arrangements(
                &[
                    Unknown,
                    Unknown,
                    Unknown,
                    Unknown,
                    Operational,
                    Damaged,
                    Operational,
                    Operational,
                    Operational,
                    Damaged,
                    Operational,
                    Operational,
                    Operational,
                ],
                &[4, 1, 1]
            ),
            4
        );

        // ?###???????? 3,2,1
        assert_eq!(
            count_possible_arrangements(
                &[
                    Unknown, Damaged, Damaged, Damaged, Unknown, Unknown, Unknown, Unknown,
                    Unknown, Unknown, Unknown, Unknown,
                ],
                &[3, 2, 1]
            ),
            10
        );
    }
}
