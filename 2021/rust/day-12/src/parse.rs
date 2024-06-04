use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn parse_pairs(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(newline, separated_pair(alpha1, tag("-"), alpha1))(input)
}
