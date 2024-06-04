#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;
use num::Integer;
use std::collections::HashMap;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> usize {
    let (directions, nodes) = parse_directions_and_nodes(input).unwrap().1;
    let node_map: HashMap<&str, (&str, &str)> = nodes
        .into_iter()
        .map(
            |Node {
                 this_name,
                 connected,
             }| (this_name, connected),
        )
        .collect();

    let mut current_node = "AAA";
    let mut steps = 0;

    while current_node != "ZZZ" {
        let dir = directions[steps % directions.len()];
        let (left, right) = node_map.get(current_node).unwrap();
        current_node = match dir {
            Direction::Left => left,
            Direction::Right => right,
        };
        steps += 1;
    }

    steps
}

pub fn process_part2(input: &str) -> usize {
    let (directions, nodes) = parse_directions_and_nodes(input).unwrap().1;
    let node_map: HashMap<&str, (&str, &str)> = nodes
        .iter()
        .map(
            |&Node {
                 this_name,
                 connected,
             }| (this_name, connected),
        )
        .collect();

    let mut current_nodes_and_loop_lengths: Vec<_> = nodes
        .into_iter()
        .filter_map(|Node { this_name, .. }| this_name.ends_with("A").then_some((this_name, None)))
        .collect();
    let mut steps = 0;

    while !current_nodes_and_loop_lengths
        .iter()
        .all(|&(node_name, _)| node_name.ends_with("Z"))
    {
        let dir = directions[steps % directions.len()];
        for (node_name, loop_length) in current_nodes_and_loop_lengths.iter_mut() {
            let (left, right) = node_map.get(node_name).unwrap();
            *node_name = match dir {
                Direction::Left => left,
                Direction::Right => right,
            };

            if loop_length.is_none() && node_name.ends_with("Z") {
                *loop_length = Some(steps);
            }
        }

        if current_nodes_and_loop_lengths
            .iter()
            .all(|(_, loop_length)| loop_length.is_some())
        {
            return current_nodes_and_loop_lengths
                .into_iter()
                .map(|(_, loop_length)| loop_length.unwrap() + 1)
                .fold(1, |acc, loop_length| acc.lcm(&loop_length));
        }

        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        const OTHER_TEST_INPUT: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

        assert_eq!(process_part1(TEST_INPUT), 2);
        assert_eq!(process_part1(OTHER_TEST_INPUT), 6);
        assert_eq!(process_part1(&get_input()), 20_513);
    }

    #[test]
    fn process_part2_test() {
        const OTHER_TEST_INPUT: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;

        assert_eq!(process_part2(OTHER_TEST_INPUT), 6);
        assert_eq!(process_part2(&get_input()), 15_995_167_053_923);
    }
}
