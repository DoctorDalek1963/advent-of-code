use std::collections::{HashMap, HashSet};

pub mod bin;

pub type Coord = (i32, i32);

pub fn parse_map(input: &str) -> ((usize, usize), HashMap<char, Vec<Coord>>) {
    let y_bound = input.lines().count();
    let x_bound = input.lines().next().unwrap().chars().count();

    let mut symbol_map = HashMap::new();

    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if matches!(c, '0'..='9' | 'a'..='z' | 'A'..='Z') {
                symbol_map
                    .entry(c)
                    .and_modify(|v: &mut Vec<_>| v.push((x as i32, y as i32)))
                    .or_insert(vec![(x as i32, y as i32)]);
            }
        }
    }

    ((x_bound, y_bound), symbol_map)
}

pub fn find_all_simple_antinodes_in_bounds_for_one_symbol(
    bounds: (usize, usize),
    coords: &[Coord],
) -> HashSet<Coord> {
    let mut antinodes = HashSet::new();

    for (idx, &coord_a) in coords.iter().enumerate() {
        for &coord_b in coords.iter().skip(idx + 1) {
            let (ax, ay) = coord_a;
            let (bx, by) = coord_b;
            let a_to_b = (bx - ax, by - ay);

            let node_a = (ax - a_to_b.0, ay - a_to_b.1);
            if (0..bounds.0 as i32).contains(&node_a.0) && (0..bounds.1 as i32).contains(&node_a.1)
            {
                antinodes.insert(node_a);
            }

            let node_b = (bx + a_to_b.0, by + a_to_b.1);
            if (0..bounds.0 as i32).contains(&node_b.0) && (0..bounds.1 as i32).contains(&node_b.1)
            {
                antinodes.insert(node_b);
            }
        }
    }

    antinodes
}

pub fn find_all_complex_antinodes_in_bounds_for_one_symbol(
    bounds: (usize, usize),
    coords: &[Coord],
) -> HashSet<Coord> {
    let mut antinodes = HashSet::new();

    for (idx, &coord_a) in coords.iter().enumerate() {
        for &coord_b in coords.iter().skip(idx + 1) {
            let (ax, ay) = coord_a;
            let (bx, by) = coord_b;
            let a_to_b = (bx - ax, by - ay);

            antinodes.insert(coord_a);
            antinodes.insert(coord_b);

            let mut i = 1;

            loop {
                let node = (ax - i * a_to_b.0, ay - i * a_to_b.1);
                if (0..bounds.0 as i32).contains(&node.0) && (0..bounds.1 as i32).contains(&node.1)
                {
                    antinodes.insert(node);
                    i += 1;
                } else {
                    break;
                }
            }

            i = 1;

            loop {
                let node = (bx + i * a_to_b.0, by + i * a_to_b.1);
                if (0..bounds.0 as i32).contains(&node.0) && (0..bounds.1 as i32).contains(&node.1)
                {
                    antinodes.insert(node);
                    i += 1;
                } else {
                    break;
                }
            }
        }
    }

    antinodes
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_map() {
        assert_eq!(
            parse_map(TEST_INPUT),
            (
                (12, 12),
                HashMap::from([
                    ('0', vec![(8, 1), (5, 2), (7, 3), (4, 4)]),
                    ('A', vec![(6, 5), (8, 8), (9, 9)]),
                ])
            )
        )
    }
}
