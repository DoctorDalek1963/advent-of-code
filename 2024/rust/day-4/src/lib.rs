pub mod bin;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    /// An array of all the directions.
    fn all() -> [Self; 8] {
        [
            Self::North,
            Self::NorthEast,
            Self::East,
            Self::SouthEast,
            Self::South,
            Self::SouthWest,
            Self::West,
            Self::NorthWest,
        ]
    }

    /// Convert a direction to its `(dx, dy)` offset.
    fn offset(self) -> (i32, i32) {
        match self {
            Self::North => (0, -1),
            Self::NorthEast => (1, -1),
            Self::East => (1, 0),
            Self::SouthEast => (1, 1),
            Self::South => (0, 1),
            Self::SouthWest => (-1, 1),
            Self::West => (-1, 0),
            Self::NorthWest => (-1, -1),
        }
    }
}

pub fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|row| row.chars().collect()).collect()
}

/// Scan the grid for "XMAS" starting at `(x, y)` and looking in the given direction.
///
/// This function returns `Some(())` for success and `None` for failure just to make the
/// implementation simpler.
fn scan_for_xmas_starting_here(
    grid: &[Vec<char>],
    coord: (usize, usize),
    direction: Direction,
) -> Option<()> {
    let (x, y) = coord;
    let (dx, dy) = direction.offset();

    let c1 = *grid.get(y)?.get(x)?;
    let c2 = *grid
        .get((y as i32 + dy) as usize)?
        .get((x as i32 + dx) as usize)?;
    let c3 = *grid
        .get((y as i32 + 2 * dy) as usize)?
        .get((x as i32 + 2 * dx) as usize)?;
    let c4 = *grid
        .get((y as i32 + 3 * dy) as usize)?
        .get((x as i32 + 3 * dx) as usize)?;

    if (c1, c2, c3, c4) == ('X', 'M', 'A', 'S') {
        Some(())
    } else {
        None
    }
}

/// Find all the "XMAS" strings in the grid.
pub fn find_all_xmases_in_grid(grid: &[Vec<char>]) -> Vec<((usize, usize), Direction)> {
    let mut all_xmases = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == 'X' {
                all_xmases.extend(Direction::all().into_iter().filter_map(|direction| {
                    scan_for_xmas_starting_here(grid, (x, y), direction)
                        .map(|()| ((x, y), direction))
                }));
            }
        }
    }

    all_xmases
}

/// Scan for an X-MAS centered here.
///
/// This function returns `Some(())` for success and `None` for failure just to make the
/// implementation simpler.
fn scan_for_x_mas_centred_here(grid: &[Vec<char>], coord: (usize, usize)) -> Option<()> {
    let (x, y) = coord;

    let c = *grid.get(y)?.get(x)?;
    let tl = *grid
        .get((y as i32 - 1) as usize)?
        .get((x as i32 - 1) as usize)?;
    let tr = *grid
        .get((y as i32 - 1) as usize)?
        .get((x as i32 + 1) as usize)?;
    let bl = *grid
        .get((y as i32 + 1) as usize)?
        .get((x as i32 - 1) as usize)?;
    let br = *grid
        .get((y as i32 + 1) as usize)?
        .get((x as i32 + 1) as usize)?;

    // centre && \ diag && / diag
    if c == 'A'
        && ((tl == 'M' && br == 'S') || (tl == 'S' && br == 'M'))
        && ((tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M'))
    {
        Some(())
    } else {
        None
    }
}

/// Find all the X-MASes in the grid.
pub fn find_all_x_mases_in_grid(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut all_x_mases = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, _char) in row.iter().enumerate() {
            if scan_for_x_mas_centred_here(grid, (x, y)).is_some() {
                all_x_mases.push((x, y));
            }
        }
    }

    all_x_mases
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;
