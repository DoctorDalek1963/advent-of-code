use crate::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

pub fn parse_monkey_group(input: &str) -> IResult<&str, MonkeyGroup> {
    let (input, monkeys) = separated_list1(newline, parse_monkey)(input)?;
    Ok((input, MonkeyGroup { monkeys }))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, (_, _idx)) = separated_pair(tag("Monkey"), tag(" "), complete::u128)(input)?;
    let (input, _) = tag(":\n  Starting items: ")(input)?;
    let (input, items) = separated_list1(tag(", "), complete::u128)(input)?;

    let (input, operation) = parse_operation(input)?;

    let (input, test) = parse_monkey_test(input)?;

    let (input, _) = newline(input)?;

    Ok((
        input,
        Monkey {
            items,
            operation,
            test,
            inspect_counter: 0,
        },
    ))
}

fn parse_operation(input: &str) -> IResult<&str, MonkeyOperation> {
    fn parse_left_or_right(input: &str) -> IResult<&str, Option<u128>> {
        alt((tag("old").map(|_| None), complete::u128.map(|x| Some(x))))(input)
    }

    let (input, _) = tag("\n  Operation: new = ")(input)?;
    let (input, left) = parse_left_or_right(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, operator) = alt((
        tag("*").map(|_| Operator::Times),
        tag("+").map(|_| Operator::Plus),
    ))(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, right) = parse_left_or_right(input)?;

    Ok((
        input,
        MonkeyOperation {
            left,
            operator,
            right,
        },
    ))
}

fn parse_monkey_test(input: &str) -> IResult<&str, MonkeyTest> {
    let (input, _) = tag("\n  Test: divisible by ")(input)?;
    let (input, modulus) = complete::u128(input)?;

    let (input, _) = tag("\n    If true: throw to monkey ")(input)?;
    let (input, if_true) = complete::u128(input)?;

    let (input, _) = tag("\n    If false: throw to monkey ")(input)?;
    let (input, if_false) = complete::u128(input)?;

    Ok((
        input,
        MonkeyTest {
            modulus,
            if_true: if_true as usize,
            if_false: if_false as usize,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_monkey_group_test() {
        assert_eq!(
            parse_monkey_group(TEST_INPUT),
            Ok((
                "",
                MonkeyGroup {
                    monkeys: vec![
                        Monkey {
                            items: vec![79, 98],
                            operation: MonkeyOperation {
                                left: None,
                                operator: Operator::Times,
                                right: Some(19)
                            },
                            test: MonkeyTest {
                                modulus: 23,
                                if_true: 2,
                                if_false: 3
                            },
                            inspect_counter: 0
                        },
                        Monkey {
                            items: vec![54, 65, 75, 74],
                            operation: MonkeyOperation {
                                left: None,
                                operator: Operator::Plus,
                                right: Some(6)
                            },
                            test: MonkeyTest {
                                modulus: 19,
                                if_true: 2,
                                if_false: 0
                            },
                            inspect_counter: 0
                        },
                        Monkey {
                            items: vec![79, 60, 97],
                            operation: MonkeyOperation {
                                left: None,
                                operator: Operator::Times,
                                right: None,
                            },
                            test: MonkeyTest {
                                modulus: 13,
                                if_true: 1,
                                if_false: 3
                            },
                            inspect_counter: 0
                        },
                        Monkey {
                            items: vec![74],
                            operation: MonkeyOperation {
                                left: None,
                                operator: Operator::Plus,
                                right: Some(3)
                            },
                            test: MonkeyTest {
                                modulus: 17,
                                if_true: 0,
                                if_false: 1
                            },
                            inspect_counter: 0
                        },
                    ]
                }
            ))
        )
    }

    #[test]
    fn test_parse_monkey() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
";

        assert_eq!(
            parse_monkey(input),
            Ok((
                "",
                Monkey {
                    items: vec![79, 98],
                    operation: MonkeyOperation {
                        left: None,
                        operator: Operator::Times,
                        right: Some(19)
                    },
                    test: MonkeyTest {
                        modulus: 23,
                        if_true: 2,
                        if_false: 3
                    },
                    inspect_counter: 0
                }
            ))
        );
    }

    #[test]
    fn parse_operation_test() {
        let input = "\n  Operation: new = old * 19";

        assert_eq!(
            parse_operation(input),
            Ok((
                "",
                MonkeyOperation {
                    left: None,
                    operator: Operator::Times,
                    right: Some(19),
                }
            ))
        );

        let input = "\n  Operation: new = old + 3";

        assert_eq!(
            parse_operation(input),
            Ok((
                "",
                MonkeyOperation {
                    left: None,
                    operator: Operator::Plus,
                    right: Some(3),
                }
            ))
        );

        let input = "\n  Operation: new = old * old";

        assert_eq!(
            parse_operation(input),
            Ok((
                "",
                MonkeyOperation {
                    left: None,
                    operator: Operator::Times,
                    right: None,
                }
            ))
        );
    }

    #[test]
    fn test_parse_monkey_test() {
        let input = "
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

        assert_eq!(
            parse_monkey_test(input),
            Ok((
                "",
                MonkeyTest {
                    modulus: 23,
                    if_true: 2,
                    if_false: 3,
                }
            ))
        );

        let input = "
  Test: divisible by 17
    If true: throw to monkey 4
    If false: throw to monkey 1";

        assert_eq!(
            parse_monkey_test(input),
            Ok((
                "",
                MonkeyTest {
                    modulus: 17,
                    if_true: 4,
                    if_false: 1,
                }
            ))
        );
    }
}
