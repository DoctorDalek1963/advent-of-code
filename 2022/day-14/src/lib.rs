pub mod bin;

use nom::{
    bytes::complete::tag, character::complete, multi::separated_list1, sequence::separated_pair,
    IResult,
};
use std::{collections::HashSet, fmt::Debug};

pub type Point = (u32, u32);

#[derive(Clone, Eq, PartialEq)]
pub struct RockMap {
    rock_points: HashSet<Point>,
    sand_points: HashSet<Point>,
}

enum NextPointError {
    Blocked,
    SourceBlocked,
    FallenIntoAbyss,
}

pub fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    separated_list1(
        tag("\n"),
        separated_list1(
            tag(" -> "),
            separated_pair(complete::u32, tag(","), complete::u32),
        ),
    )(input)
}

fn points_in_straight_line(p: Point, q: Point) -> Vec<Point> {
    let (px, py) = p;
    let (qx, qy) = q;

    if px == qx {
        let range = {
            let range = py..=qy;
            if range.is_empty() {
                qy..=py
            } else {
                range
            }
        };

        range.into_iter().map(|y| (px, y)).collect()
    } else if py == qy {
        let range = {
            let range = px..=qx;
            if range.is_empty() {
                qx..=px
            } else {
                range
            }
        };

        range.into_iter().map(|x| (x, py)).collect()
    } else {
        panic!("This function doens't understand non-straight lines");
    }
}

impl Debug for RockMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "RockMap {{")?;
        let points_iter = self
            .rock_points
            .iter()
            .map(|(x, _)| x)
            .chain(self.sand_points.iter().map(|(x, _)| x));
        let x_range = (points_iter.clone().min().unwrap() - 2)..=(points_iter.max().unwrap() + 2);

        for y in 0..self.max_rock_y() + 1 {
            write!(f, "  ")?;

            for x in x_range.clone() {
                let point = &(x, y);

                if self.rock_points.contains(point) && self.sand_points.contains(point) {
                    write!(f, "@")?;
                } else if self.rock_points.contains(point) {
                    write!(f, "#")?;
                } else if self.sand_points.contains(point) {
                    write!(f, "o")?;
                } else {
                    write!(f, " ")?;
                };
            }

            writeln!(f)?;
        }

        write!(f, "  ")?;
        for _ in x_range.clone() {
            write!(f, "=")?;
        }
        writeln!(f)?;

        writeln!(f, "}}")?;

        Ok(())
    }
}

impl RockMap {
    pub fn from_lines(lines: Vec<Vec<Point>>) -> Self {
        let mut rock_points = HashSet::new();

        for points in lines {
            for i in 0..(points.len() - 1) {
                rock_points.extend(points_in_straight_line(points[i], points[i + 1]));
            }
        }

        Self {
            rock_points,
            sand_points: HashSet::new(),
        }
    }

    fn is_occupied(&self, point: &Point) -> bool {
        self.rock_points.contains(point) || self.sand_points.contains(point)
    }

    fn max_rock_y(&self) -> &u32 {
        self.rock_points.iter().map(|(_, y)| y).max().unwrap()
    }

    fn is_in_abyss(&self, (_, y): &Point) -> bool {
        y > self.max_rock_y()
    }

    fn get_next_point(&self, point: &Point, with_abyss: bool) -> Result<Point, NextPointError> {
        let (x, y) = *point;

        if !with_abyss && y == *self.max_rock_y() + 1 {
            return Err(NextPointError::Blocked);
        }

        if with_abyss && self.is_in_abyss(point) {
            return Err(NextPointError::FallenIntoAbyss);
        }

        let next = (x, y + 1);
        if !self.is_occupied(&next) {
            return Ok(next);
        }

        let next = (x - 1, y + 1);
        if !self.is_occupied(&next) {
            return Ok(next);
        }

        let next = (x + 1, y + 1);
        if !self.is_occupied(&next) {
            return Ok(next);
        }

        if *point == (500, 0) {
            Err(NextPointError::SourceBlocked)
        } else {
            Err(NextPointError::Blocked)
        }
    }

    pub fn drop_sand_with_abyss(&mut self, with_abyss: bool) -> Option<Point> {
        let mut sand_point = (500, 0);
        loop {
            match self.get_next_point(&sand_point, with_abyss) {
                Ok(p) => {
                    sand_point = p;
                }
                Err(NextPointError::Blocked) => {
                    self.sand_points.insert(sand_point);
                    return Some(sand_point);
                }
                Err(NextPointError::SourceBlocked) => {
                    self.sand_points.insert((500, 0));
                    return None;
                }
                Err(NextPointError::FallenIntoAbyss) => return None,
            };
        }
    }
}

#[cfg(test)]
pub const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
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
                    vec![(498, 4), (498, 6), (496, 6)],
                    vec![(503, 4), (502, 4), (502, 9), (494, 9)]
                ]
            ))
        );
    }

    #[test]
    fn rockmap_from_lines_test() {
        #[rustfmt::skip]
        let map = RockMap {
            rock_points: HashSet::from([
                (498, 4), (498, 5), (498, 6), (497, 6), (496, 6),
                (503, 4), (502, 4), (502, 5), (502, 6), (502, 7),
                (502, 8), (502, 9), (501, 9), (500, 9), (499, 9),
                (498, 9), (497, 9), (496, 9), (495, 9), (494, 9),
            ]),
            sand_points: HashSet::new(),
        };

        assert_eq!(RockMap::from_lines(parse_lines(TEST_INPUT).unwrap().1), map);
    }
}
