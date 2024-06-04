use day_4::*;

fn process(s: &str) -> usize {
    s.lines()
        .map(parse_ranges)
        .map(range_fully_contains_other)
        .filter(|&e| e)
        .count()
}

fn main() {
    println!("{}", process(&get_input()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_test() {
        const INPUT: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(process(INPUT), 2);
    }
}
