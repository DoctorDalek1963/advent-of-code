use day_DAYNUM::{bin::process_part1, bin::process_part2};

fn get_input() -> String {
    std::fs::read_to_string("./input.txt").unwrap()
}

fn main() {
    println!("Part 1: {:?}", process_part1(&get_input()));
    println!("Part 2: {:?}", process_part2(&get_input()));
}
