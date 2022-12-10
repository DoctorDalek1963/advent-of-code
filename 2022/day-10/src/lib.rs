#![feature(iter_intersperse)]

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

pub mod bin;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

pub struct CPU {
    register: i32,
    current_cycle: u32,
    signal_strength_total: i32,
    pixels: Vec<bool>,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            register: 1,
            current_cycle: 1,
            signal_strength_total: 0,
            pixels: vec![],
        }
    }
}

impl CPU {
    fn increment_current_cycle(&mut self) {
        // For part 1
        if (self.current_cycle as i32 - 20) % 40 == 0 {
            self.signal_strength_total += self.register * self.current_cycle as i32;
        }

        // For part 2
        let row_index = (self.current_cycle - 1) % 40;
        if i32::abs(row_index as i32 - self.register) <= 1 {
            self.pixels.push(true);
        } else {
            self.pixels.push(false);
        }

        // Increment
        self.current_cycle += 1;
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => self.increment_current_cycle(),
            Instruction::Addx(n) => {
                self.increment_current_cycle();
                self.increment_current_cycle();
                self.register += n;
            }
        };
    }

    pub fn get_crt_output(&self) -> String {
        self.pixels
            .chunks(40)
            .map(|s| {
                s.iter()
                    .map(|&b| if b { '#' } else { '.' })
                    .collect::<String>()
            })
            .intersperse("\n".to_string())
            .collect()
    }
}

pub fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    fn parse_single_instruction(input: &str) -> IResult<&str, Instruction> {
        fn parse_addx(input: &str) -> IResult<&str, Instruction> {
            let (input, (_, n)) = separated_pair(tag("addx"), tag(" "), complete::i32)(input)?;
            Ok((input, Instruction::Addx(n)))
        }

        alt((tag("noop").map(|_| Instruction::Noop), parse_addx))(input)
    }

    separated_list1(newline, parse_single_instruction)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_instructions_test() {
        use Instruction::*;

        const INPUT: &str = "addx 15
addx -11
noop
addx -3
noop
addx 5
";

        assert_eq!(
            parse_instructions(INPUT),
            Ok((
                "\n",
                vec![Addx(15), Addx(-11), Noop, Addx(-3), Noop, Addx(5)]
            ))
        );
    }

    #[test]
    fn pixels_test() {
        const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
";

        let mut cpu = CPU::default();
        for instruction in parse_instructions(INPUT).unwrap().1 {
            cpu.execute_instruction(instruction);
        }

        assert_eq!(
            cpu.pixels,
            vec![
                true, true, false, false, true, true, false, false, true, true, false, false, true,
                true, false, false, true, true, false, false, true,
            ]
        );
    }
}
