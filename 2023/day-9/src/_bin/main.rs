#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use day_9::bin::{get_input, process_part1, process_part2};

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    println!("Part 1: {}", process_part1(&get_input()));
    println!("Part 2: {}", process_part2(&get_input()));
}
