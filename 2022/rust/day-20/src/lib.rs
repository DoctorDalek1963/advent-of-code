pub mod bin;

use tracing::{debug, instrument};

pub fn parse_list(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.split("\n").filter_map(|s| s.parse::<i32>().ok())
}

#[instrument(skip(list))]
pub fn mix_list(list: impl Iterator<Item = i32>) -> Vec<i32> {
    let original: Vec<_> = list.enumerate().collect();
    let mut state = original.clone();
    let len = original.len() as i32;

    debug!(?original);
    debug!(?len);

    for (id, offset) in original {
        let idx_in_state = state
            .iter()
            .position(|&(state_id, _)| state_id == id)
            .expect(&format!("id {:?} should be in the state list", id));
        debug!(?id, ?idx_in_state);
        debug!(?state);

        let element = state.remove(idx_in_state);

        let new_index = {
            let idx_in_state = idx_in_state as i32;
            let sum = offset + idx_in_state;

            (if sum > 0 && sum < len {
                sum
            } else if sum >= len {
                (sum + 1).rem_euclid(len)
            } else if sum < 0 {
                (sum - 1).rem_euclid(len)
            } else if sum == 0 {
                len - 1
            } else {
                unreachable!(
                    "unable to get new_index with offset={offset} and idx_in_state={idx_in_state}"
                );
            }) as usize
        };
        debug!(?new_index);

        state.insert(new_index, element);
        debug!(?element, ?state, "after re-inserting");
    }

    assert_eq!(state.len() as i32, len);
    state.iter().map(|&(_, n)| n).collect()
}

#[cfg(test)]
pub const TEST_INPUT: &str = "1
2
-3
3
-2
0
4
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_list_test() {
        assert_eq!(
            parse_list(TEST_INPUT).collect::<Vec<_>>(),
            vec![1, 2, -3, 3, -2, 0, 4]
        );
    }

    #[test]
    fn mix_list_test() {
        let _ = tracing_subscriber::fmt::try_init();
        assert_eq!(
            mix_list(parse_list(TEST_INPUT)),
            vec![1, 2, -3, 4, 0, 3, -2]
        );
    }
}
