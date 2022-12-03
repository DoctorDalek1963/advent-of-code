#![feature(iter_array_chunks)]

use day_3::*;

fn get_total_badge_priority(file_contents: &str) -> u32 {
    file_contents
        .lines()
        .array_chunks::<3>()
        .map(|[r1, r2, r3]| {
            r1.chars()
                .filter(|&c| r2.contains(c) && r3.contains(c))
                .next()
                .unwrap()
        })
        .map(|c| char_to_priority(c))
        .sum()
}

fn main() {
    let total_badge_priority: u32 = get_total_badge_priority(&get_input());

    println!("{}", total_badge_priority);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_total_badge_priority_test() {
        const INPUT: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(get_total_badge_priority(INPUT), 70)
    }
}
