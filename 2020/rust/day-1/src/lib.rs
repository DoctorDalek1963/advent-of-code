pub mod bin;

pub fn parse_nums(input: &str) -> Vec<u32> {
    input
        .split("\n")
        .filter_map(|num| num.parse().ok())
        .collect()
}

pub fn find_pair(nums: &[u32]) -> (u32, u32) {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == 2020 {
                return (nums[i], nums[j]);
            }
        }
    }

    panic!("find_pair() should always be able to find the pair")
}

pub fn find_triplet(nums: &[u32]) -> (u32, u32, u32) {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            for k in (j + 1)..nums.len() {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    return (nums[i], nums[j], nums[k]);
                }
            }
        }
    }

    panic!("find_triplet() should always be able to find the triplet")
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"1721
979
366
299
675
1456
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_pair_test() {
        assert_eq!(find_pair(&parse_nums(TEST_INPUT)), (1721, 299));
    }
}
