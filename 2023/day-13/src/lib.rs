#![feature(iter_intersperse)]

use bitvec::vec::BitVec;

pub mod bin;

type Scan = Vec<BitVec>;

fn parse_single_scan(input: &str) -> Scan {
    input
        .lines()
        .map(|row| row.chars().map(|c| c == '#').collect())
        .collect()
}

pub fn parse_scans(input: &str) -> Vec<Scan> {
    input
        .split("\n\n")
        .map(|block| parse_single_scan(block))
        .collect()
}

pub fn render_scan(scan: &Scan) -> String {
    scan.iter()
        .map(|row| row.iter().map(|bit| if *bit { '#' } else { '.' }).collect())
        .intersperse(String::from("\n"))
        .collect()
}

pub fn find_lines_of_symmetry(
    scan: &Scan,
    is_columns: bool,
    is_smudged: bool,
) -> impl Iterator<Item = usize> + '_ {
    let total = if is_columns {
        scan.first().unwrap().len()
    } else {
        scan.len()
    };
    let function: fn(&Scan, usize) -> bool = match (is_columns, is_smudged) {
        (true, true) => is_column_of_smudged_symmetry,
        (true, false) => is_column_of_symmetry,
        (false, true) => is_row_of_smudged_symmetry,
        (false, false) => is_row_of_symmetry,
    };

    (0..(total - 1))
        .into_iter()
        .filter_map(move |idx| function(&scan, idx).then_some(idx + 1))
}

fn is_column_of_symmetry(scan: &Scan, col_idx: usize) -> bool {
    scan.iter().all(|row| {
        let before = &row[..=col_idx];
        let after = &row[(col_idx + 1)..];

        before.iter().rev().zip(after).all(|(a, b)| a == b)
    })
}

fn is_row_of_symmetry(scan: &Scan, row_idx: usize) -> bool {
    let before = scan[..=row_idx].iter().rev();
    let after = &scan[(row_idx + 1)..];

    before.zip(after).all(|(a, b)| a == b)
}

fn is_column_of_smudged_symmetry(scan: &Scan, col_idx: usize) -> bool {
    let mut errors: u8 = 0;

    for row in scan {
        let before = &row[..=col_idx];
        let after = &row[(col_idx + 1)..];

        for (a, b) in before.iter().rev().zip(after) {
            if a != b {
                errors += 1;
                if errors > 1 {
                    return false;
                }
            }
        }
    }

    errors == 1
}

