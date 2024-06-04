use nom::{bytes::complete::tag, character::complete, multi::separated_list1, IResult};

pub mod bin;

pub fn parse_crab_positions(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), complete::u32)(input)
}

pub fn brute_force_optimal_fuel_cost(positions: Vec<u32>) -> u32 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..=max)
        .into_iter()
        .map(|pos| positions.iter().map(|x| x.abs_diff(pos)).sum())
        .min()
        .unwrap()
}

pub fn brute_force_optimal_fuel_cost_part_2(positions: Vec<u32>) -> u32 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..=max)
        .into_iter()
        .map(|pos| {
            positions
                .iter()
                .map(|x| {
                    let n = x.abs_diff(pos);
                    n * (n + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
pub const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_crab_positions_test() {
        assert_eq!(
            parse_crab_positions(TEST_INPUT),
            Ok(("", vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]))
        );
    }
}
