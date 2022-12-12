use crate::Map;
use nom::{
    character::complete::{anychar, newline},
    multi::{fill, separated_list1},
    IResult,
};

fn char_to_num(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 - 97,
        'S' => 0,
        'E' => 25,
        _ => panic!("bad char: {c:?}"),
    }
}

fn parse_char_grid<const W: usize, const H: usize>(input: &str) -> IResult<&str, [[char; W]; H]> {
    fn parse_row<const W: usize>(input: &str) -> IResult<&str, [char; W]> {
        let mut row: [char; W] = ['-'; W];
        let (input, _) = fill(anychar, &mut row)(input)?;
        Ok((input, row))
    }

    let mut grid: [[char; W]; H] = [['-'; W]; H];
    let (input, v) = separated_list1(newline, parse_row::<W>)(input)?;
    grid.copy_from_slice(&v[..]);

    Ok((input, grid))
}

fn find_pos<const W: usize, const H: usize>(grid: &[[char; W]; H], c: char) -> (usize, usize) {
    for x in 0..W {
        for y in 0..H {
            if grid[y][x] == c {
                return (y, x);
            }
        }
    }
    panic!();
}

pub fn parse_map<const W: usize, const H: usize>(input: &str) -> Map<W, H> {
    let char_grid = parse_char_grid::<W, H>(input).unwrap().1;
    let start = find_pos(&char_grid, 'S');
    let end = find_pos(&char_grid, 'E');
    let coords = char_grid.map(|row| row.map(char_to_num));

    Map { coords, start, end }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;

    #[test]
    fn parse_map_test() {
        let map = Map {
            coords: [
                [0, 0, 1, 16, 15, 14, 13, 12],
                [0, 1, 2, 17, 24, 23, 23, 11],
                [0, 2, 2, 18, 25, 25, 23, 10],
                [0, 2, 2, 19, 20, 21, 22, 9],
                [0, 1, 3, 4, 5, 6, 7, 8],
            ],
            start: (0, 0),
            end: (2, 5),
        };

        assert_eq!(parse_map(TEST_INPUT), map);
    }
}
