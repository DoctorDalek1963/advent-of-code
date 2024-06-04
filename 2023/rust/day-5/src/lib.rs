#![feature(array_chunks)]

use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1, newline},
    multi::separated_list1,
    IResult,
};
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

pub mod bin;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MapTriplet {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl MapTriplet {
    #[inline]
    fn convert(&self, number: u64) -> Option<u64> {
        debug_assert!(self.range_length >= 1);
        if number >= self.source_range_start
            && number <= (self.source_range_start + self.range_length - 1)
        {
            Some(self.destination_range_start + number - self.source_range_start)
        } else {
            None
        }
    }

    fn convert_num_with_list(list: &[Self], number: u64) -> u64 {
        for map in list {
            if let Some(n) = map.convert(number) {
                return n;
            }
        }
        number
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil_map: Vec<MapTriplet>,
    soil_to_fertilizer_map: Vec<MapTriplet>,
    fertilizer_to_water_map: Vec<MapTriplet>,
    water_to_light_map: Vec<MapTriplet>,
    light_to_temperature_map: Vec<MapTriplet>,
    temperature_to_humidity_map: Vec<MapTriplet>,
    humidity_to_location_map: Vec<MapTriplet>,
}

impl Almanac {
    fn get_locations_from_iter<'i>(
        &'i self,
        iter: impl ParallelIterator<Item = u64> + 'i,
    ) -> impl ParallelIterator<Item = u64> + 'i {
        iter.map(|seed| {
            let soil = MapTriplet::convert_num_with_list(&self.seed_to_soil_map, seed);
            let fertilizer = MapTriplet::convert_num_with_list(&self.soil_to_fertilizer_map, soil);
            let water =
                MapTriplet::convert_num_with_list(&self.fertilizer_to_water_map, fertilizer);
            let light = MapTriplet::convert_num_with_list(&self.water_to_light_map, water);
            let temperature =
                MapTriplet::convert_num_with_list(&self.light_to_temperature_map, light);
            let humidity =
                MapTriplet::convert_num_with_list(&self.temperature_to_humidity_map, temperature);
            MapTriplet::convert_num_with_list(&self.humidity_to_location_map, humidity)
        })
    }

    pub fn get_locations(&self) -> impl ParallelIterator<Item = u64> + '_ {
        self.get_locations_from_iter(self.seeds.par_iter().copied())
    }

    pub fn reinterpret_seed_numbers_and_get_locations(
        &self,
    ) -> impl ParallelIterator<Item = u64> + '_ {
        self.get_locations_from_iter(
            self.seeds
                .array_chunks()
                .par_bridge()
                .map(|&[start, range]| start..(start + range))
                .flatten(),
        )
    }
}

fn parse_map_triplets(input: &str) -> IResult<&str, Vec<MapTriplet>> {
    fn parse_map_triplet(input: &str) -> IResult<&str, MapTriplet> {
        let (input, destination_range_start) = complete::u64(input)?;
        let (input, _) = multispace1(input)?;
        let (input, source_range_start) = complete::u64(input)?;
        let (input, _) = multispace1(input)?;
        let (input, range_length) = complete::u64(input)?;
        Ok((
            input,
            MapTriplet {
                destination_range_start,
                source_range_start,
                range_length,
            },
        ))
    }

    separated_list1(newline, parse_map_triplet)(input)
}

