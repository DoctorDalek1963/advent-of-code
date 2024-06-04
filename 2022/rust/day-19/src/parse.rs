use crate::{Blueprint, Cost, ResourceType};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

fn parse_num_and_resource(input: &str) -> IResult<&str, (u16, ResourceType)> {
    use ResourceType::*;

    separated_pair(
        complete::u16,
        tag(" "),
        alt((
            tag("ore").map(|_| Ore),
            tag("clay").map(|_| Clay),
            tag("obsidian").map(|_| Obsidian),
        )),
    )(input)
}

fn parse_cost(input: &str) -> IResult<&str, Cost> {
    separated_list1(tag(" and "), parse_num_and_resource)(input)
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, id_number) = preceded(tag("Blueprint "), complete::u32)(input)?;
    let (input, ore_robot_cost) = preceded(tag(": Each ore robot costs "), parse_cost)(input)?;
    let (input, clay_robot_cost) = preceded(tag(". Each clay robot costs "), parse_cost)(input)?;
    let (input, obsidian_robot_cost) =
        preceded(tag(". Each obsidian robot costs "), parse_cost)(input)?;
    let (input, geode_robot_cost) = preceded(tag(". Each geode robot costs "), parse_cost)(input)?;
    let (input, _) = tag(".")(input)?;

    Ok((
        input,
        Blueprint {
            id_number,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        },
    ))
}

pub fn parse_blueprint_list(input: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list1(tag("\n"), parse_blueprint)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    fn parse_blueprint_list_test() {
        use ResourceType::*;

        let blueprint_list = vec![
            Blueprint {
                id_number: 1,
                ore_robot_cost: vec![(4, Ore)],
                clay_robot_cost: vec![(2, Ore)],
                obsidian_robot_cost: vec![(3, Ore), (14, Clay)],
                geode_robot_cost: vec![(2, Ore), (7, Obsidian)],
            },
            Blueprint {
                id_number: 2,
                ore_robot_cost: vec![(2, Ore)],
                clay_robot_cost: vec![(3, Ore)],
                obsidian_robot_cost: vec![(3, Ore), (8, Clay)],
                geode_robot_cost: vec![(3, Ore), (12, Obsidian)],
            },
        ];

        assert_eq!(parse_blueprint_list(TEST_INPUT), Ok(("\n", blueprint_list)));
    }
}
