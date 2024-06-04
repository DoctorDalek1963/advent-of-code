pub mod bin;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    multi::separated_list1,
    IResult,
};
use pathfinding::prelude::dijkstra_all;
use std::collections::HashMap;

pub fn parse_cave_system<'s>(input: &'s str) -> IResult<&'s str, CaveSystem<'s>> {
    fn parse_valve_tuple<'s>(input: &'s str) -> IResult<&'s str, (&'s str, (u32, Vec<&'s str>))> {
        let (input, _) = tag("Valve ")(input)?;
        let (input, name) = alpha1(input)?;
        let (input, _) = tag(" has flow rate=")(input)?;
        let (input, rate) = complete::u32(input)?;
        let (input, _) = alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        ))(input)?;
        let (input, other_valves) = separated_list1(tag(", "), alpha1)(input)?;

        Ok((input, (name, (rate, other_valves))))
    }

    let (input, valve_tuples) = separated_list1(tag("\n"), parse_valve_tuple)(input)?;

    let mut valves: HashMap<&str, (u32, Vec<&str>)> = HashMap::new();
    for (name, data) in valve_tuples {
        valves.insert(name, data);
    }

    Ok((input, CaveSystem::new(valves)))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaveSystem<'s> {
    valves: HashMap<&'s str, (u32, Vec<&'s str>)>,
    total_rate: u32,
    current_rate: u32,
}

impl<'s> CaveSystem<'s> {
    fn new(valves: HashMap<&'s str, (u32, Vec<&'s str>)>) -> Self {
        let total_rate = valves.values().map(|&(n, _)| n).sum();
        Self {
            valves,
            total_rate,
            current_rate: 0,
        }
    }

    pub fn find_max_pressure(&self) -> u32 {
        let x = dijkstra_all(&"AA", |x| {
            self.valves.get(x).unwrap().1.iter().map(|&name| (name, 1))
        });
        dbg!(x);
        todo!()
    }
}

#[cfg(test)]
pub const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cave_system_test() {
        let valves = HashMap::from([
            ("AA", (0, vec!["DD", "II", "BB"])),
            ("BB", (13, vec!["CC", "AA"])),
            ("CC", (2, vec!["DD", "BB"])),
            ("DD", (20, vec!["CC", "AA", "EE"])),
            ("EE", (3, vec!["FF", "DD"])),
            ("FF", (0, vec!["EE", "GG"])),
            ("GG", (0, vec!["FF", "HH"])),
            ("HH", (22, vec!["GG"])),
            ("II", (0, vec!["AA", "JJ"])),
            ("JJ", (21, vec!["II"])),
        ]);
        assert_eq!(
            parse_cave_system(TEST_INPUT),
            Ok(("\n", CaveSystem::new(valves)))
        );
    }
}
