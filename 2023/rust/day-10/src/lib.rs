use std::collections::HashMap;

pub mod bin;

pub type Point = (usize, usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    Ground,
    StartingPosition,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl TryFrom<char> for TileType {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::StartingPosition),
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            'F' => Ok(Self::SouthEast),
            '7' => Ok(Self::SouthWest),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connection {
    Ground,
    StartingPosition([Point; 4]),
    Conn(ConnectionType, [Point; 2]),
}

impl From<(Point, TileType)> for Connection {
    fn from(((px, py), tile_type): (Point, TileType)) -> Self {
        match tile_type {
            TileType::Ground => Self::Ground,
            TileType::StartingPosition => {
                Self::StartingPosition([(px - 1, py), (px + 1, py), (px, py - 1), (px, py + 1)])
            }
            TileType::Vertical => {
                Self::Conn(ConnectionType::Vertical, [(px, py - 1), (px, py + 1)])
            }
            TileType::Horizontal => {
                Self::Conn(ConnectionType::Horizontal, [(px - 1, py), (px + 1, py)])
            }
            TileType::NorthEast => {
                Self::Conn(ConnectionType::NorthEast, [(px, py - 1), (px + 1, py)])
            }
            TileType::NorthWest => {
                Self::Conn(ConnectionType::NorthWest, [(px, py - 1), (px - 1, py)])
            }
            TileType::SouthEast => {
                Self::Conn(ConnectionType::SouthEast, [(px, py + 1), (px + 1, py)])
            }
            TileType::SouthWest => {
                Self::Conn(ConnectionType::SouthWest, [(px, py + 1), (px - 1, py)])
            }
        }
    }
}

pub fn parse_map(input: &str) -> Vec<Vec<Connection>> {
    input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, c)| ((x + 1, y + 1), TileType::try_from(c).unwrap()).into())
                .collect()
        })
        .collect()
}

pub fn get_coords_map(connections: Vec<Vec<Connection>>) -> HashMap<Point, Connection> {
    connections
        .into_iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(move |(x, connection)| ((x + 1, y + 1), connection))
        })
        .flatten()
        .collect()
}

#[cfg(test)]
pub const TEST_INPUT_SMALL_ALONE: &str = r#".....
.S-7.
.|.|.
.L-J.
.....
"#;

#[cfg(test)]
pub const TEST_INPUT_SMALL_CLUTTERED: &str = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"#;

#[cfg(test)]
pub const TEST_INPUT_LARGE_ALONE: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;

#[cfg(test)]
pub const TEST_INPUT_LARGE_CLUTTERED: &str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_map_test() {
        use Connection::*;
        use ConnectionType::*;

        assert_eq!(
            parse_map(TEST_INPUT_SMALL_ALONE),
            vec![
                vec![Ground, Ground, Ground, Ground, Ground],
                vec![
                    Ground,
                    StartingPosition([(1, 2), (3, 2), (2, 1), (2, 3)]),
                    Conn(Horizontal, [(2, 2), (4, 2)]),
                    Conn(SouthWest, [(4, 3), (3, 2)]),
                    Ground,
                ],
                vec![
                    Ground,
                    Conn(Vertical, [(2, 2), (2, 4)]),
                    Ground,
                    Conn(Vertical, [(4, 2), (4, 4)]),
                    Ground,
                ],
                vec![
                    Ground,
                    Conn(NorthEast, [(2, 3), (3, 4)]),
                    Conn(Horizontal, [(2, 4), (4, 4)]),
                    Conn(NorthWest, [(4, 3), (3, 4)]),
                    Ground,
                ],
                vec![Ground, Ground, Ground, Ground, Ground],
            ]
        );

        assert_eq!(
            parse_map(TEST_INPUT_SMALL_CLUTTERED),
            vec![
                vec![
                    Conn(Horizontal, [(0, 1), (2, 1)]),
                    Conn(NorthEast, [(2, 0), (3, 1)]),
                    Conn(Vertical, [(3, 0), (3, 2)]),
                    Conn(SouthEast, [(4, 2), (5, 1)]),
                    Conn(SouthWest, [(5, 2), (4, 1)]),
                ],
                vec![
                    Conn(SouthWest, [(1, 3), (0, 2)]),
                    StartingPosition([(1, 2), (3, 2), (2, 1), (2, 3)]),
                    Conn(Horizontal, [(2, 2), (4, 2)]),
                    Conn(SouthWest, [(4, 3), (3, 2)]),
                    Conn(Vertical, [(5, 1), (5, 3)]),
                ],
                vec![
                    Conn(NorthEast, [(1, 2), (2, 3)]),
                    Conn(Vertical, [(2, 2), (2, 4)]),
                    Conn(SouthWest, [(3, 4), (2, 3)]),
                    Conn(Vertical, [(4, 2), (4, 4)]),
                    Conn(Vertical, [(5, 2), (5, 4)]),
                ],
                vec![
                    Conn(Horizontal, [(0, 4), (2, 4)]),
                    Conn(NorthEast, [(2, 3), (3, 4)]),
                    Conn(Horizontal, [(2, 4), (4, 4)]),
                    Conn(NorthWest, [(4, 3), (3, 4)]),
                    Conn(Vertical, [(5, 3), (5, 5)]),
                ],
                vec![
                    Conn(NorthEast, [(1, 4), (2, 5)]),
                    Conn(Vertical, [(2, 4), (2, 6)]),
                    Conn(Horizontal, [(2, 5), (4, 5)]),
                    Conn(NorthWest, [(4, 4), (3, 5)]),
                    Conn(SouthEast, [(5, 6), (6, 5)]),
                ],
            ]
        );
    }
}
