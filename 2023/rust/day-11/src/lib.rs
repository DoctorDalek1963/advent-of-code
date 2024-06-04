use lazy_static::lazy_static;
use nom::{character::complete::newline, multi::separated_list1, IResult};
use nom_regex::str::re_find;
use regex::Regex;

pub mod bin;

lazy_static! {
    static ref ROW: Regex = Regex::new(r"^[.#]+").unwrap();
}

/// A list of rows of bools. True if there's a galaxy there, false if there isn't.
pub type Universe = Vec<Vec<bool>>;

pub fn parse_universe(input: &str) -> IResult<&str, Universe> {
    fn parse_row(input: &str) -> IResult<&str, Vec<bool>> {
        let (input, row) = re_find(ROW.clone())(input)?;
        let galaxies = row
            .chars()
            .map(|c| match c {
                '.' => false,
                '#' => true,
                _ => unreachable!("We should only match . or #"),
            })
            .collect();
        Ok((input, galaxies))
    }

    separated_list1(newline, parse_row)(input)
}

pub fn rows_cols_to_copy(universe: &Universe) -> (Vec<usize>, Vec<usize>) {
    let rows_to_copy = universe
        .iter()
        .enumerate()
        .filter_map(|(idx, row)| row.iter().all(|x| !x).then_some(idx))
        .collect();

    let cols_to_copy = (0..universe.first().unwrap().len())
        .filter_map(|idx| universe.iter().all(|row| !row[idx]).then_some(idx))
        .collect();

    (rows_to_copy, cols_to_copy)
}

pub fn manhattan_distance((px, py): (usize, usize), (qx, qy): (usize, usize)) -> usize {
    px.abs_diff(qx) + py.abs_diff(qy)
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_universe_test() {
        assert_eq!(
            parse_universe(TEST_INPUT),
            Ok((
                "\n",
                vec![
                    [false, false, false, true, false, false, false, false, false, false]
                        .into_iter()
                        .collect(),
                    [false, false, false, false, false, false, false, true, false, false]
                        .into_iter()
                        .collect(),
                    [true, false, false, false, false, false, false, false, false, false]
                        .into_iter()
                        .collect(),
                    [false, false, false, false, false, false, false, false, false, false]
                        .into_iter()
                        .collect(),
                    [false, false, false, false, false, false, true, false, false, false]
                        .into_iter()
                        .collect(),
                    [false, true, false, false, false, false, false, false, false, false]
                        .into_iter()
                        .collect(),
                    [false, false, false, false, false, false, false, false, false, true]
                        .into_iter()
                        .collect(),
                    [false, false, false, false, false, false, false, false, false, false]
                        .into_iter()
                        .collect(),
                    [false, false, false, false, false, false, false, true, false, false]
                        .into_iter()
                        .collect(),
                    [true, false, false, false, true, false, false, false, false, false]
                        .into_iter()
                        .collect(),
                ]
            ))
        );
    }

    #[test]
    fn manhattan_distance_test() {
        assert_eq!(manhattan_distance((6, 1), (11, 5)), 9);
        assert_eq!(manhattan_distance((0, 4), (10, 9)), 15);
        assert_eq!(manhattan_distance((2, 0), (7, 12)), 17);
        assert_eq!(manhattan_distance((11, 0), (11, 5)), 5);
    }
}
