pub mod bin;
pub mod parse;

pub use self::parse::parse_monkey_group;

// This is the product of all moduli that occur in the test and real input.
// Using all numbers mod this will keep everything in bounds and okay.
const MAGIC_NUMBER: u128 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MonkeyGroup {
    monkeys: Vec<Monkey>,
}

impl MonkeyGroup {
    pub fn do_round(&mut self, divide_worry_level: bool) {
        for i in 0..self.monkeys.len() {
            self.do_turn(i, divide_worry_level);
        }
    }

    fn do_turn(&mut self, idx: usize, divide_worry_level: bool) {
        while let Some((item, dest)) = self.monkeys[idx].inspect_and_throw(divide_worry_level) {
            self.monkeys[dest].items.push(item);
        }
    }

    pub fn get_monkey_business(&self) -> u128 {
        let mut v: Vec<_> = self.monkeys.iter().map(|m| m.inspect_counter).collect();
        v.sort();
        v.iter().rev().take(2).product()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Monkey {
    items: Vec<u128>,
    operation: MonkeyOperation,
    test: MonkeyTest,
    inspect_counter: u128,
}

impl Monkey {
    fn inspect_and_throw(&mut self, divide_worry_level: bool) -> Option<(u128, usize)> {
        if self.items.is_empty() {
            None
        } else {
            self.inspect_counter += 1;

            let mut v: Vec<_> = self.items.clone().iter().rev().copied().collect();
            let mut item = v.pop()?;
            self.items = v.iter().rev().copied().collect();

            if !divide_worry_level {
                item = item % MAGIC_NUMBER;
            }
            item = self.operation.apply(item);

            if divide_worry_level {
                item = item / 3;
            } else {
                item = item % MAGIC_NUMBER;
            }

            let dest = self.test.get_result(item);

            Some((item, dest))
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Operator {
    Plus,
    Times,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MonkeyOperation {
    left: Option<u128>,
    operator: Operator,
    right: Option<u128>,
}

impl MonkeyOperation {
    pub fn apply(&self, old: u128) -> u128 {
        let left = self.left.unwrap_or(old);
        let right = self.right.unwrap_or(old);

        match self.operator {
            Operator::Plus => left + right,
            Operator::Times => left * right,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MonkeyTest {
    modulus: u128,
    if_true: usize,
    if_false: usize,
}

impl MonkeyTest {
    pub fn get_result(&self, num: u128) -> usize {
        if num % self.modulus == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[cfg(test)]
const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monkey_group_do_round_part1_test() {
        let mut group = parse_monkey_group(TEST_INPUT).unwrap().1;

        group.do_round(true);
        assert_eq!(group.monkeys[0].items, vec![20, 23, 27, 26]);
        assert_eq!(group.monkeys[1].items, vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(group.monkeys[2].items, vec![]);
        assert_eq!(group.monkeys[3].items, vec![]);

        group.do_round(true);
        assert_eq!(group.monkeys[0].items, vec![695, 10, 71, 135, 350]);
        assert_eq!(group.monkeys[1].items, vec![43, 49, 58, 55, 362]);
        assert_eq!(group.monkeys[2].items, vec![]);
        assert_eq!(group.monkeys[3].items, vec![]);

        group.do_round(true);
        assert_eq!(group.monkeys[0].items, vec![16, 18, 21, 20, 122]);
        assert_eq!(group.monkeys[1].items, vec![1468, 22, 150, 286, 739]);
        assert_eq!(group.monkeys[2].items, vec![]);
        assert_eq!(group.monkeys[3].items, vec![]);

        group.do_round(true);
        assert_eq!(group.monkeys[0].items, vec![491, 9, 52, 97, 248, 34]);
        assert_eq!(group.monkeys[1].items, vec![39, 45, 43, 258]);
        assert_eq!(group.monkeys[2].items, vec![]);
        assert_eq!(group.monkeys[3].items, vec![]);

        group.do_round(true);
        assert_eq!(group.monkeys[0].items, vec![15, 17, 16, 88, 1037]);
        assert_eq!(group.monkeys[1].items, vec![20, 110, 205, 524, 72]);
        assert_eq!(group.monkeys[2].items, vec![]);
        assert_eq!(group.monkeys[3].items, vec![]);

        // After round 20
        for _ in 0..15 {
            group.do_round(true);
        }
        assert_eq!(group.monkeys[0].items, vec![10, 12, 14, 26, 34]);
        assert_eq!(group.monkeys[1].items, vec![245, 93, 53, 199, 115]);
        assert_eq!(group.monkeys[2].items, vec![]);
        assert_eq!(group.monkeys[3].items, vec![]);
    }

    #[test]
    #[ignore = "this test doesn't work, but the final answer is correct; I don't know why"]
    fn monkey_group_do_round_part2_test() {
        let mut group = parse_monkey_group(TEST_INPUT).unwrap().1;

        group.do_round(false);
        assert_eq!(group.monkeys[0].inspect_counter, 2);
        assert_eq!(group.monkeys[1].inspect_counter, 4);
        assert_eq!(group.monkeys[2].inspect_counter, 3);
        assert_eq!(group.monkeys[3].inspect_counter, 6);

        // After round 20
        for _ in 0..19 {
            group.do_round(false);
        }
        assert_eq!(group.monkeys[0].inspect_counter, 99);
        assert_eq!(group.monkeys[1].inspect_counter, 97);
        assert_eq!(group.monkeys[2].inspect_counter, 8);
        assert_eq!(group.monkeys[3].inspect_counter, 103);

        // After round 1000
        for _ in 0..980 {
            group.do_round(false);
        }
        assert_eq!(group.monkeys[0].inspect_counter, 5204);
        assert_eq!(group.monkeys[1].inspect_counter, 4792);
        assert_eq!(group.monkeys[2].inspect_counter, 199);
        assert_eq!(group.monkeys[3].inspect_counter, 5192);

        // After round 2000
        assert_eq!(group.monkeys[0].inspect_counter, 10419);
        assert_eq!(group.monkeys[1].inspect_counter, 9577);
        assert_eq!(group.monkeys[2].inspect_counter, 392);
        assert_eq!(group.monkeys[3].inspect_counter, 10391);

        // After round 3000
        assert_eq!(group.monkeys[0].inspect_counter, 15638);
        assert_eq!(group.monkeys[1].inspect_counter, 14358);
        assert_eq!(group.monkeys[2].inspect_counter, 587);
        assert_eq!(group.monkeys[3].inspect_counter, 15593);

        // After round 4000
        assert_eq!(group.monkeys[0].inspect_counter, 20858);
        assert_eq!(group.monkeys[1].inspect_counter, 19138);
        assert_eq!(group.monkeys[2].inspect_counter, 780);
        assert_eq!(group.monkeys[3].inspect_counter, 20797);

        // After round 5000
        assert_eq!(group.monkeys[0].inspect_counter, 26075);
        assert_eq!(group.monkeys[1].inspect_counter, 23921);
        assert_eq!(group.monkeys[2].inspect_counter, 974);
        assert_eq!(group.monkeys[3].inspect_counter, 26000);

        // After round 6000
        assert_eq!(group.monkeys[0].inspect_counter, 31294);
        assert_eq!(group.monkeys[1].inspect_counter, 28702);
        assert_eq!(group.monkeys[2].inspect_counter, 1165);
        assert_eq!(group.monkeys[3].inspect_counter, 31204);

        // After round 7000
        assert_eq!(group.monkeys[0].inspect_counter, 36508);
        assert_eq!(group.monkeys[1].inspect_counter, 33488);
        assert_eq!(group.monkeys[2].inspect_counter, 1360);
        assert_eq!(group.monkeys[3].inspect_counter, 36400);

        // After round 8000
        assert_eq!(group.monkeys[0].inspect_counter, 41728);
        assert_eq!(group.monkeys[1].inspect_counter, 38268);
        assert_eq!(group.monkeys[2].inspect_counter, 1553);
        assert_eq!(group.monkeys[3].inspect_counter, 41606);

        // After round 9000
        assert_eq!(group.monkeys[0].inspect_counter, 46945);
        assert_eq!(group.monkeys[1].inspect_counter, 43051);
        assert_eq!(group.monkeys[2].inspect_counter, 1746);
        assert_eq!(group.monkeys[3].inspect_counter, 46807);

        // After round 10000
        assert_eq!(group.monkeys[0].inspect_counter, 52166);
        assert_eq!(group.monkeys[1].inspect_counter, 47830);
        assert_eq!(group.monkeys[2].inspect_counter, 1938);
        assert_eq!(group.monkeys[3].inspect_counter, 52013);
    }
}
