use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete, multi::separated_list1, IResult, Parser};

pub mod bin;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LanternFish(u8);

impl LanternFish {
    fn new() -> Self {
        Self(8)
    }

    /// Decrement the internal timer and return whether or not we spawned a new one.
    fn tick(&mut self) -> bool {
        if self.0 == 0 {
            self.0 = 6;
            true
        } else {
            self.0 -= 1;
            false
        }
    }
}

impl From<u8> for LanternFish {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

pub fn simulate_lanternfish_with_vec(mut fish: Vec<LanternFish>, days: u16) -> usize {
    for _ in 0..days {
        let mut new_fish: usize = 0;

        for f in fish.iter_mut() {
            if f.tick() {
                new_fish += 1;
            }
        }

        fish.extend(vec![LanternFish::new(); new_fish]);
    }

    fish.len()
}

pub fn simulate_lanternfish_with_map(fish: Vec<LanternFish>, days: u16) -> usize {
    // Since there are only 9 states for the timer of a laternfish, we can use an array to count
    // the fish with each timer value
    let mut map: [usize; 9] = {
        let map = fish.into_iter().counts();
        [
            *map.get(&LanternFish(0)).unwrap_or(&0),
            *map.get(&LanternFish(1)).unwrap_or(&0),
            *map.get(&LanternFish(2)).unwrap_or(&0),
            *map.get(&LanternFish(3)).unwrap_or(&0),
            *map.get(&LanternFish(4)).unwrap_or(&0),
            *map.get(&LanternFish(5)).unwrap_or(&0),
            *map.get(&LanternFish(6)).unwrap_or(&0),
            *map.get(&LanternFish(7)).unwrap_or(&0),
            *map.get(&LanternFish(8)).unwrap_or(&0),
        ]
    };
    let mut new_map: [usize; 9];

    for _ in 0..days {
        new_map = [0; 9];

        new_map[8] += map[0];
        new_map[6] += map[0];

        new_map[0] += map[1];
        new_map[1] += map[2];
        new_map[2] += map[3];
        new_map[3] += map[4];
        new_map[4] += map[5];
        new_map[5] += map[6];
        new_map[6] += map[7];
        new_map[7] += map[8];

        map.copy_from_slice(&new_map);
    }

    map.iter().sum()
}

pub fn parse_lanternfish(input: &str) -> IResult<&str, Vec<LanternFish>> {
    separated_list1(tag(","), complete::u8.map(Into::into))(input)
}

#[cfg(test)]
pub const TEST_INPUT: &str = "3,4,3,1,2";
