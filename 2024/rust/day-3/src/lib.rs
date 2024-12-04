pub mod bin;

use regex::Regex;

lazy_static::lazy_static! {
    static ref MUL_REGEX: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
}

pub fn read_all_mul_instructions(input: &str) -> Vec<(u32, u32)> {
    MUL_REGEX
        .captures_iter(input)
        .map(|c| {
            let (_, [x, y]) = c.extract();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

/// One of three instructions, either enable muls, disabling muls, or a mul instruction itself.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConditionalMul {
    Enable,
    Disable,
    Mul(u32, u32),
}

pub fn read_all_conditional_mul_instructions(input: &str) -> Vec<ConditionalMul> {
    let reg = regex::Regex::new(r"(do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\))").unwrap();

    reg.captures_iter(input)
        .map(|c| {
            let (thing, [_]) = c.extract();
            match thing {
                "do()" => ConditionalMul::Enable,
                "don't()" => ConditionalMul::Disable,
                _ => {
                    let (_, [x, y]) = MUL_REGEX.captures(thing).unwrap().extract();
                    ConditionalMul::Mul(x.parse().unwrap(), y.parse().unwrap())
                }
            }
        })
        .collect()
}
