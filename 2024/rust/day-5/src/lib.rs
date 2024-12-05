pub mod bin;

use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PageOrderingRule(u8, u8);

pub fn parse_input(input: &str) -> (Vec<PageOrderingRule>, Vec<Vec<u8>>) {
    let mut input_split = input.split("\n\n");

    let rules = input_split
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut x = line.split("|");
            let left = x.next().unwrap().parse().unwrap();
            let right = x.next().unwrap().parse().unwrap();
            PageOrderingRule(left, right)
        })
        .collect();

    let page_lists = input_split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    (rules, page_lists)
}

#[inline]
fn cmp_with_rules(rules: &[PageOrderingRule], a: u8, b: u8) -> Ordering {
    if rules.contains(&PageOrderingRule(a, b)) {
        Ordering::Less
    } else if rules.contains(&PageOrderingRule(b, a)) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

/// Sort the page list into order according to the [`PageOrderingRule`]s.
pub fn order_page_list(rules: &[PageOrderingRule], mut page_list: Vec<u8>) -> Vec<u8> {
    page_list.sort_by(|&a, &b| cmp_with_rules(rules, a, b));
    page_list
}

/// Check if the given page list is sorted according to the [`PageOrderingRule`]s.
pub fn is_page_list_ordered(rules: &[PageOrderingRule], page_list: &[u8]) -> bool {
    page_list.is_sorted_by(|&a, &b| {
        matches!(
            cmp_with_rules(rules, a, b),
            Ordering::Less | Ordering::Equal
        )
    })
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT),
            (
                vec![
                    PageOrderingRule(47, 53,),
                    PageOrderingRule(97, 13,),
                    PageOrderingRule(97, 61,),
                    PageOrderingRule(97, 47,),
                    PageOrderingRule(75, 29,),
                    PageOrderingRule(61, 13,),
                    PageOrderingRule(75, 53,),
                    PageOrderingRule(29, 13,),
                    PageOrderingRule(97, 29,),
                    PageOrderingRule(53, 29,),
                    PageOrderingRule(61, 53,),
                    PageOrderingRule(97, 53,),
                    PageOrderingRule(61, 29,),
                    PageOrderingRule(47, 13,),
                    PageOrderingRule(75, 47,),
                    PageOrderingRule(97, 75,),
                    PageOrderingRule(47, 61,),
                    PageOrderingRule(75, 61,),
                    PageOrderingRule(47, 29,),
                    PageOrderingRule(75, 13,),
                    PageOrderingRule(53, 13,),
                ],
                vec![
                    vec![75, 47, 61, 53, 29,],
                    vec![97, 61, 53, 29, 13,],
                    vec![75, 29, 13,],
                    vec![75, 97, 47, 61, 53,],
                    vec![61, 13, 29,],
                    vec![97, 13, 75, 29, 47,],
                ],
            )
        );
    }

    #[test]
    fn test_is_page_list_ordered() {
        let (rules, _) = parse_input(TEST_INPUT);

        assert!(is_page_list_ordered(&rules, &[75, 47, 61, 53, 29]));
        assert!(is_page_list_ordered(&rules, &[97, 61, 53, 29, 13]));
        assert!(is_page_list_ordered(&rules, &[75, 29, 13]));
        assert!(!is_page_list_ordered(&rules, &[75, 97, 47, 61, 53]));
        assert!(!is_page_list_ordered(&rules, &[61, 13, 29]));
        assert!(!is_page_list_ordered(&rules, &[97, 13, 75, 29, 47]));
    }
}
