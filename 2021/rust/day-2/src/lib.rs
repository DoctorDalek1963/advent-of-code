use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

pub mod bin;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    Forward(u8),
    Up(u8),
    Down(u8),
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    use Command::*;

    let (input, text) = alt((tag("forward"), tag("up"), tag("down")))(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, num) = complete::u8(input)?;

    let command = match text {
        "forward" => Forward(num),
        "up" => Up(num),
        "down" => Down(num),
        _ => unreachable!(),
    };

    Ok((input, command))
}

pub fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(newline, parse_command)(input)
}

pub struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

impl Position {
    pub fn execute_command_part1(&mut self, command: Command) {
        use Command::*;

        match command {
            Forward(n) => self.horizontal += n as i32,
            Up(n) => self.depth -= n as i32,
            Down(n) => self.depth += n as i32,
        };
    }

    pub fn execute_command_part2(&mut self, command: Command) {
        use Command::*;

        match command {
            Forward(n) => {
                self.horizontal += n as i32;
                self.depth += self.aim * n as i32;
            }
            Up(n) => self.aim -= n as i32,
            Down(n) => self.aim += n as i32,
        };
    }

    pub fn position_product(&self) -> i32 {
        self.depth * self.horizontal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_test() {
        use Command::*;

        assert_eq!(parse_command("forward 5"), Ok(("", Forward(5))));
        assert_eq!(parse_command("up 3"), Ok(("", Up(3))));
        assert_eq!(parse_command("down 7"), Ok(("", Down(7))));
    }
}
