use nom::{
    branch::alt,
    character::complete::{char, newline},
    multi::{many1, separated_list1},
    IResult,
};

pub mod bin;

type BitArray<const N: usize> = [bool; N];

pub fn parse_bit_arrays<const N: usize>(input: &str) -> IResult<&str, Vec<BitArray<N>>> {
    fn single_bit(input: &str) -> IResult<&str, bool> {
        let (input, bit) = alt((char('0'), char('1')))(input)?;
        let bit = match bit {
            '0' => false,
            '1' => true,
            _ => unreachable!(),
        };
        Ok((input, bit))
    }

    fn bit_array<const N: usize>(input: &str) -> IResult<&str, BitArray<N>> {
        let mut bit_array = [false; N];

        let (input, bit_vec) = many1(single_bit)(input)?;
        bit_array.copy_from_slice(&bit_vec[..]);

        Ok((input, bit_array))
    }

    separated_list1(newline, bit_array)(input)
}

fn bit_array_to_number<const N: usize>(array: BitArray<N>) -> u32 {
    let mut num: u32 = 0;

    for (idx, &val) in array.iter().rev().enumerate() {
        if val {
            num += u32::pow(2, idx as u32);
        }
    }

    num
}

pub fn find_power_consumption<const N: usize>(arrays: Vec<BitArray<N>>) -> u32 {
    let gamma_rate = {
        let mut arr = [false; N];
        for i in 0..N {
            let truth_count: usize = arrays.iter().map(|s| s[i]).filter(|&e| e).count();
            arr[i] = truth_count as usize > arrays.len() / 2;
        }

        bit_array_to_number(arr)
    };

    gamma_rate * (gamma_rate ^ bit_array_to_number([true; N]))
}

fn find_oxygen_generator_rating_as_bit_array<const N: usize>(arrays: &Vec<BitArray<N>>) -> u32 {
    let mut vec = arrays.clone();

    for i in 0..N {
        let truth_count = vec.iter().map(|s| s[i]).filter(|&e| e).count();
        let most_common = if truth_count * 2 == vec.len() {
            true
        } else {
            truth_count > vec.len() / 2
        };
        vec = vec
            .iter()
            .filter(|&s| s[i] == most_common)
            .copied()
            .collect();

        if vec.len() == 1 {
            break;
        }
    }

    assert!(vec.len() == 1);
    bit_array_to_number(*vec.get(0).unwrap())
}

fn find_co2_scrubber_rating<const N: usize>(arrays: &Vec<BitArray<N>>) -> u32 {
    let mut vec = arrays.clone();

    for i in 0..N {
        let truth_count = vec.iter().map(|s| s[i]).filter(|&e| e).count();
        let least_common = if truth_count * 2 == vec.len() {
            false
        } else {
            truth_count <= vec.len() / 2
        };
        vec = vec
            .iter()
            .filter(|&s| s[i] == least_common)
            .copied()
            .collect();

        if vec.len() == 1 {
            break;
        }
    }

    assert!(vec.len() == 1);
    bit_array_to_number(*vec.get(0).unwrap())
}

pub fn find_life_support_rating<const N: usize>(arrays: Vec<BitArray<N>>) -> u32 {
    find_oxygen_generator_rating_as_bit_array(&arrays) * find_co2_scrubber_rating(&arrays)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHORT_INPUT: &str = "00100
11110
10110
";

    const FULL_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn parse_bit_arrays_test() {
        let bit_arrays = vec![
            [false, false, true, false, false],
            [true, true, true, true, false],
            [true, false, true, true, false],
        ];

        assert_eq!(parse_bit_arrays(SHORT_INPUT), Ok(("\n", bit_arrays)));
    }

    #[test]
    fn bit_array_to_number_test() {
        assert_eq!(
            parse_bit_arrays::<5>(SHORT_INPUT)
                .unwrap()
                .1
                .iter()
                .copied()
                .map(bit_array_to_number)
                .collect::<Vec<_>>(),
            vec![4, 30, 22]
        );
    }

    #[test]
    fn find_oxygen_generator_rating_test() {
        assert_eq!(
            find_oxygen_generator_rating_as_bit_array(
                &parse_bit_arrays::<5>(FULL_INPUT).unwrap().1
            ),
            23
        );
    }

    #[test]
    fn find_co2_scrubber_rating_test() {
        assert_eq!(
            find_co2_scrubber_rating(&parse_bit_arrays::<5>(FULL_INPUT).unwrap().1),
            10
        );
    }
}
