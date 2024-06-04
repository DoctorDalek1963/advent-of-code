use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, newline},
    multi::separated_list1,
    IResult,
};

pub mod bin;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Node<'s> {
    this_name: &'s str,
    connected: (&'s str, &'s str),
}

fn parse_node<'s>(input: &'s str) -> IResult<&'s str, Node<'s>> {
    let (input, this_name) = alphanumeric1(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left_name) = alphanumeric1(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right_name) = alphanumeric1(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((
        input,
        Node {
            this_name,
            connected: (left_name, right_name),
        },
    ))
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, raw) = alpha1(input)?;
    let directions = raw
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!("We should only be parsing L and R for directions"),
        })
        .collect();

    Ok((input, directions))
}

pub fn parse_directions_and_nodes<'s>(
    input: &'s str,
) -> IResult<&'s str, (Vec<Direction>, Vec<Node<'s>>)> {
    let (input, directions) = parse_directions(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, nodes) = separated_list1(newline, parse_node)(input)?;

    Ok((input, (directions, nodes)))
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_directions_and_nodes_test() {
        use Direction::*;

        assert_eq!(
            parse_directions_and_nodes(TEST_INPUT),
            Ok((
                "\n",
                (
                    vec![Right, Left],
                    vec![
                        Node {
                            this_name: "AAA",
                            connected: ("BBB", "CCC")
                        },
                        Node {
                            this_name: "BBB",
                            connected: ("DDD", "EEE",),
                        },
                        Node {
                            this_name: "CCC",
                            connected: ("ZZZ", "GGG",),
                        },
                        Node {
                            this_name: "DDD",
                            connected: ("DDD", "DDD",),
                        },
                        Node {
                            this_name: "EEE",
                            connected: ("EEE", "EEE",),
                        },
                        Node {
                            this_name: "GGG",
                            connected: ("GGG", "GGG",),
                        },
                        Node {
                            this_name: "ZZZ",
                            connected: ("ZZZ", "ZZZ",),
                        },
                    ]
                )
            ))
        );
    }
}
