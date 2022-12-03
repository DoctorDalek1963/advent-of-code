pub fn get_totals_for_groups(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|n| n.parse::<u32>().unwrap()).sum())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_totals_for_groups_test() {
        let input = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(
            get_totals_for_groups(input),
            vec![6000, 4000, 11000, 24000, 10000]
        );
    }
}
