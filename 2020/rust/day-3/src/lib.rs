use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

pub mod bin;

/// A vec of rows. Each interior vec is a single row of the map.
pub type Map = Vec<Vec<bool>>;

fn parse_row(input: &str) -> IResult<&str, Vec<bool>> {
    map(many1(alt((tag("."), tag("#")))), |strings| {
        strings.into_iter().map(|s| s == "#").collect()
    })(input)
}

pub fn parse_map(input: &str) -> IResult<&str, Map> {
    separated_list1(newline, parse_row)(input)
}

pub fn count_collisions(map: &Map, slope: (usize, usize)) -> usize {
    let mut pos = (0, 0);
    let mut count = 0;
    let row_width = map[0].len();

    loop {
        let (across, down) = pos;
        if down >= map.len() {
            break;
        }

        if map[down][across % row_width] {
            count += 1;
        }

        pos = (across + slope.0, down + slope.1);
    }

    count
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_map_test() {
        assert_eq!(
            parse_map(TEST_INPUT),
            Ok((
                "\n",
                vec![
                    vec![false, false, true, true, false, false, false, false, false, false, false],
                    vec![true, false, false, false, true, false, false, false, true, false, false],
                    vec![false, true, false, false, false, false, true, false, false, true, false],
                    vec![false, false, true, false, true, false, false, false, true, false, true],
                    vec![false, true, false, false, false, true, true, false, false, true, false],
                    vec![false, false, true, false, true, true, false, false, false, false, false],
                    vec![false, true, false, true, false, true, false, false, false, false, true],
                    vec![false, true, false, false, false, false, false, false, false, false, true],
                    vec![true, false, true, true, false, false, false, true, false, false, false],
                    vec![true, false, false, false, true, true, false, false, false, false, true],
                    vec![false, true, false, false, true, false, false, false, true, false, true],
                ]
            ))
        );
    }
}
