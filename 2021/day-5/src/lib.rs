use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub mod bin;

pub type Point = (u32, u32);

pub fn parse_lines(input: &str) -> IResult<&str, Vec<(Point, Point)>> {
    fn parse_coord(input: &str) -> IResult<&str, Point> {
        separated_pair(complete::u32, tag(","), complete::u32)(input)
    }

    separated_list1(
        newline,
        separated_pair(parse_coord, tag(" -> "), parse_coord),
    )(input)
}

pub fn get_points_on_straight_line((x1, y1): Point, (x2, y2): Point) -> Option<Vec<Point>> {
    if x1 == x2 {
        Some(
            (if y2 >= y1 { y1..=y2 } else { y2..=y1 })
                .into_iter()
                .map(|y| (x1, y))
                .collect(),
        )
    } else if y1 == y2 {
        Some(
            (if x2 >= x1 { x1..=x2 } else { x2..=x1 })
                .into_iter()
                .map(|x| (x, y1))
                .collect(),
        )
    } else {
        None
    }
}

pub fn get_points_on_straight_or_diagonal_lines(
    (x1, y1): Point,
    (x2, y2): Point,
) -> Option<Vec<Point>> {
    Some(
        get_points_on_straight_line((x1, y1), (x2, y2)).unwrap_or_else(|| {
            let mut v = vec![];
            let mut x = x1;
            let mut y = y1;

            // ..3
            // .2.
            // 1..
            if x1 < x2 && y1 < y2 {
                while x <= x2 && y <= y2 {
                    v.push((x, y));
                    x += 1;
                    y += 1;
                }

            // 1..
            // .2.
            // ..3
            } else if x1 < x2 && y1 > y2 {
                while x <= x2 && y >= y2 {
                    v.push((x, y));
                    x += 1;
                    y -= 1;
                }

            // 3..
            // .2.
            // ..1
            } else if x1 > x2 && y1 < y2 {
                while x >= x2 && y <= y2 {
                    v.push((x, y));
                    x -= 1;
                    y += 1;
                }

            // ..1
            // .2.
            // 3..
            } else if x1 > x2 && y1 > y2 {
                while x >= x2 && y >= y2 {
                    v.push((x, y));
                    x -= 1;
                    y -= 1;
                }
            } else {
                unreachable!();
            }

            v
        }),
    )
}

#[cfg(test)]
pub const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_lines_test() {
        assert_eq!(
            parse_lines(TEST_INPUT),
            Ok((
                "\n",
                vec![
                    ((0, 9), (5, 9)),
                    ((8, 0), (0, 8)),
                    ((9, 4), (3, 4)),
                    ((2, 2), (2, 1)),
                    ((7, 0), (7, 4)),
                    ((6, 4), (2, 0)),
                    ((0, 9), (2, 9)),
                    ((3, 4), (1, 4)),
                    ((0, 0), (8, 8)),
                    ((5, 5), (8, 2)),
                ]
            ))
        );
    }

    #[test]
    fn get_points_on_straight_line_test() {
        assert_eq!(
            get_points_on_straight_line((0, 9), (5, 9)),
            Some(vec![(0, 9), (1, 9), (2, 9), (3, 9), (4, 9), (5, 9)])
        );
        assert_eq!(
            get_points_on_straight_line((7, 0), (7, 4)),
            Some(vec![(7, 0), (7, 1), (7, 2), (7, 3), (7, 4)])
        );
        assert_eq!(get_points_on_straight_line((5, 5), (8, 2)), None);
    }

    #[test]
    fn get_points_on_straight_or_diagonal_lines_test() {
        assert_eq!(
            get_points_on_straight_or_diagonal_lines((1, 1), (3, 3)),
            Some(vec![(1, 1), (2, 2), (3, 3)])
        );
        assert_eq!(
            get_points_on_straight_or_diagonal_lines((9, 7), (7, 9)),
            Some(vec![(9, 7), (8, 8), (7, 9)])
        );
    }
}
