pub mod bin;

pub fn parse_equations(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let mut line_split = line.split(": ");
            let target = line_split.next().unwrap().parse().unwrap();
            let nums = line_split
                .next()
                .unwrap()
                .split(" ")
                .map(|n| n.parse().unwrap())
                .collect();
            (target, nums)
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn operator_permutations_simple(length: u32) -> Vec<Vec<Operator>> {
    if length == 0 {
        vec![vec![]]
    } else {
        let n_minus_one = operator_permutations_simple(length - 1);
        let mut new_perms = Vec::with_capacity(2usize.pow(length));

        for perm in n_minus_one {
            let mut v = Vec::with_capacity(length as usize);

            v.push(Operator::Add);
            v.extend(perm);
            new_perms.push(v.clone());

            v[0] = Operator::Multiply;
            new_perms.push(v);
        }

        new_perms
    }
}

fn operator_permutations_with_concat(length: u32) -> Vec<Vec<Operator>> {
    if length == 0 {
        vec![vec![]]
    } else {
        let n_minus_one = operator_permutations_with_concat(length - 1);
        let mut new_perms = Vec::with_capacity(2usize.pow(length));

        for perm in n_minus_one {
            let mut v = Vec::with_capacity(length as usize);

            v.push(Operator::Add);
            v.extend(perm);
            new_perms.push(v.clone());

            v[0] = Operator::Multiply;
            new_perms.push(v.clone());

            v[0] = Operator::Concatenate;
            new_perms.push(v);
        }

        new_perms
    }
}

fn does_solve_equation(target: i64, nums: &[i64], operators: &[Operator]) -> bool {
    assert_eq!(
        operators.len() + 1,
        nums.len(),
        "We must have one more number than we do operators"
    );
    assert!(!nums.is_empty(), "We must have at least one number");

    let mut acc = *nums.first().unwrap();

    for (&n, operator) in nums.iter().skip(1).zip(operators) {
        match operator {
            Operator::Add => acc += n,
            Operator::Multiply => acc *= n,
            Operator::Concatenate => acc = acc * 10i64.pow(n.ilog10() + 1) + n,
        }

        if acc > target {
            return false;
        }
    }

    acc == target
}

pub fn ways_to_solve_simple(target: i64, nums: &[i64]) -> usize {
    operator_permutations_simple(nums.len() as u32 - 1)
        .into_iter()
        .filter(|perm| does_solve_equation(target, nums, perm))
        .count()
}

pub fn ways_to_solve_with_concat(target: i64, nums: &[i64]) -> usize {
    operator_permutations_with_concat(nums.len() as u32 - 1)
        .into_iter()
        .filter(|perm| does_solve_equation(target, nums, perm))
        .count()
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_equations() {
        assert_eq!(
            parse_equations(TEST_INPUT),
            vec![
                (190, vec![10, 19]),
                (3267, vec![81, 40, 27]),
                (83, vec![17, 5]),
                (156, vec![15, 6]),
                (7290, vec![6, 8, 6, 15]),
                (161011, vec![16, 10, 13]),
                (192, vec![17, 8, 14]),
                (21037, vec![9, 7, 18, 13]),
                (292, vec![11, 6, 16, 20]),
            ]
        );
    }

    #[test]
    fn test_operator_permutations() {
        use Operator::*;

        assert_eq!(operator_permutations_simple(0), vec![vec![]]);
        assert_eq!(
            operator_permutations_simple(1),
            vec![vec![Add], vec![Multiply]]
        );
        assert_eq!(
            operator_permutations_simple(2),
            vec![
                vec![Add, Add],
                vec![Multiply, Add],
                vec![Add, Multiply],
                vec![Multiply, Multiply],
            ]
        );

        assert_eq!(operator_permutations_with_concat(0), vec![vec![]]);
        assert_eq!(
            operator_permutations_with_concat(1),
            vec![vec![Add], vec![Multiply], vec![Concatenate]]
        );
        assert_eq!(
            operator_permutations_with_concat(2),
            vec![
                vec![Add, Add],
                vec![Multiply, Add],
                vec![Concatenate, Add],
                vec![Add, Multiply],
                vec![Multiply, Multiply],
                vec![Concatenate, Multiply],
                vec![Add, Concatenate],
                vec![Multiply, Concatenate],
                vec![Concatenate, Concatenate],
            ]
        );
    }

    #[test]
    fn test_ways_to_solve() {
        assert_eq!(ways_to_solve_simple(190, &[10, 19]), 1);
        assert_eq!(ways_to_solve_simple(3267, &[81, 40, 27]), 2);
        assert_eq!(ways_to_solve_simple(292, &[11, 6, 16, 20]), 1);

        assert_eq!(ways_to_solve_with_concat(156, &[15, 6]), 1);
        assert_eq!(ways_to_solve_with_concat(7290, &[6, 8, 6, 15]), 1);
        assert_eq!(ways_to_solve_with_concat(192, &[17, 8, 14]), 1);
    }
}
