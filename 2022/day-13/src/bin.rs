#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(s: &str) -> usize {
    let pairs = parse_packet_pairs(s).unwrap().1;
    pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, (left, right))| if left < right { Some(idx + 1) } else { None })
        .sum()
}

pub fn process_part2(s: &str) -> usize {
    use PacketComp::*;

    let divider_2 = List(vec![List(vec![Int(2)])]);
    let divider_6 = List(vec![List(vec![Int(6)])]);

    let mut packets: Vec<PacketComp> = parse_packet_pairs(s)
        .unwrap()
        .1
        .iter()
        .flat_map(|(a, b)| vec![a.clone(), b.clone()])
        .collect();

    packets.push(divider_2.clone());
    packets.push(divider_6.clone());

    packets.sort();

    let idx_2 = packets
        .iter()
        .enumerate()
        .find_map(|(idx, x)| if *x == divider_2 { Some(idx + 1) } else { None })
        .unwrap();
    let idx_6 = packets
        .iter()
        .enumerate()
        .find_map(|(idx, x)| if *x == divider_6 { Some(idx + 1) } else { None })
        .unwrap();

    idx_2 * idx_6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 13);
        assert_eq!(process_part1(&get_input()), 6072);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_part2(TEST_INPUT), 140);
        assert_eq!(process_part2(&get_input()), 22184);
    }
}
