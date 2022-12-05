pub mod bin;
pub mod parse;

use std::fs;

pub fn get_input() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

pub type Stacks = [Vec<char>; 9];

#[derive(Debug, PartialEq)]
pub struct Instruction {
    source: usize,
    dest: usize,
    num: u16,
}

impl Instruction {
    pub fn parse(input: &str) -> Self {
        self::parse::parse_instruction(input).unwrap().1
    }

    pub fn parse_many(input: &str) -> Vec<Self> {
        self::parse::parse_all_instructions(input).unwrap().1
    }

    pub fn perform(&self, stacks: &mut Stacks) {
        for _ in 0..self.num {
            let c = stacks[self.source - 1].pop().unwrap();
            stacks[self.dest - 1].push(c);
        }
    }

    pub fn perform_together(&self, stacks: &mut Stacks) {
        let mut v = vec![];
        for _ in 0..self.num {
            v.push(stacks[self.source - 1].pop().unwrap());
        }
        v.reverse();
        for c in v {
            stacks[self.dest - 1].push(c);
        }
    }
}

pub fn parse_whole_file(input: &str) -> (Stacks, Vec<Instruction>) {
    self::parse::parse_whole_file(input).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perform_instruction_test() {
        let mut stacks = [
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];

        let inst = Instruction {
            source: 1,
            dest: 3,
            num: 1,
        };
        inst.perform(&mut stacks);
        assert_eq!(stacks[0], vec!['Z']);
        assert_eq!(stacks[2], vec!['P', 'N']);

        let inst = Instruction {
            source: 2,
            dest: 1,
            num: 3,
        };
        inst.perform(&mut stacks);
        assert_eq!(stacks[1], vec![]);
        assert_eq!(stacks[0], vec!['Z', 'D', 'C', 'M']);
    }

    #[test]
    fn perform_together_instruction_test() {
        let mut stacks = [
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];

        let inst = Instruction {
            source: 1,
            dest: 3,
            num: 1,
        };
        inst.perform_together(&mut stacks);
        assert_eq!(stacks[0], vec!['Z']);
        assert_eq!(stacks[2], vec!['P', 'N']);

        let inst = Instruction {
            source: 2,
            dest: 1,
            num: 3,
        };
        inst.perform_together(&mut stacks);
        assert_eq!(stacks[0], vec!['Z', 'M', 'C', 'D']);
        assert_eq!(stacks[1], vec![]);
    }

    #[test]
    fn parse_instruction_test() {
        assert_eq!(
            Instruction::parse("move 1 from 2 to 4"),
            Instruction {
                source: 2,
                dest: 4,
                num: 1,
            }
        );

        assert_eq!(
            Instruction::parse("move 18 from 4 to 9"),
            Instruction {
                source: 4,
                dest: 9,
                num: 18,
            }
        );

        assert_eq!(
            Instruction::parse("move 1 from 3 to 1"),
            Instruction {
                source: 3,
                dest: 1,
                num: 1,
            }
        );

        assert_eq!(
            Instruction::parse("move 2 from 2 to 1"),
            Instruction {
                source: 2,
                dest: 1,
                num: 2,
            }
        );
    }

    #[test]
    fn parse_many_instructions_test() {
        let input = "move 1 from 2 to 4
move 18 from 4 to 9
move 1 from 3 to 1
move 2 from 2 to 1";

        let instructions = vec![
            Instruction {
                source: 2,
                dest: 4,
                num: 1,
            },
            Instruction {
                source: 4,
                dest: 9,
                num: 18,
            },
            Instruction {
                source: 3,
                dest: 1,
                num: 1,
            },
            Instruction {
                source: 2,
                dest: 1,
                num: 2,
            },
        ];

        assert_eq!(Instruction::parse_many(input), instructions);
    }
}
