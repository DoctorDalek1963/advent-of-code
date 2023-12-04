use lazy_static::lazy_static;
use regex::Regex;

pub mod bin;

lazy_static! {
    static ref NUMBER: Regex = Regex::new(r"\d+").unwrap();
    static ref SYMBOL: Regex = Regex::new(r"[^.0-9]").unwrap();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    line_no: usize,
    start_idx: usize,
    end_idx: usize,
}

impl Position {
    fn is_adjacent(&self, other: &Self) -> bool {
        if self.line_no == other.line_no {
            self.end_idx + 1 == other.start_idx || other.end_idx + 1 == self.start_idx
        } else if self.line_no.abs_diff(other.line_no) == 1 {
            (self.end_idx + 1 >= other.start_idx && self.end_idx <= other.end_idx)
                || (other.end_idx + 1 >= self.start_idx && other.end_idx <= self.end_idx)
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Object {
    Number(usize),
    Symbol { is_gear: bool },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schematic {
    numbers: Vec<(usize, Position)>,
    symbols: Vec<(bool, Position)>,
}

impl Schematic {
    pub fn get_part_numbers(&self) -> impl Iterator<Item = usize> + '_ {
        self.numbers.iter().copied().filter_map(|(num, pos)| {
            if self
                .symbols
                .iter()
                .any(|(_is_gear, sym_pos)| sym_pos.is_adjacent(&pos))
            {
                Some(num)
            } else {
                None
            }
        })
    }

    pub fn get_gear_ratios(&self) -> impl Iterator<Item = usize> + '_ {
        self.symbols.iter().filter_map(|&(is_gear, pos)| {
            if is_gear {
                let adjacent_numbers: Vec<usize> = self
                    .numbers
                    .iter()
                    .filter_map(|&(num, num_pos)| num_pos.is_adjacent(&pos).then(|| num))
                    .collect();

                if adjacent_numbers.len() == 2 {
                    Some(adjacent_numbers[0] * adjacent_numbers[1])
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}

pub fn parse_schematic(input: &str) -> Schematic {
    let mut objects: Vec<(Object, Position)> = Vec::new();

    for (line_no, line) in input.lines().enumerate() {
        let numbers = NUMBER.find_iter(line).map(|m| {
            (
                Object::Number(m.as_str().parse::<usize>().unwrap()),
                Position {
                    line_no,
                    start_idx: m.start(),
                    end_idx: m.end() - 1,
                },
            )
        });
        objects.extend(numbers);

        let symbols = SYMBOL.find_iter(line).map(|m| {
            (
                Object::Symbol {
                    is_gear: m.as_str() == "*",
                },
                Position {
                    line_no,
                    start_idx: m.start(),
                    end_idx: m.end() - 1,
                },
            )
        });
        objects.extend(symbols);
    }

    let (numbers, symbols) = objects.into_iter().fold(
        (vec![], vec![]),
        |(mut numbers, mut symbols), (obj, pos)| {
            match obj {
                Object::Number(num) => numbers.push((num, pos)),
                Object::Symbol { is_gear } => symbols.push((is_gear, pos)),
            };
            (numbers, symbols)
        },
    );

    Schematic { numbers, symbols }
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_schematic_test() {
        assert_eq!(
            parse_schematic(TEST_INPUT),
            Schematic {
                numbers: vec![
                    (
                        467,
                        Position {
                            line_no: 0,
                            start_idx: 0,
                            end_idx: 2,
                        },
                    ),
                    (
                        114,
                        Position {
                            line_no: 0,
                            start_idx: 5,
                            end_idx: 7,
                        },
                    ),
                    (
                        35,
                        Position {
                            line_no: 2,
                            start_idx: 2,
                            end_idx: 3,
                        },
                    ),
                    (
                        633,
                        Position {
                            line_no: 2,
                            start_idx: 6,
                            end_idx: 8,
                        },
                    ),
                    (
                        617,
                        Position {
                            line_no: 4,
                            start_idx: 0,
                            end_idx: 2,
                        },
                    ),
                    (
                        58,
                        Position {
                            line_no: 5,
                            start_idx: 7,
                            end_idx: 8,
                        },
                    ),
                    (
                        592,
                        Position {
                            line_no: 6,
                            start_idx: 2,
                            end_idx: 4,
                        },
                    ),
                    (
                        755,
                        Position {
                            line_no: 7,
                            start_idx: 6,
                            end_idx: 8,
                        },
                    ),
                    (
                        664,
                        Position {
                            line_no: 9,
                            start_idx: 1,
                            end_idx: 3,
                        },
                    ),
                    (
                        598,
                        Position {
                            line_no: 9,
                            start_idx: 5,
                            end_idx: 7,
                        },
                    ),
                ],
                symbols: vec![
                    (
                        true,
                        Position {
                            line_no: 1,
                            start_idx: 3,
                            end_idx: 3,
                        }
                    ),
                    (
                        false,
                        Position {
                            line_no: 3,
                            start_idx: 6,
                            end_idx: 6,
                        }
                    ),
                    (
                        true,
                        Position {
                            line_no: 4,
                            start_idx: 3,
                            end_idx: 3,
                        }
                    ),
                    (
                        false,
                        Position {
                            line_no: 5,
                            start_idx: 5,
                            end_idx: 5,
                        }
                    ),
                    (
                        false,
                        Position {
                            line_no: 8,
                            start_idx: 3,
                            end_idx: 3,
                        }
                    ),
                    (
                        true,
                        Position {
                            line_no: 8,
                            start_idx: 5,
                            end_idx: 5,
                        }
                    ),
                ],
            }
        );
    }

    #[test]
    fn position_is_adjacent_test() {
        assert!(Position {
            line_no: 0,
            start_idx: 0,
            end_idx: 2
        }
        .is_adjacent(&Position {
            line_no: 1,
            start_idx: 3,
            end_idx: 3
        }));

        assert!(Position {
            line_no: 4,
            start_idx: 0,
            end_idx: 2
        }
        .is_adjacent(&Position {
            line_no: 4,
            start_idx: 3,
            end_idx: 3
        }));

        assert!(!Position {
            line_no: 5,
            start_idx: 7,
            end_idx: 8
        }
        .is_adjacent(&Position {
            line_no: 5,
            start_idx: 5,
            end_idx: 5
        }));

        assert!(Position {
            line_no: 2,
            start_idx: 2,
            end_idx: 3
        }
        .is_adjacent(&Position {
            line_no: 1,
            start_idx: 3,
            end_idx: 3
        }));
    }

    #[test]
    fn get_part_numbers_test() {
        assert_eq!(
            parse_schematic(TEST_INPUT)
                .get_part_numbers()
                .collect::<Vec<_>>(),
            vec![467, 35, 633, 617, 592, 755, 664, 598]
        );
    }
}
