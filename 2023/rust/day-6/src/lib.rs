use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace0, multispace1, newline},
    multi::separated_list1,
    IResult,
};

pub mod bin;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    /// Return an iterator over all the button hold lengths that would result in a win.
    pub fn ways_to_win(&self) -> impl Iterator<Item = u64> + '_ {
        (0..=self.time)
            .filter_map(|time| (time * (self.time - time) > self.record_distance).then_some(time))
    }
}

pub fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, times) = separated_list1(multispace1, complete::u64)(input)?;
    let (input, _) = newline(input)?;

    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, distances) = separated_list1(multispace1, complete::u64)(input)?;

    debug_assert_eq!(
        times.len(),
        distances.len(),
        "We must have the same number of times and distances"
    );

    let races = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, record_distance)| Race {
            time,
            record_distance,
        })
        .collect();

    Ok((input, races))
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_races_test() {
        assert_eq!(
            parse_races(TEST_INPUT),
            Ok((
                "\n",
                vec![
                    Race {
                        time: 7,
                        record_distance: 9
                    },
                    Race {
                        time: 15,
                        record_distance: 40
                    },
                    Race {
                        time: 30,
                        record_distance: 200
                    },
                ]
            ))
        );
    }

    #[test]
    fn ways_to_win_test() {
        assert_eq!(
            parse_races(TEST_INPUT)
                .unwrap()
                .1
                .into_iter()
                .map(|race| race.ways_to_win().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            vec![
                vec![2, 3, 4, 5],
                vec![4, 5, 6, 7, 8, 9, 10, 11],
                vec![11, 12, 13, 14, 15, 16, 17, 18, 19],
            ]
        );
    }
}
