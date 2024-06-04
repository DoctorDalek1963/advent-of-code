use crate::{CubeSet, Game};
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(newline, parse_game)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = complete::u16(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, cube_sets) = parse_cube_sets(input)?;

    Ok((input, Game { id, cube_sets }))
}

fn parse_cube_sets(input: &str) -> IResult<&str, Vec<CubeSet>> {
    separated_list1(tag("; "), parse_cube_set)(input)
}

fn parse_cube_set(input: &str) -> IResult<&str, CubeSet> {
    let (input, pairs) =
        separated_list1(tag(", "), separated_pair(complete::u16, tag(" "), alpha1))(input)?;

    let mut set = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };
    for (num, colour) in pairs {
        match colour {
            "red" => set.red = num,
            "green" => set.green = num,
            "blue" => set.blue = num,
            _ => (),
        };
    }
    Ok((input, set))
}
