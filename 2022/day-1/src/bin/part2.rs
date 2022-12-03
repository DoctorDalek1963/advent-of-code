use day_1::get_totals_for_groups;
use std::fs;

fn main() {
    let file_contents = fs::read_to_string("./input.txt").unwrap();

    let mut cals: Vec<u32> = get_totals_for_groups(&file_contents)
        .iter()
        .copied()
        .collect();

    cals.sort_by(|a, b| b.cmp(a));
    println!("{:?}", cals.iter().take(3).sum::<u32>());
}
