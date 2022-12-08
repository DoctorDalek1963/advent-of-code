use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

pub mod bin;

fn parse_depths(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(newline, complete::u32)(input)
}

fn count_increases_in_vec(vec: Vec<u32>) -> u32 {
    vec.iter()
        .skip(1)
        .fold((0, vec.get(0).unwrap()), |(count, &previous), current| {
            if *current > previous {
                (count + 1, current)
            } else {
                (count, current)
            }
        })
        .0
}

pub fn count_increasing_depth_differences(input: &str) -> u32 {
    let depths = parse_depths(input).unwrap().1;
    count_increases_in_vec(depths)
}

pub fn count_increasing_window_sums(input: &str) -> u32 {
    let depths = parse_depths(input).unwrap().1;
    let window_sums: Vec<u32> = depths[..]
        .windows(3)
        .map(|s| s.iter().sum::<u32>())
        .collect();
    count_increases_in_vec(window_sums)
}
