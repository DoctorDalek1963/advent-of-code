use nom::{
    character::complete::{anychar, newline},
    multi::{many1, separated_list1},
    IResult,
};

pub mod bin;

type Grid<T, const N: usize> = [[T; N]; N];

type HeightGrid<const N: usize> = Grid<u8, N>;
type BoolGrid<const N: usize> = Grid<bool, N>;
type ScoreGrid<const N: usize> = Grid<u32, N>;

fn check_if_visible_from_outside<const N: usize>(grid: HeightGrid<N>) -> BoolGrid<N> {
    let mut arr = [[true; N]; N];

    for row in 1..(N - 1) {
        for column in 1..(N - 1) {
            let height = grid[row][column];

            let left = &grid[row][..column];
            let right = &grid[row][(column + 1)..];
            let above = &grid[..row]
                .iter()
                .map(|slice| slice[column])
                .collect::<Vec<u8>>();
            let below = &grid[(row + 1)..]
                .iter()
                .map(|slice| slice[column])
                .collect::<Vec<u8>>();

            // A tree is visible if any direction has a sightline to the edge of the grid
            let visible = [left, right, above, below]
                .iter()
                .any(|&it| it.iter().filter(|&n| *n >= height).count() == 0);

            arr[row][column] = visible;
        }
    }

    arr
}

pub fn count_visible_trees<const N: usize>(grid: HeightGrid<N>) -> usize {
    check_if_visible_from_outside(grid)
        .iter()
        .flatten()
        .filter(|&&b| b)
        .count()
}

fn nom_parse_height_grid<const N: usize>(input: &str) -> IResult<&str, HeightGrid<N>> {
    fn single_digit(input: &str) -> IResult<&str, u8> {
        let (input, c) = anychar(input)?;

        // This is a bodge but I don't care
        let n = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Char,
                )))
            }
        };

        Ok((input, n))
    }

    let (input, vec) = separated_list1(newline, many1(single_digit))(input)?;
    let mut arr = [[0; N]; N];
    let height = vec.len();
    let width = vec.get(0).unwrap().len();

    for row in 0..height {
        for column in 0..width {
            arr[row][column] = *vec.get(row).unwrap().get(column).unwrap();
        }
    }

    Ok((input, arr))
}

pub fn parse_height_grid<const N: usize>(input: &str) -> HeightGrid<N> {
    nom_parse_height_grid(input).unwrap().1
}

fn find_scenic_scores<const N: usize>(grid: HeightGrid<N>) -> ScoreGrid<N> {
    let mut arr = [[0; N]; N];

    for row in 1..(N - 1) {
        for column in 1..(N - 1) {
            let height = grid[row][column];

            let left = &grid[row][..column]
                .iter()
                .rev()
                .copied()
                .collect::<Vec<u8>>();
            let right = &grid[row][(column + 1)..];
            let above = &grid[..row]
                .iter()
                .map(|slice| slice[column])
                .rev()
                .collect::<Vec<u8>>();
            let below = &grid[(row + 1)..]
                .iter()
                .map(|slice| slice[column])
                .collect::<Vec<u8>>();

            let score: u32 = [left, right, above, below]
                .iter()
                .map(|&it| {
                    it.iter()
                        .fold((false, 0u32), |(mut vision_blocked, acc), n| {
                            let new_score = if vision_blocked { acc } else { acc + 1 };

                            if *n >= height {
                                vision_blocked = true;
                            }

                            (vision_blocked, new_score)
                        })
                        .1
                })
                .product();

            arr[row][column] = score;
        }
    }

    arr
}

pub fn find_best_scenic_score<const N: usize>(grid: HeightGrid<N>) -> u32 {
    *find_scenic_scores(grid).iter().flatten().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_GRID: HeightGrid<5> = [
        [3, 0, 3, 7, 3],
        [2, 5, 5, 1, 2],
        [6, 5, 3, 3, 2],
        [3, 3, 5, 4, 9],
        [3, 5, 3, 9, 0],
    ];

    const INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn transform_to_bool_array_test() {
        let bool_grid: BoolGrid<5> = [
            [true, true, true, true, true],
            [true, true, true, false, true],
            [true, true, false, true, true],
            [true, false, true, false, true],
            [true, true, true, true, true],
        ];

        assert_eq!(check_if_visible_from_outside(TEST_GRID), bool_grid);
    }

    #[test]
    fn parse_height_grid_test() {
        assert_eq!(parse_height_grid(INPUT), TEST_GRID);
    }

    #[test]
    fn find_scenic_scores_test() {
        let score_grid: ScoreGrid<5> = [
            [0, 0, 0, 0, 0],
            [0, 1, 4, 1, 0],
            [0, 6, 1, 2, 0],
            [0, 1, 8, 3, 0],
            [0, 0, 0, 0, 0],
        ];

        assert_eq!(find_scenic_scores(TEST_GRID), score_grid);
    }
}
