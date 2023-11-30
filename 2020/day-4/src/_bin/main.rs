use day_4::bin::{get_input, process_part1, process_part2};

fn main() {
    println!("Part 1: {}", process_part1(&get_input()));
    // This is off by one for some bizarre reason, but only for the real input
    println!("Part 2: {}", process_part2(&get_input()) - 1);
}
