use std::collections::HashSet;

pub mod bin;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn next_position(self, position: Coord, bounds: &(usize, usize)) -> Option<Coord> {
        let Coord(x, y) = position;

        match self {
            Self::North => Some(Coord(x, y.checked_sub(1)?)),
            Self::East => {
                if x + 1 >= bounds.0 {
                    None
                } else {
                    Some(Coord(x + 1, y))
                }
            }
            Self::South => {
                if y + 1 >= bounds.1 {
                    None
                } else {
                    Some(Coord(x, y + 1))
                }
            }
            Self::West => Some(Coord(x.checked_sub(1)?, y)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coord(usize, usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Guard {
    position: Coord,
    direction: Direction,
}

impl Guard {
    pub fn take_step(self, bounds: &(usize, usize), obstacles: &[Coord]) -> Option<Self> {
        if obstacles.contains(&self.direction.next_position(self.position, bounds)?) {
            Some(Self {
                direction: self.direction.turn_right(),
                ..self
            })
        } else {
            Some(Self {
                position: self.direction.next_position(self.position, bounds)?,
                ..self
            })
        }
    }
}

/// Parse the map and return `(bounds, obstacle_positions, guard_position, guard_direction)`
pub fn parse_map(input: &str) -> ((usize, usize), Vec<Coord>, Guard) {
    let y_bound = input.lines().count();
    let x_bound = input.lines().next().unwrap().chars().count();

    let mut obstacle_positions = Vec::new();
    let mut guard_info: Option<Guard> = None;

    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            match c {
                '#' => obstacle_positions.push(Coord(x, y)),
                '^' => {
                    guard_info = Some(Guard {
                        position: Coord(x, y),
                        direction: Direction::North,
                    })
                }
                '>' => {
                    guard_info = Some(Guard {
                        position: Coord(x, y),
                        direction: Direction::East,
                    })
                }
                'v' => {
                    guard_info = Some(Guard {
                        position: Coord(x, y),
                        direction: Direction::South,
                    })
                }
                '<' => {
                    guard_info = Some(Guard {
                        position: Coord(x, y),
                        direction: Direction::West,
                    })
                }
                '.' => (),
                _ => panic!("Unexpected character {c:?} in map"),
            };
        }
    }

    let Some(guard) = guard_info else {
        panic!("Failed to find guard when parsing map");
    };

    ((x_bound, y_bound), obstacle_positions, guard)
}

/// Does the given map contain a loop for the guard?
pub fn map_has_loop(bounds: &(usize, usize), obstacles: &[Coord], mut guard: Guard) -> bool {
    let mut visited = HashSet::new();

    visited.insert(guard);
    while let Some(new_guard) = guard.take_step(bounds, obstacles) {
        guard = new_guard;

        if !visited.insert(guard) {
            return true;
        }
    }

    false
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
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
                (10, 10),
                vec![
                    Coord(4, 0),
                    Coord(9, 1),
                    Coord(2, 3),
                    Coord(7, 4),
                    Coord(1, 6),
                    Coord(8, 7),
                    Coord(0, 8),
                    Coord(6, 9),
                ],
                Guard {
                    position: Coord(4, 6),
                    direction: Direction::North,
                }
            )
        );
    }
}
