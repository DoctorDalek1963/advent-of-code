use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, one_of},
    multi::separated_list1,
    IResult,
};
use std::collections::HashSet;

pub mod bin;

type Point = (i32, i32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct BoardState<const N: usize> {
    head_pos: Point,
    tail: [Point; N],
    tail_visited_points: HashSet<Point>,
}

impl<const N: usize> Default for BoardState<N> {
    fn default() -> Self {
        let mut tail_visited_points = HashSet::new();
        tail_visited_points.insert((0, 0));

        Self {
            head_pos: (0, 0),
            tail: [(0, 0); N],
            tail_visited_points,
        }
    }
}

impl<const N: usize> BoardState<N> {
    fn move_direction(&mut self, direction: Direction) {
        use Direction::*;

        let (x, y) = self.head_pos;
        self.head_pos = match direction {
            Up => (x, y + 1),
            Down => (x, y - 1),
            Left => (x - 1, y),
            Right => (x + 1, y),
        };

        self.tail[0] = find_new_point(self.head_pos, self.tail[0]);

        for i in 1..N {
            self.tail[i] = find_new_point(self.tail[i - 1], self.tail[i]);
        }

        self.tail_visited_points.insert(self.tail[N - 1]);
    }

    pub fn move_many_directions(&mut self, directions: Vec<(Direction, u8)>) {
        for (direction, num) in directions {
            for _ in 0..num {
                self.move_direction(direction);
            }
        }
    }
}

fn find_new_point(ahead: Point, current_point: Point) -> Point {
    if chebyshev_distance(ahead, current_point) > 1 {
        get_surrounding_points(current_point)
            .iter()
            .copied()
            .min_by_key(|&p| euclidean_distance(p, ahead))
            .unwrap()
    } else {
        current_point
    }
}

fn chebyshev_distance(p: Point, q: Point) -> u32 {
    i32::max(i32::abs(p.0 - q.0), i32::abs(p.1 - q.1)) as u32
}

fn euclidean_distance(p: Point, q: Point) -> u32 {
    let dx = (p.0 - q.0) as f32;
    let dy = (p.1 - q.1) as f32;
    (1_000_000.0 * f32::sqrt(dx * dx + dy * dy)) as u32
}

fn get_surrounding_points((x, y): Point) -> [Point; 8] {
    [
        (x + 1, y),
        (x + 1, y + 1),
        (x, y + 1),
        (x - 1, y + 1),
        (x - 1, y),
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
    ]
}

fn parse_directions(input: &str) -> IResult<&str, Vec<(Direction, u8)>> {
    fn parse_direction(input: &str) -> IResult<&str, (Direction, u8)> {
        use Direction::*;

        let (input, direction) = one_of("UDLR")(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, num) = complete::u8(input)?;

        let dir = match direction {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => unreachable!(),
        };

        Ok((input, (dir, num)))
    }

    separated_list1(newline, parse_direction)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn parse_directions_test() {
        use Direction::*;

        let directions = vec![
            (Right, 4),
            (Up, 4),
            (Left, 3),
            (Down, 1),
            (Right, 4),
            (Down, 1),
            (Left, 5),
            (Right, 2),
        ];

        assert_eq!(parse_directions(INPUT), Ok(("\n", directions)));
    }

    #[test]
    fn chebyshev_distance_test() {
        assert_eq!(chebyshev_distance((0, 1), (1, 2)), 1);
        assert_eq!(chebyshev_distance((0, 1), (1, 1)), 1);
        assert_eq!(chebyshev_distance((1, 2), (1, 2)), 0);
        assert_eq!(chebyshev_distance((0, 0), (1, 2)), 2);
        assert_eq!(chebyshev_distance((0, 1), (-3, -1)), 3);
        assert_eq!(chebyshev_distance((0, 1), (-3, 0)), 3);
    }

    #[test]
    fn get_surrounding_points_test() {
        assert_eq!(
            get_surrounding_points((0, 0)),
            [
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1)
            ]
        );
    }

    #[test]
    fn move_direction_one_tail_test() {
        use Direction::*;

        let mut board = BoardState::<1>::default();

        assert_eq!(board.head_pos, (0, 0));
        assert_eq!(board.tail[0], (0, 0));

        board.move_direction(Right);
        assert_eq!(board.head_pos, (1, 0));
        assert_eq!(board.tail[0], (0, 0));

        board.move_direction(Right);
        assert_eq!(board.head_pos, (2, 0));
        assert_eq!(board.tail[0], (1, 0));

        board.move_direction(Right);
        assert_eq!(board.head_pos, (3, 0));
        assert_eq!(board.tail[0], (2, 0));

        board.move_direction(Right);
        assert_eq!(board.head_pos, (4, 0));
        assert_eq!(board.tail[0], (3, 0));

        board.move_direction(Up);
        assert_eq!(board.head_pos, (4, 1));
        assert_eq!(board.tail[0], (3, 0));

        board.move_direction(Up);
        assert_eq!(board.head_pos, (4, 2));
        assert_eq!(board.tail[0], (4, 1));

        board.move_direction(Up);
        assert_eq!(board.head_pos, (4, 3));
        assert_eq!(board.tail[0], (4, 2));

        board.move_direction(Up);
        assert_eq!(board.head_pos, (4, 4));
        assert_eq!(board.tail[0], (4, 3));

        board.move_direction(Left);
        assert_eq!(board.head_pos, (3, 4));
        assert_eq!(board.tail[0], (4, 3));

        board.move_direction(Left);
        assert_eq!(board.head_pos, (2, 4));
        assert_eq!(board.tail[0], (3, 4));
    }
}
