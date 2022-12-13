pub mod bin;
mod parse;

use std::cmp::Ordering;

pub use self::parse::parse_packet_pairs;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PacketComp {
    Int(u32),
    List(Vec<PacketComp>),
}

impl PartialOrd for PacketComp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketComp {
    fn cmp(&self, other: &Self) -> Ordering {
        use PacketComp::*;

        match (self, other) {
            (Int(left), Int(right)) => left.cmp(right),
            (List(_), Int(_)) => self.cmp(&List(vec![other.clone()])),
            (Int(_), List(_)) => List(vec![self.clone()]).cmp(other),
            (List(left), List(right)) => {
                let len = left.len().min(right.len());
                for i in 0..len {
                    let x = left[i].cmp(&right[i]);
                    if x == Ordering::Less || x == Ordering::Greater {
                        return x;
                    }
                }

                left.len().cmp(&right.len())
            }
        }
    }
}

#[cfg(test)]
pub const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_ord_packet_cmp_test() {
        let pairs = parse_packet_pairs(TEST_INPUT).unwrap().1;

        let (l, r) = &pairs[0];
        assert!(l < r);

        let (l, r) = &pairs[1];
        assert!(l < r);

        let (l, r) = &pairs[2];
        assert!(l > r);

        let (l, r) = &pairs[3];
        assert!(l < r);

        let (l, r) = &pairs[4];
        assert!(l > r);

        let (l, r) = &pairs[5];
        assert!(l < r);

        let (l, r) = &pairs[6];
        assert!(l > r);

        let (l, r) = &pairs[7];
        assert!(l > r);
    }
}
