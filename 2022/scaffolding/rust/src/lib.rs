pub mod bin;

use std::fs;

pub fn get_input() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
}
