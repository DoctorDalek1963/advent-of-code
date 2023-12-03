use crate::Rule;
use lazy_static::lazy_static;
use nom::{
    branch::alt, bytes::complete::tag, character::complete, multi::separated_list1, IResult, Parser,
};
use nom_regex::str::re_capture;
use regex::Regex;

lazy_static! {
    static ref RULE_START: Regex = Regex::new(r"(.*?)( bags contain )").unwrap();
    static ref CONTAINED_BAG_COLOUR: Regex = Regex::new(r"(.*?)( bags?)").unwrap();
}

pub fn parse_rules<'s>(input: &'s str) -> IResult<&'s str, Vec<Rule<'s>>> {
    let (input, rules) = separated_list1(tag(".\n"), parse_rule)(input)?;
    let (input, _) = tag(".\n")(input)?;
    Ok((input, rules))
}

fn parse_rule<'s>(input: &'s str) -> IResult<&'s str, Rule<'s>> {
    let (input, colour_match) = re_capture(RULE_START.clone())(input)?;
    let colour = colour_match[1];
    let (input, contains) = separated_list1(tag(", "), parse_contained_bag)(input)?;
    Ok((
        input,
        Rule {
            colour,
            contains: contains.into_iter().filter_map(|x| x).collect(),
        },
    ))
}

fn parse_contained_bag<'s>(input: &'s str) -> IResult<&'s str, Option<(u16, &'s str)>> {
    fn with_number<'s>(input: &'s str) -> IResult<&'s str, (u16, &'s str)> {
        let (input, number) = complete::u16(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, colour_match) = re_capture(CONTAINED_BAG_COLOUR.clone())(input)?;
        let colour = colour_match[1];
        Ok((input, (number, colour)))
    }

    fn without_number<'s>(input: &'s str) -> IResult<&'s str, ()> {
        let (input, _) = tag("no other bags")(input)?;
        Ok((input, ()))
    }

    alt((
        with_number.map(|pair| Some(pair)),
        without_number.map(|_| None),
    ))(input)
}
