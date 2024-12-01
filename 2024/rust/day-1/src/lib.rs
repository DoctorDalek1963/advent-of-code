pub mod bin;

pub fn get_two_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut nums = line.split("   ");
            let a: u32 = nums.next().unwrap().parse().unwrap();
            let b: u32 = nums.next().unwrap().parse().unwrap();
            (a, b)
        })
        .unzip()
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
