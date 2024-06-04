use std::cmp::Ordering::{self, Equal, Greater, Less};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for HandShape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use HandShape::*;

        Some(match self {
            Rock => match other {
                Rock => Equal,
                Paper => Less,
                Scissors => Greater,
            },
            Paper => match other {
                Rock => Greater,
                Paper => Equal,
                Scissors => Less,
            },
            Scissors => match other {
                Rock => Less,
                Paper => Greater,
                Scissors => Equal,
            },
        })
    }
}

impl Ord for HandShape {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl From<char> for HandShape {
    fn from(c: char) -> Self {
        use HandShape::*;

        match c {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => unreachable!(),
        }
    }
}

impl HandShape {
    pub fn raw_score(&self) -> u32 {
        use HandShape::*;

        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

pub enum WinType {
    Win,
    Draw,
    Lose,
}

impl From<char> for WinType {
    fn from(c: char) -> Self {
        use WinType::*;

        match c {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            _ => unreachable!(),
        }
    }
}

impl WinType {
    pub fn get_winning_shape_against(&self, shape: HandShape) -> HandShape {
        use HandShape::*;
        use WinType::*;

        match self {
            Win => match shape {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
            Draw => shape,
            Lose => match shape {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            },
        }
    }
}
