use crate::PacketComp;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn parse_packet_comp(input: &str) -> IResult<&str, PacketComp> {
    use PacketComp::*;

    alt((
        complete::u32.map(|x| Int(x)),
        delimited(
            tag("["),
            separated_list0(tag(","), parse_packet_comp).map(|x| List(x)),
            tag("]"),
        ),
    ))(input)
}

pub fn parse_packet_pairs(input: &str) -> IResult<&str, Vec<(PacketComp, PacketComp)>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(parse_packet_comp, newline, parse_packet_comp),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_packet_comp_test() {
        use PacketComp::*;

        assert_eq!(parse_packet_comp("1"), Ok(("", Int(1))));
        assert_eq!(
            parse_packet_comp("[1,2]"),
            Ok(("", List(vec![Int(1), Int(2)])))
        );
        assert_eq!(parse_packet_comp("[]"), Ok(("", List(vec![]))));
        assert_eq!(
            parse_packet_comp("[1,[2,3]]"),
            Ok(("", List(vec![Int(1), List(vec![Int(2), Int(3)])])))
        );
        assert_eq!(
            parse_packet_comp("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
            Ok((
                "",
                List(vec![
                    Int(1),
                    List(vec![
                        Int(2),
                        List(vec![
                            Int(3),
                            List(vec![Int(4), List(vec![Int(5), Int(6), Int(7)])])
                        ])
                    ]),
                    Int(8),
                    Int(9)
                ])
            ))
        );
    }

    #[test]
    fn parse_packet_pairs_test() {
        use PacketComp::*;

        assert_eq!(
            parse_packet_pairs(TEST_INPUT),
            Ok((
                "\n",
                vec![
                    (
                        List(vec![Int(1), Int(1), Int(3), Int(1), Int(1)]),
                        List(vec![Int(1), Int(1), Int(5), Int(1), Int(1)])
                    ),
                    (
                        List(vec![List(vec![Int(1)]), List(vec![Int(2), Int(3), Int(4)])]),
                        List(vec![List(vec![Int(1)]), Int(4)])
                    ),
                    (
                        List(vec![Int(9)]),
                        List(vec![List(vec![Int(8), Int(7), Int(6)])])
                    ),
                    (
                        List(vec![List(vec![Int(4), Int(4)]), Int(4), Int(4)]),
                        List(vec![List(vec![Int(4), Int(4)]), Int(4), Int(4), Int(4)])
                    ),
                    (
                        List(vec![Int(7), Int(7), Int(7), Int(7)]),
                        List(vec![Int(7), Int(7), Int(7)])
                    ),
                    (List(vec![]), List(vec![Int(3)])),
                    (
                        List(vec![List(vec![List(vec![])])]),
                        List(vec![List(vec![])])
                    ),
                    (
                        List(vec![
                            Int(1),
                            List(vec![
                                Int(2),
                                List(vec![
                                    Int(3),
                                    List(vec![Int(4), List(vec![Int(5), Int(6), Int(7)])])
                                ])
                            ]),
                            Int(8),
                            Int(9)
                        ]),
                        List(vec![
                            Int(1),
                            List(vec![
                                Int(2),
                                List(vec![
                                    Int(3),
                                    List(vec![Int(4), List(vec![Int(5), Int(6), Int(0)])])
                                ])
                            ]),
                            Int(8),
                            Int(9)
                        ])
                    ),
                ]
            ))
        );
    }
}
