use day_3::*;

fn get_total_mismatched_type_priorities(file_contents: &str) -> u32 {
    file_contents
        .lines()
        .map(|line| char_to_priority(get_mismatched_type_from_rucksack(line).unwrap()))
        .sum()
}

fn main() {
    println!("{}", get_total_mismatched_type_priorities(&get_input()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_total_mismatched_type_priorities_test() {
        const INPUT: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(get_total_mismatched_type_priorities(INPUT), 157);
    }
}
