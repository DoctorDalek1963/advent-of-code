#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::*;

pub fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

pub fn process_part1(input: &str) -> u32 {
    read_all_mul_instructions(input)
        .into_iter()
        .map(|(x, y)| x * y)
        .sum()
}

pub fn process_part2(input: &str) -> u32 {
    let mut mul_enabled = true;
    let mut acc = 0u32;

    for instruction in read_all_conditional_mul_instructions(input) {
        match instruction {
            ConditionalMul::Enable => mul_enabled = true,
            ConditionalMul::Disable => mul_enabled = false,
            ConditionalMul::Mul(x, y) => {
                if mul_enabled {
                    acc += x * y;
                }
            }
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn process_part1_test() {
        assert_eq!(
            process_part1(
                r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#
            ),
            161
        );
        assert_eq!(process_part1(&get_input()), 187_825_547);
    }

    #[test]
    fn process_part2_test() {
        assert_eq!(
            process_part2(
                r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#
            ),
            48
        );
        assert_eq!(process_part2(&get_input()), 85_508_223);
    }
}
