use std::fs;

pub fn get_input() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

pub fn do_stuff() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_name_test() {}
}