pub fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(multispace1, complete::u64)(input)?;

    let (input, _) = multispace1(input)?;
    let (input, _) = tag("seed-to-soil map:\n")(input)?;
    let (input, seed_to_soil_map) = parse_map_triplets(input)?;

    let (input, _) = multispace1(input)?;
    let (input, _) = tag("soil-to-fertilizer map:\n")(input)?;
    let (input, soil_to_fertilizer_map) = parse_map_triplets(input)?;

    let (input, _) = multispace1(input)?;
    let (input, _) = tag("fertilizer-to-water map:\n")(input)?;
    let (input, fertilizer_to_water_map) = parse_map_triplets(input)?;

    let (input, _) = multispace1(input)?;
    let (input, _) = tag("water-to-light map:\n")(input)?;
    let (input, water_to_light_map) = parse_map_triplets(input)?;

    let (input, _) = multispace1(input)?;
    let (input, _) = tag("light-to-temperature map:\n")(input)?;
    let (input, light_to_temperature_map) = parse_map_triplets(input)?;

    let (input, _) = multispace1(input)?;
    let (input, _) = tag("temperature-to-humidity map:\n")(input)?;
    let (input, temperature_to_humidity_map) = parse_map_triplets(input)?;

    let (input, _) = multispace1(input)?;
    let (input, _) = tag("humidity-to-location map:\n")(input)?;
    let (input, humidity_to_location_map) = parse_map_triplets(input)?;

    Ok((
        input,
        Almanac {
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        },
    ))
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_almanac_test() {
        assert_eq!(
            parse_almanac(TEST_INPUT),
            Ok((
                "\n",
                Almanac {
                    seeds: vec![79, 14, 55, 13],
                    seed_to_soil_map: vec![
                        MapTriplet {
                            destination_range_start: 50,
                            source_range_start: 98,
                            range_length: 2,
                        },
                        MapTriplet {
                            destination_range_start: 52,
                            source_range_start: 50,
                            range_length: 48,
                        },
                    ],
                    soil_to_fertilizer_map: vec![
                        MapTriplet {
                            destination_range_start: 0,
                            source_range_start: 15,
                            range_length: 37,
                        },
                        MapTriplet {
                            destination_range_start: 37,
                            source_range_start: 52,
                            range_length: 2,
                        },
                        MapTriplet {
                            destination_range_start: 39,
                            source_range_start: 0,
                            range_length: 15,
                        },
                    ],
                    fertilizer_to_water_map: vec![
                        MapTriplet {
                            destination_range_start: 49,
                            source_range_start: 53,
                            range_length: 8,
                        },
                        MapTriplet {
                            destination_range_start: 0,
                            source_range_start: 11,
                            range_length: 42,
                        },
                        MapTriplet {
                            destination_range_start: 42,
                            source_range_start: 0,
                            range_length: 7,
                        },
                        MapTriplet {
                            destination_range_start: 57,
                            source_range_start: 7,
                            range_length: 4,
                        },
                    ],
                    water_to_light_map: vec![
                        MapTriplet {
                            destination_range_start: 88,
                            source_range_start: 18,
                            range_length: 7,
                        },
                        MapTriplet {
                            destination_range_start: 18,
                            source_range_start: 25,
                            range_length: 70,
                        },
                    ],
                    light_to_temperature_map: vec![
                        MapTriplet {
                            destination_range_start: 45,
                            source_range_start: 77,
                            range_length: 23,
                        },
                        MapTriplet {
                            destination_range_start: 81,
                            source_range_start: 45,
                            range_length: 19,
                        },
                        MapTriplet {
                            destination_range_start: 68,
                            source_range_start: 64,
                            range_length: 13,
                        },
                    ],
                    temperature_to_humidity_map: vec![
                        MapTriplet {
                            destination_range_start: 0,
                            source_range_start: 69,
                            range_length: 1,
                        },
                        MapTriplet {
                            destination_range_start: 1,
                            source_range_start: 0,
                            range_length: 69,
                        },
                    ],
                    humidity_to_location_map: vec![
                        MapTriplet {
                            destination_range_start: 60,
                            source_range_start: 56,
                            range_length: 37,
                        },
                        MapTriplet {
                            destination_range_start: 56,
                            source_range_start: 93,
                            range_length: 4,
                        },
                    ],
                }
            ))
        );
    }

    #[test]
    fn almanac_get_locations_test() {
        assert_eq!(
            parse_almanac(TEST_INPUT)
                .unwrap()
                .1
                .get_locations()
                .collect::<Vec<_>>(),
            vec![82, 43, 86, 35]
        );
    }
}
