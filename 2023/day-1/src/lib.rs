use lazy_static::lazy_static;
use regex::Regex;

pub mod bin;

lazy_static! {
    static ref NUMBERS: Regex =
        Regex::new(r"([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
}

pub fn get_basic_calibration_value(line: &str) -> u32 {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    10 * digits.first().unwrap() + digits.last().unwrap()
}

pub fn get_complex_calibration_value(line: &str) -> u32 {
    let digits: Vec<u32> = NUMBERS
        .find_iter(
            &line
                .replace("twone", "twoone")
                .replace("eightwo", "eighttwo")
                .replace("nineight", "nineeight")
                .replace("oneight", "oneeight"),
        )
        .map(|m| match m.as_str() {
            "0" | "zero" => 0,
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            _ => unreachable!("NUMBERS can only match certain things"),
        })
        .collect();

    10 * digits.first().unwrap() + digits.last().unwrap()
}

#[cfg(test)]
pub const TEST_INPUT_1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

#[cfg(test)]
pub const TEST_INPUT_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn get_complex_calibration_value_test() {
        assert_eq!(
            TEST_INPUT_2
                .lines()
                .map(get_complex_calibration_value)
                .collect::<Vec<_>>(),
            vec![29, 83, 13, 24, 42, 14, 76]
        );
        assert_eq!(get_complex_calibration_value("3twone"), 31);
    }
}
