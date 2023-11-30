use nom::{bytes::complete::tag, character::complete::alpha1, multi::separated_list1, IResult};
use std::collections::{HashMap, HashSet};

pub mod bin;

pub fn parse_groups(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    separated_list1(tag("\n\n"), separated_list1(tag("\n"), alpha1))(input)
}

pub fn collect_responses(group: &[&str]) -> HashSet<char> {
    let mut set = HashSet::new();
    for response in group {
        for c in response.chars() {
            set.insert(c);
        }
    }
    set
}

pub fn collect_and_count_responses(group: &[&str]) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for response in group {
        for c in response.chars() {
            map.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }
    }
    map
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_groups_test() {
        assert_eq!(
            parse_groups(TEST_INPUT),
            Ok((
                "\n",
                vec![
                    vec!["abc"],
                    vec!["a", "b", "c"],
                    vec!["ab", "ac"],
                    vec!["a", "a", "a", "a",],
                    vec!["b"],
                ]
            ))
        );
    }
}
