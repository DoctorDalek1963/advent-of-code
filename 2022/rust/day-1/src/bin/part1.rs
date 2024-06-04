use day_1::get_totals_for_groups;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("./input.txt").unwrap();

    let max_cals = get_totals_for_groups(&file_contents)
        .iter()
        .copied()
        .max()
        .unwrap();

    println!("{}", max_cals);
}
