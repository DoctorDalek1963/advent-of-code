pub mod bin;
mod parse;

pub use self::parse::parse_games;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CubeSet {
    red: u16,
    green: u16,
    blue: u16,
}

impl CubeSet {
    pub fn power(&self) -> u32 {
        self.red as u32 * self.green as u32 * self.blue as u32
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    id: u16,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    pub fn is_possible(&self, available_cubes: CubeSet) -> bool {
        for set in &self.cube_sets {
            if set.red > available_cubes.red
                || set.green > available_cubes.green
                || set.blue > available_cubes.blue
            {
                return false;
            }
        }

        true
    }

    pub fn get_minimum_set(&self) -> CubeSet {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for set in &self.cube_sets {
            red = red.max(set.red);
            green = green.max(set.green);
            blue = blue.max(set.blue);
        }

        CubeSet { red, green, blue }
    }
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_games_test() {
        assert_eq!(
            parse_games(TEST_INPUT),
            Ok((
                "\n",
                vec![
                    Game {
                        id: 1,
                        cube_sets: vec![
                            CubeSet {
                                red: 4,
                                green: 0,
                                blue: 3,
                            },
                            CubeSet {
                                red: 1,
                                green: 2,
                                blue: 6,
                            },
                            CubeSet {
                                red: 0,
                                green: 2,
                                blue: 0,
                            },
                        ],
                    },
                    Game {
                        id: 2,
                        cube_sets: vec![
                            CubeSet {
                                red: 0,
                                green: 2,
                                blue: 1,
                            },
                            CubeSet {
                                red: 1,
                                green: 3,
                                blue: 4,
                            },
                            CubeSet {
                                red: 0,
                                green: 1,
                                blue: 1,
                            },
                        ],
                    },
                    Game {
                        id: 3,
                        cube_sets: vec![
                            CubeSet {
                                red: 20,
                                green: 8,
                                blue: 6,
                            },
                            CubeSet {
                                red: 4,
                                green: 13,
                                blue: 5,
                            },
                            CubeSet {
                                red: 1,
                                green: 5,
                                blue: 0,
                            },
                        ],
                    },
                    Game {
                        id: 4,
                        cube_sets: vec![
                            CubeSet {
                                red: 3,
                                green: 1,
                                blue: 6,
                            },
                            CubeSet {
                                red: 6,
                                green: 3,
                                blue: 0,
                            },
                            CubeSet {
                                red: 14,
                                green: 3,
                                blue: 15,
                            },
                        ],
                    },
                    Game {
                        id: 5,
                        cube_sets: vec![
                            CubeSet {
                                red: 6,
                                green: 3,
                                blue: 1,
                            },
                            CubeSet {
                                red: 1,
                                green: 2,
                                blue: 2,
                            },
                        ],
                    },
                ]
            ))
        );
    }
}
