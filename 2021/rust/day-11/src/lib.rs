pub mod bin;

use std::fmt;

/// A whole set of octopodes (more than one octopus).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Octopodes {
    /// Each element is the current energy of the octopus and whether it's flashed on this step.
    energies: [[(u8, bool); 10]; 10],

    /// Keep track of the total flashes.
    total_flashes: u32,
}

impl fmt::Debug for Octopodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "energies =\n")?;

        for row in self.energies.iter() {
            for &(energy, flashed) in row.iter() {
                if flashed {
                    write!(f, "+")?;
                } else {
                    write!(f, "{}", energy)?;
                }
            }
            write!(f, "\n")?;
        }

        write!(f, "total_flashes = {}", self.total_flashes)?;
        Ok(())
    }
}

impl Octopodes {
    /// Parse a set of octopodes from a string grid representation.
    pub fn parse(input: &str) -> Self {
        let energy_vec = input
            .lines()
            .map(|line| {
                let v = line
                    .chars()
                    .map(|c| (c.to_string().parse().unwrap(), false))
                    .collect::<Vec<(u8, bool)>>();

                let mut arr = [(0u8, false); 10];
                arr.copy_from_slice(&v[..10]);
                arr
            })
            .collect::<Vec<[(u8, bool); 10]>>();

        let mut energies = [[(0u8, false); 10]; 10];
        energies.copy_from_slice(&energy_vec);

        Self {
            energies,
            total_flashes: 0,
        }
    }

    /// Flash the appropriate octopodes and return whether or not new octopodes have flashed.
    fn do_flashes(&mut self) -> bool {
        let mut new_flashes = false;
        let mut indices_to_increment: Vec<(i8, i8)> = vec![];

        for row in 0..10 {
            for column in 0..10 {
                let (energy, flashed) = &mut self.energies[row][column];
                if *energy > 9 && !*flashed {
                    *flashed = true;
                    self.total_flashes += 1;
                    new_flashes = true;

                    let row = row as i8;
                    let column = column as i8;
                    indices_to_increment.push((row + 1, column + 1));
                    indices_to_increment.push((row + 1, column));
                    indices_to_increment.push((row + 1, column - 1));
                    indices_to_increment.push((row, column + 1));
                    indices_to_increment.push((row, column - 1));
                    indices_to_increment.push((row - 1, column + 1));
                    indices_to_increment.push((row - 1, column));
                    indices_to_increment.push((row - 1, column - 1));
                }
            }
        }

        for (row, column) in indices_to_increment
            .into_iter()
            .filter_map(|(row, column)| -> Option<(usize, usize)> {
                match (row.try_into(), column.try_into()) {
                    (Ok(row), Ok(column)) => Some((row, column)),
                    _ => None,
                }
            })
            .filter(|&(row, column)| row < 10 && column < 10)
        {
            self.energies[row][column].0 += 1;
        }

        new_flashes
    }

    /// Run a single step and return a bool to indicate whether all the octopodes have flashed.
    pub fn run_step(&mut self) -> bool {
        for row in self.energies.iter_mut() {
            for (energy, _) in row.iter_mut() {
                *energy += 1;
            }
        }

        // All the logic happens in this function.
        while self.do_flashes() {}

        let mut total_flashed = 0;

        for row in self.energies.iter_mut() {
            for (energy, flashed) in row.iter_mut() {
                if *flashed {
                    total_flashed += 1;
                    *energy = 0;
                    *flashed = false;
                }
            }
        }

        total_flashed == 100
    }

