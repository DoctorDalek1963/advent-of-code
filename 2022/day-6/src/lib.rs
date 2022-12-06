pub mod bin;

use itertools::Itertools;
use std::fs;

pub fn get_input() -> String {
    fs::read_to_string("./input.txt").unwrap()
}

pub fn find_unique_slice<const N: usize>(s: &str) -> usize {
    let mut i: usize = 0;
    loop {
        let slice: &str = &s[i..(i + N)];
        if slice.chars().unique().count() == N {
            return i + N;
        } else {
            i += 1;
        }
    }
}
