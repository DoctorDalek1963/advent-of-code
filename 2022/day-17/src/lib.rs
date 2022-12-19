pub mod bin;

use std::{
    cmp::Ordering, collections::BTreeSet, iter::Cycle, ops::Add, slice::Iter, vec::IntoIter,
};

/// All points are (x, y) with the origin being in the bottom left.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point(u64, u64);

impl Add<Self> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Point(px, py) = self;
        let Point(qx, qy) = rhs;
        Point(px + qx, py + qy)
    }
}

impl Add<Vec<Self>> for Point {
    type Output = Vec<Self>;

    fn add(self, rhs: Vec<Self>) -> Self::Output {
        rhs.iter().map(|&p| self + p).collect()
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;

        match self.1.cmp(&other.1) {
            Less => Less,
            Equal => self.0.cmp(&other.0),
            Greater => Greater,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

pub fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        })
        .collect()
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum RockShape {
    Horizontal,
    Plus,
    LShape,
    Vertical,
    Square,
}

impl RockShape {
    #[inline]
    fn next_shape(self) -> Self {
        use RockShape::*;

        match self {
            Horizontal => Plus,
            Plus => LShape,
            LShape => Vertical,
            Vertical => Square,
            Square => Horizontal,
        }
    }

    /// Get a list of points that the rock occupies, relative to (0, 0) being the bottom left
    /// corner of the rock's bounding box.
    fn points(&self) -> Vec<Point> {
        use RockShape::*;

        match self {
            Horizontal => vec![Point(0, 0), Point(1, 0), Point(2, 0), Point(3, 0)],
            Plus => vec![
                Point(1, 0),
                Point(0, 1),
                Point(1, 1),
                Point(2, 1),
                Point(1, 2),
            ],
            LShape => vec![
                Point(0, 0),
                Point(1, 0),
                Point(2, 0),
                Point(2, 1),
                Point(2, 2),
            ],
            Vertical => vec![Point(0, 0), Point(0, 1), Point(0, 2), Point(0, 3)],
            Square => vec![Point(0, 0), Point(1, 0), Point(0, 1), Point(1, 1)],
        }
    }
}

pub struct Chamber {
    occupied_cells: BTreeSet<Point>,
    current_rock_shape: RockShape,
    directions: Cycle<IntoIter<Direction>>,
}

impl Chamber {
    pub fn new(directions: Vec<Direction>) -> Self {
        Self {
            occupied_cells: BTreeSet::new(),
            current_rock_shape: RockShape::Horizontal,
            directions: directions.into_iter().cycle(),
        }
    }

    pub fn max_height(&self) -> Option<u64> {
        self.occupied_cells.last().map(|&Point(_, y)| y)
    }

    fn get_start_point(&self) -> Point {
        Point(2, self.max_height().map(|n| n + 4).unwrap_or(3))
    }

    pub fn drop_rock(&mut self) {
        let mut rock_points = self.get_start_point() + self.current_rock_shape.points();

        loop {
            let next_points = match self.directions.next().unwrap() {
                Direction::Left => {
                    if rock_points.iter().all(|&Point(x, _)| x > 0) {
                        rock_points
                            .iter()
                            .map(|&Point(x, y)| Point(x - 1, y))
                            .collect()
                    } else {
                        rock_points.clone()
                    }
                }
                Direction::Right => {
                    if rock_points.iter().all(|&Point(x, _)| x < 6) {
                        rock_points
                            .iter()
                            .map(|&Point(x, y)| Point(x + 1, y))
                            .collect()
                    } else {
                        rock_points.clone()
                    }
                }
            };
            if !next_points.iter().any(|p| self.occupied_cells.contains(p)) {
                rock_points = next_points;
            }

            if rock_points.iter().any(|p| p.1 == 0) {
                break;
            }

            let next_points: Vec<Point> = rock_points
                .iter()
                .map(|&Point(x, y)| Point(x, y - 1))
                .collect();

            if next_points.iter().any(|p| self.occupied_cells.contains(p)) {
                break;
            } else {
                rock_points = next_points;
            }
        }

        self.occupied_cells.extend(rock_points);
        self.current_rock_shape = self.current_rock_shape.next_shape();
    }
}
