#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

fn process_general(input: &str, factor: usize) -> usize {
    let universe = parse_universe(input).unwrap().1;
    let (rows_to_copy, cols_to_copy) = rows_cols_to_copy(&universe);

    let galaxies = universe
        .into_iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.into_iter()
                .enumerate()
                .filter_map(|(col_idx, position)| position.then_some((row_idx, col_idx)))
                .collect::<Vec<_>>()
        })
        .flatten()
        .map(|(row, col)| {
            let prior_empty_rows = rows_to_copy.iter().filter(|&&idx| idx < row).count();
            let prior_empty_cols = cols_to_copy.iter().filter(|&&idx| idx < col).count();

            (
                row + prior_empty_rows * (factor - 1),
                col + prior_empty_cols * (factor - 1),
            )
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            sum += manhattan_distance(galaxies[i], galaxies[j]);
        }
    }
    sum
}

pub fn process_part1(input: &str) -> usize {
    process_general(input, 2)
}

pub fn process_part2(input: &str) -> usize {
    process_general(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TEST_INPUT;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(process_part1(TEST_INPUT), 374);
        assert_eq!(process_part1(&get_input()), 9_608_724);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(process_general(TEST_INPUT, 10), 1030);
        assert_eq!(process_general(TEST_INPUT, 100), 8410);
        // No example given for TEST_INPUT with factor of 1_000_000
        assert_eq!(process_part2(&get_input()), 904_633_799_472);
    }
}
