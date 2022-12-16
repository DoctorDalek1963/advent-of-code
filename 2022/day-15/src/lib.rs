#![feature(btree_drain_filter)]

pub mod bin;

use nom::{bytes::complete::tag, character::complete, multi::separated_list1, IResult};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::{collections::BTreeSet, sync::Mutex};

pub type Point = (i32, i32);

#[inline]
fn manhattan_distance(p: Point, q: Point) -> u32 {
    i32::abs_diff(p.0, q.0) + i32::abs_diff(p.1, q.1)
}

pub fn parse_sensor_beacon_pairs(input: &str) -> IResult<&str, Vec<SensorBeaconPair>> {
    fn parse_sensor_beacon_pair(input: &str) -> IResult<&str, SensorBeaconPair> {
        let (input, _) = tag("Sensor at x=")(input)?;
        let (input, sx) = complete::i32(input)?;
        let (input, _) = tag(", y=")(input)?;
        let (input, sy) = complete::i32(input)?;
        let (input, _) = tag(": closest beacon is at x=")(input)?;
        let (input, bx) = complete::i32(input)?;
        let (input, _) = tag(", y=")(input)?;
        let (input, by) = complete::i32(input)?;

        Ok((
            input,
            SensorBeaconPair {
                sensor: (sx, sy),
                beacon: (bx, by),
            },
        ))
    }

    separated_list1(tag("\n"), parse_sensor_beacon_pair)(input)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SensorBeaconPair {
    sensor: Point,
    beacon: Point,
}

impl SensorBeaconPair {
    fn get_dist(&self) -> u32 {
        manhattan_distance(self.sensor, self.beacon)
    }

    fn contains(&self, point: Point) -> bool {
        manhattan_distance(self.sensor, point) <= self.get_dist()
    }

    fn get_points_just_outside_range(&self) -> BTreeSet<Point> {
        let dist = self.get_dist() as i32;
        let mut set = BTreeSet::new();
        let (sx, sy) = self.sensor;

        let mut x: i32;
        let mut y: i32;

        (x, y) = (sx, sy - dist - 1);
        while y < sy {
            set.insert((x, y));
            x += 1;
            y += 1;
        }
        while x > sx {
            set.insert((x, y));
            x -= 1;
            y += 1;
        }
        while y > sy {
            set.insert((x, y));
            x -= 1;
            y -= 1;
        }
        while x < sx {
            set.insert((x, y));
            x += 1;
            y -= 1;
        }

        set
    }
}

pub fn count_occupied_cells_at_y_level(sb_pairs: Vec<SensorBeaconPair>, y_level: i32) -> usize {
    let x_iter = sb_pairs
        .iter()
        .map(|&SensorBeaconPair { sensor: _, beacon }| beacon.0);
    let x_range = (x_iter.clone().min().unwrap())..=(x_iter.max().unwrap() * 15 / 10);

    let sensors_and_distances: Vec<_> =
        sb_pairs.iter().map(|&s| (s.sensor, s.get_dist())).collect();

    x_range
        .into_iter()
        .filter(|&x| {
            sensors_and_distances
                .iter()
                .any(|&(sensor, dist)| manhattan_distance((x, y_level), sensor) <= dist)
        })
        .filter(|&x| {
            !sb_pairs
                .iter()
                .any(|&SensorBeaconPair { sensor: _, beacon }| beacon == (x, y_level))
        })
        .count()
}

pub fn get_tuning_frequency_of_beacon_pos(sb_pairs: Vec<SensorBeaconPair>, max: i32) -> u64 {
    let search_space: Mutex<BTreeSet<Point>> = Mutex::new(BTreeSet::new());
    sb_pairs.par_iter().for_each(|&sbp| {
        let mut points = sbp.get_points_just_outside_range();
        let points_inside: Vec<Point> = points
            .drain_filter(|&(x, y)| x >= 0 && x <= max && y >= 0 && y <= max)
            .collect();

        search_space.lock().unwrap().extend(points_inside);
    });

    for p in search_space.lock().unwrap().iter() {
        if !sb_pairs.iter().any(|s| s.contains(*p)) {
            let (x, y) = *p;
            return x as u64 * 4_000_000 + y as u64;
        }
    }

    unreachable!("there should be exactly one possible point for the distress beacon");
}

#[cfg(test)]
pub const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sensor_beacon_pairs_test() {
        assert_eq!(
            parse_sensor_beacon_pairs(TEST_INPUT),
            Ok((
                "\n",
                vec![
                    SensorBeaconPair {
                        sensor: (2, 18),
                        beacon: (-2, 15)
                    },
                    SensorBeaconPair {
                        sensor: (9, 16),
                        beacon: (10, 16)
                    },
                    SensorBeaconPair {
                        sensor: (13, 2),
                        beacon: (15, 3)
                    },
                    SensorBeaconPair {
                        sensor: (12, 14),
                        beacon: (10, 16)
                    },
                    SensorBeaconPair {
                        sensor: (10, 20),
                        beacon: (10, 16)
                    },
                    SensorBeaconPair {
                        sensor: (14, 17),
                        beacon: (10, 16)
                    },
                    SensorBeaconPair {
                        sensor: (8, 7),
                        beacon: (2, 10)
                    },
                    SensorBeaconPair {
                        sensor: (2, 0),
                        beacon: (2, 10)
                    },
                    SensorBeaconPair {
                        sensor: (0, 11),
                        beacon: (2, 10)
                    },
                    SensorBeaconPair {
                        sensor: (20, 14),
                        beacon: (25, 17)
                    },
                    SensorBeaconPair {
                        sensor: (17, 20),
                        beacon: (21, 22)
                    },
                    SensorBeaconPair {
                        sensor: (16, 7),
                        beacon: (15, 3)
                    },
                    SensorBeaconPair {
                        sensor: (14, 3),
                        beacon: (15, 3)
                    },
                    SensorBeaconPair {
                        sensor: (20, 1),
                        beacon: (15, 3)
                    },
                ]
            ))
        );
    }

    #[test]
    fn get_points_just_outside_range_test() {
        let sbp = SensorBeaconPair {
            sensor: (0, 0),
            beacon: (3, 0),
        };
        let set = BTreeSet::from([
            (0, 4),
            (1, 3),
            (2, 2),
            (3, 1),
            (4, 0),
            (3, -1),
            (2, -2),
            (1, -3),
            (0, -4),
            (-1, -3),
            (-2, -2),
            (-3, -1),
            (-4, 0),
            (-3, 1),
            (-2, 2),
            (-1, 3),
        ]);

        assert_eq!(sbp.get_points_just_outside_range(), set);
    }
}