    pub fn total_flashes(&self) -> u32 {
        self.total_flashes
    }
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn octopodes_parse_test() {
        assert_eq!(
            Octopodes::parse(TEST_INPUT),
            Octopodes {
                #[rustfmt::skip]
                energies: [
                    [
                        (5, false), (4, false), (8, false), (3, false), (1, false),
                        (4, false), (3, false), (2, false), (2, false), (3, false)
                    ],
                    [
                        (2, false), (7, false), (4, false), (5, false), (8, false),
                        (5, false), (4, false), (7, false), (1, false), (1, false)
                    ],
                    [
                        (5, false), (2, false), (6, false), (4, false), (5, false),
                        (5, false), (6, false), (1, false), (7, false), (3, false)
                    ],
                    [
                        (6, false), (1, false), (4, false), (1, false), (3, false),
                        (3, false), (6, false), (1, false), (4, false), (6, false)
                    ],
                    [
                        (6, false), (3, false), (5, false), (7, false), (3, false),
                        (8, false), (5, false), (4, false), (7, false), (8, false)
                    ],
                    [
                        (4, false), (1, false), (6, false), (7, false), (5, false),
                        (2, false), (4, false), (6, false), (4, false), (5, false)
                    ],
                    [
                        (2, false), (1, false), (7, false), (6, false), (8, false),
                        (4, false), (1, false), (7, false), (2, false), (1, false)
                    ],
                    [
                        (6, false), (8, false), (8, false), (2, false), (8, false),
                        (8, false), (1, false), (1, false), (3, false), (4, false)
                    ],
                    [
                        (4, false), (8, false), (4, false), (6, false), (8, false),
                        (4, false), (8, false), (5, false), (5, false), (4, false)
                    ],
                    [
                        (5, false), (2, false), (8, false), (3, false), (7, false),
                        (5, false), (1, false), (5, false), (2, false), (6, false)
                    ]
                ],
                total_flashes: 0
            }
        );
    }

    #[test]
    fn output_after_each_step_test() {
        fn get_debug<T: fmt::Debug>(thing: T) -> String {
            use std::fmt::Write;

            let mut s = String::new();
            write!(&mut s, "{:?}", thing).unwrap();
            s
        }

        let mut octopodes = Octopodes::parse(TEST_INPUT);

        // Step 1
        octopodes.run_step();
        assert_eq!(
            get_debug(octopodes),
            r#"energies =
6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637
total_flashes = 0"#,
            "Step 1"
        );

        // Step 2
        octopodes.run_step();
        assert_eq!(
            get_debug(octopodes),
            r#"energies =
8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848
total_flashes = 35"#,
            "Step 2"
        );

        // Step 3
        octopodes.run_step();
        assert_eq!(
            get_debug(octopodes),
            r#"energies =
0050900866
8500800575
9900000039
9700000041
9935080063
7712300000
7911250009
2211130000
0421125000
0021119000
total_flashes = 80"#,
            "Step 3"
        );

        // Step 4
        octopodes.run_step();
        assert_eq!(
            get_debug(octopodes),
            r#"energies =
2263031977
0923031697
0032221150
0041111163
0076191174
0053411122
0042361120
5532241122
1532247211
1132230211
total_flashes = 96"#,
            "Step 4"
        );

        // Step 5
        octopodes.run_step();
        assert_eq!(
            get_debug(octopodes),
            r#"energies =
4484144000
2044144000
2253333493
1152333274
1187303285
1164633233
1153472231
6643352233
2643358322
2243341322
total_flashes = 104"#,
            "Step 5"
        );

        // Step 6
        octopodes.run_step();
        assert_eq!(
            get_debug(octopodes),
            r#"energies =
5595255111
3155255222
3364444605
2263444496
2298414396
2275744344
2264583342
7754463344
3754469433
3354452433
total_flashes = 105"#,
            "Step 6"
        );

        // Step 7
        octopodes.run_step();
        assert_eq!(
            get_debug(octopodes),
            r#"energies =
6707366222
4377366333
4475555827
3496655709
3500625609
3509955566
3486694453
8865585555
4865580644
4465574644
total_flashes = 112"#,
            "Step 7"
        );

        // Step 8
        octopodes.run_step();
        assert_eq!(
            get_debug(octopodes),
            r#"energies =
7818477333
5488477444
5697666949
4608766830
4734946730
4740097688
6900007564
0000009666
8000004755
6800007755
total_flashes = 136"#,
            "Step 8"
        );

        // Step 9
        octopodes.run_step();
        assert_eq!(
            get_debug(octopodes),
            r#"energies =
9060000644
7800000976
6900000080
5840000082
5858000093
6962400000
8021250009
2221130009
9111128097
7911119976
total_flashes = 175"#,
            "Step 9"
        );

        // Step 10
        octopodes.run_step();
        assert_eq!(
            get_debug(octopodes),
            r#"energies =
0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000
total_flashes = 204"#,
            "Step 10"
        );
    }
}
