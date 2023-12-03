use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub mod bin;
mod parse;

pub use self::parse::parse_rules;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule<'s> {
    colour: &'s str,
    contains: Vec<(u16, &'s str)>,
}

/// How many colours of bag can contain the given colour?
pub fn how_many_can_contain<'s>(rules: &[Rule<'s>], colour: &'s str) -> usize {
    // Maps each colour to all the colours that can contain it
    let mut backwards_map: HashMap<&'s str, HashSet<&'s str>> = HashMap::new();

    for rule in rules {
        for &(_, contained) in &rule.contains {
            backwards_map
                .entry(contained)
                .and_modify(|containers| {
                    containers.insert(rule.colour);
                })
                .or_insert([rule.colour].into());
        }
    }

    // A map from a colour to `(explored, reachable)`, where `explored` is whether we've explored
    // all the values in the list, and `reachable` is every colour that can contain the key
    let mut new_map: HashMap<&'s str, (bool, HashSet<&'s str>)> = HashMap::new();
    new_map.insert(colour, (false, backwards_map.get(colour).unwrap().clone()));

    while new_map.values().any(|(explored, _)| !explored) {
        let mut explored_vec = vec![];

        let list: Vec<_> = new_map
            .iter()
            .filter(|(_key, (explored, _))| !explored)
            .map(|(a, (b, c))| (*a, (*b, c.clone())))
            .collect();
        for (colour, (_explored, reachable)) in list {
            for reach in reachable.iter() {
                if new_map.get(*reach).is_none() {
                    new_map.insert(
                        reach,
                        (
                            false,
                            backwards_map
                                .get(reach)
                                .map_or(HashSet::new(), |set| set.clone()),
                        ),
                    );
                }
            }
            explored_vec.push(colour);
        }

        for colour in explored_vec {
            new_map.get_mut(colour).unwrap().0 = true;
        }
    }

    new_map
        .into_values()
        .map(|(_, set)| set.into_iter())
        .flatten()
        .unique()
        .count()
}

/// How many bags are required inside a bag of the given colour?
pub fn how_many_inside<'s>(rules: &[Rule<'s>], colour: &'s str) -> usize {
    let rules_length = rules.len();
    let mut map: HashMap<&'s str, usize> = HashMap::new();

    while map.len() < rules_length {
        let mut temp_map = HashMap::new();

        // Find all resolveable rules (rules that depend entirely on colours that we already know)
        let resolveable_rules = rules
            .iter()
            .filter(|&rule| {
                rule.contains
                    .iter()
                    .all(|&(_, colour)| map.contains_key(colour))
            })
            .filter(|&rule| !map.contains_key(rule.colour));
        for rule in resolveable_rules {
            debug_assert!(!map.contains_key(rule.colour));
            let number = rule
                .contains
                .iter()
                .map(|&(num, colour)| num as usize + map.get(colour).unwrap() * num as usize)
                .sum();
            temp_map.insert(rule.colour, number);
        }

        map.extend(temp_map.drain());
    }

    *map.get(colour).unwrap()
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_rules_test() {
        assert_eq!(
            parse_rules(TEST_INPUT),
            Ok((
                "",
                vec![
                    Rule {
                        colour: "light red",
                        contains: vec![(1, "bright white",), (2, "muted yellow",),],
                    },
                    Rule {
                        colour: "dark orange",
                        contains: vec![(3, "bright white",), (4, "muted yellow",),],
                    },
                    Rule {
                        colour: "bright white",
                        contains: vec![(1, "shiny gold",),],
                    },
                    Rule {
                        colour: "muted yellow",
                        contains: vec![(2, "shiny gold",), (9, "faded blue",),],
                    },
                    Rule {
                        colour: "shiny gold",
                        contains: vec![(1, "dark olive",), (2, "vibrant plum",),],
                    },
                    Rule {
                        colour: "dark olive",
                        contains: vec![(3, "faded blue",), (4, "dotted black",),],
                    },
                    Rule {
                        colour: "vibrant plum",
                        contains: vec![(5, "faded blue",), (6, "dotted black",),],
                    },
                    Rule {
                        colour: "faded blue",
                        contains: vec![],
                    },
                    Rule {
                        colour: "dotted black",
                        contains: vec![],
                    },
                ]
            ))
        );
    }
}
