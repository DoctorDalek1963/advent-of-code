pub mod bin;
mod parse;

use pathfinding::prelude::{dijkstra, dijkstra_all};

pub use self::parse::parse_map;

pub type Point = (usize, usize);

fn valid_neighbours<const W: usize, const H: usize>(
    point: Point,
    map: &Map<W, H>,
) -> Vec<(Point, usize)> {
    let y = point.0 as i32;
    let x = point.1 as i32;

    let points = vec![(y + 1, x), (y, x + 1), (y - 1, x), (y, x - 1)];

    points
        .iter()
        .filter_map(|&(y, x)| {
            if y >= 0 && x >= 0 && y < H as i32 && x < W as i32 {
                Some((y as usize, x as usize))
            } else {
                None
            }
        })
        .filter(|&p| map.get_height(p) <= map.get_height(point) + 1)
        .map(|p| (p, 1))
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Map<const W: usize, const H: usize> {
    // Indexed as [y][x] with (0, 0) in the top left
    coords: [[u8; W]; H],

    // Both (y, x) coords
    start: Point,
    end: Point,
}

impl<const W: usize, const H: usize> Map<W, H> {
    fn find_minimum_steps_from_point(&self, point: &Point) -> Option<usize> {
        let x = dijkstra(point, |&p| valid_neighbours(p, &self), |&p| p == self.end);
        x.map(|(_, p)| p)
    }

    pub fn find_minimum_steps_from_start(&self) -> usize {
        self.find_minimum_steps_from_point(&self.start).unwrap()
    }

    pub fn find_minimum_steps_from_anywhere(&self) -> usize {
        let dijkstra_result = dijkstra_all(&self.start, |&p| valid_neighbours(p, &self));
        let starting_coords: Vec<Point> = dijkstra_result
            .iter()
            .filter_map(|(&p, (_, _))| {
                if self.get_height(p) == 0 {
                    Some(p)
                } else {
                    None
                }
            })
            .collect();

        starting_coords
            .iter()
            .filter_map(|p| self.find_minimum_steps_from_point(p))
            .min()
            .unwrap()
    }

    #[inline]
    fn get_height(&self, (y, x): Point) -> u8 {
        self.coords[y][x]
    }
}

#[cfg(test)]
pub const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbours_test() {
        let map: Map<8, 5> = parse_map(TEST_INPUT);

        assert_eq!(
            valid_neighbours((0, 0), &map)
                .iter()
                .map(|&(p, _)| p)
                .collect::<Vec<Point>>(),
            vec![(1, 0), (0, 1)]
        );
        assert_eq!(
            valid_neighbours((2, 3), &map)
                .iter()
                .map(|&(p, _)| p)
                .collect::<Vec<Point>>(),
            vec![(3, 3), (1, 3), (2, 2)]
        );
    }
}