fn is_row_of_smudged_symmetry(scan: &Scan, row_idx: usize) -> bool {
    let mut errors: u8 = 0;

    let before = scan[..=row_idx].iter().rev();
    let after = &scan[(row_idx + 1)..];

    for (row_a, row_b) in before.zip(after) {
        errors += (row_a.to_owned() ^ row_b).count_ones() as u8;
        if errors > 1 {
            return false;
        }
    }

    errors == 1
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    macro_rules! bitvec {
        ($($val:expr),* $(,)?) => {
            ::bitvec::bitvec![usize, ::bitvec::order::LocalBits; $($val),*]
        };
    }

    #[test]
    fn parse_scans_test() {
        let parsed = parse_scans(TEST_INPUT);
        let expected = vec![
            vec![
                bitvec![1, 0, 1, 1, 0, 0, 1, 1, 0],
                bitvec![0, 0, 1, 0, 1, 1, 0, 1, 0],
                bitvec![1, 1, 0, 0, 0, 0, 0, 0, 1],
                bitvec![1, 1, 0, 0, 0, 0, 0, 0, 1],
                bitvec![0, 0, 1, 0, 1, 1, 0, 1, 0],
                bitvec![0, 0, 1, 1, 0, 0, 1, 1, 0],
                bitvec![1, 0, 1, 0, 1, 1, 0, 1, 0],
            ],
            vec![
                bitvec![1, 0, 0, 0, 1, 1, 0, 0, 1],
                bitvec![1, 0, 0, 0, 0, 1, 0, 0, 1],
                bitvec![0, 0, 1, 1, 0, 0, 1, 1, 1],
                bitvec![1, 1, 1, 1, 1, 0, 1, 1, 0],
                bitvec![1, 1, 1, 1, 1, 0, 1, 1, 0],
                bitvec![0, 0, 1, 1, 0, 0, 1, 1, 1],
                bitvec![1, 0, 0, 0, 0, 1, 0, 0, 1],
            ],
        ];
        assert_eq!(parsed, expected);
    }

    #[test]
    fn is_column_of_symmetry_test() {
        let scans = parse_scans(TEST_INPUT);

        assert!(!is_column_of_symmetry(&scans[0], 0));
        assert!(!is_column_of_symmetry(&scans[0], 1));
        assert!(!is_column_of_symmetry(&scans[0], 2));
        assert!(!is_column_of_symmetry(&scans[0], 3));
        assert!(is_column_of_symmetry(&scans[0], 4));
        assert!(!is_column_of_symmetry(&scans[0], 5));
        assert!(!is_column_of_symmetry(&scans[0], 6));
        assert!(!is_column_of_symmetry(&scans[0], 7));

        assert!(!is_column_of_symmetry(&scans[1], 0));
        assert!(!is_column_of_symmetry(&scans[1], 1));
        assert!(!is_column_of_symmetry(&scans[1], 2));
        assert!(!is_column_of_symmetry(&scans[1], 3));
        assert!(!is_column_of_symmetry(&scans[1], 4));
        assert!(!is_column_of_symmetry(&scans[1], 5));
        assert!(!is_column_of_symmetry(&scans[1], 6));
        assert!(!is_column_of_symmetry(&scans[1], 7));
    }

    #[test]
    fn is_row_of_symmetry_test() {
        let scans = parse_scans(TEST_INPUT);

        assert!(!is_row_of_symmetry(&scans[0], 0));
        assert!(!is_row_of_symmetry(&scans[0], 1));
        assert!(!is_row_of_symmetry(&scans[0], 2));
        assert!(!is_row_of_symmetry(&scans[0], 3));
        assert!(!is_row_of_symmetry(&scans[0], 4));
        assert!(!is_row_of_symmetry(&scans[0], 5));

        assert!(!is_row_of_symmetry(&scans[1], 0));
        assert!(!is_row_of_symmetry(&scans[1], 1));
        assert!(!is_row_of_symmetry(&scans[1], 2));
        assert!(is_row_of_symmetry(&scans[1], 3));
        assert!(!is_row_of_symmetry(&scans[1], 4));
        assert!(!is_row_of_symmetry(&scans[1], 5));
    }

    #[test]
    fn find_lines_of_symmetry_test() {
        let scan1 = parse_single_scan(
            r#"####.....
.##.##...
#..#.####
.#..#.###
#..#...##
#..##....
.....##..
.##.#.###
#..####..
......###
#..#...##
....#.###
....##...
.##...#..
.##..#..."#,
        );

        assert_eq!(
            find_lines_of_symmetry(&scan1, true, false).collect::<Vec<_>>(),
            vec![8]
        );
        assert_eq!(
            find_lines_of_symmetry(&scan1, false, false).collect::<Vec<_>>(),
            vec![]
        );

        let scan2 = parse_single_scan(
            r#"##..#.##....##.#.
###.#....##....#.
##...####..####..
########.##.#####
##...#.#....#.#..
##.##.########.##
......#..##......
..#.##........##.
..#.#...####...#.
###.##.#.##.#.##.
##.####..##..####
...##..#.##.#..##
......##....##...
"#,
        );

        assert_eq!(
            find_lines_of_symmetry(&scan2, true, false).collect::<Vec<_>>(),
            vec![1]
        );
        assert_eq!(
            find_lines_of_symmetry(&scan2, false, false).collect::<Vec<_>>(),
            vec![]
        );

        let scan3 = parse_single_scan(
            r#"...###.##.##.....
.##.#..#.##......
..###..#.#.#.#.##
#...###....#...##
###.##..#.###..##
.##.##.##.#..#...
.##.##..#.#..#...
###.##..#.###..##
#...###....#...##
..###..#.#.#.#.##
.##.#..#.##......
...###.##.##.....
##....###.#.#####
.#.##...##.#.##..
#.#.#.###.....###
"#,
        );

        assert_eq!(
            find_lines_of_symmetry(&scan3, true, false).collect::<Vec<_>>(),
            vec![16]
        );
        assert_eq!(
            find_lines_of_symmetry(&scan3, false, false).collect::<Vec<_>>(),
            vec![]
        );

        let scan4 = parse_single_scan(
            r#"...##..##......
...###....#.###
...#.###.##....
..#.##....####.
..###.#..#..###
##...#.########
..###.#......##
..#..#.#.#.####
..###...#.#####
..###........##
##..#.#.##.####
"#,
        );

        assert_eq!(
            find_lines_of_symmetry(&scan4, true, false).collect::<Vec<_>>(),
            vec![1]
        );
        assert_eq!(
            find_lines_of_symmetry(&scan4, false, false).collect::<Vec<_>>(),
            vec![]
        );
    }
}
