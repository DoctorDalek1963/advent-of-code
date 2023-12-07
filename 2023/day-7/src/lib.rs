use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use std::collections::HashMap;

pub mod bin;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_counts(counts: &[u8]) -> Self {
        debug_assert!(counts.iter().sum::<u8>() == 5, "We need exactly 5 cards");

        match &counts[..] {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!("We have covered all possible card arrangements"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hand([Card; 5]);

impl Hand {
    fn get_card_map(&self) -> HashMap<Card, u8> {
        let mut map = HashMap::new();
        for card in self.0 {
            map.entry(card).and_modify(|count| *count += 1).or_insert(1);
        }
        map
    }

    pub fn get_type(&self) -> HandType {
        let counts = self.get_card_map().into_values().sorted().collect_vec();

        HandType::from_counts(&counts)
    }

    pub fn get_type_with_joker_rule(&self) -> HandType {
        let map = self.get_card_map();
        let mut non_joker_counts = map
            .iter()
            .filter_map(|(&card, &count)| (card != Card::Joker).then_some(count))
            .sorted()
            .collect_vec();

        // `non_joker_counts` is sorted, so the last element is the largest
        match non_joker_counts.last_mut() {
            Some(highest) => {
                *highest += *map.get(&Card::Joker).unwrap_or(&0);
                HandType::from_counts(&non_joker_counts)
            }
            None => {
                // No elements in `non_joker_counts` means we have 5 jokers
                HandType::FiveOfAKind
            }
        }
    }
}

fn parse_card(input: &str, joker: bool) -> IResult<&str, Card> {
    let (input, card) = alt((
        tag("A").map(|_| Card::Ace),
        tag("K").map(|_| Card::King),
        tag("Q").map(|_| Card::Queen),
        tag("J").map(|_| if joker { Card::Joker } else { Card::Jack }),
        tag("T").map(|_| Card::Ten),
        tag("9").map(|_| Card::Nine),
        tag("8").map(|_| Card::Eight),
        tag("7").map(|_| Card::Seven),
        tag("6").map(|_| Card::Six),
        tag("5").map(|_| Card::Five),
        tag("4").map(|_| Card::Four),
        tag("3").map(|_| Card::Three),
        tag("2").map(|_| Card::Two),
    ))(input)?;
    Ok((input, card))
}

fn parse_hand(input: &str, joker: bool) -> IResult<&str, Hand> {
    let (input, card1) = parse_card(input, joker)?;
    let (input, card2) = parse_card(input, joker)?;
    let (input, card3) = parse_card(input, joker)?;
    let (input, card4) = parse_card(input, joker)?;
    let (input, card5) = parse_card(input, joker)?;
    Ok((input, Hand([card1, card2, card3, card4, card5])))
}

pub fn parse_hands_and_bids(input: &str, joker: bool) -> IResult<&str, Vec<(Hand, u16)>> {
    separated_list1(
        newline,
        separated_pair(|inp| parse_hand(inp, joker), multispace1, complete::u16),
    )(input)
}

#[cfg(test)]
pub const TEST_INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ordering_test() {
        assert!(Card::Ace > Card::King);
        assert!(Card::King > Card::Queen);
        assert!(Card::Queen > Card::Jack);
        assert!(Card::Jack > Card::Ten);
        assert!(Card::Ten > Card::Nine);
        assert!(Card::Nine > Card::Eight);
        assert!(Card::Eight > Card::Seven);
        assert!(Card::Seven > Card::Six);
        assert!(Card::Six > Card::Five);
        assert!(Card::Five > Card::Four);
        assert!(Card::Four > Card::Three);
        assert!(Card::Three > Card::Two);
        assert!(Card::Two > Card::Joker);

        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
        assert!(HandType::FourOfAKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::ThreeOfAKind);
        assert!(HandType::ThreeOfAKind > HandType::TwoPair);
        assert!(HandType::TwoPair > HandType::OnePair);
        assert!(HandType::OnePair > HandType::HighCard);
    }

    #[test]
    fn parse_hands_and_bids_test() {
        use Card::*;
        assert_eq!(
            parse_hands_and_bids(TEST_INPUT, false),
            Ok((
                "\n",
                vec![
                    (Hand([Three, Two, Ten, Three, King]), 765),
                    (Hand([Ten, Five, Five, Jack, Five]), 684),
                    (Hand([King, King, Six, Seven, Seven]), 28),
                    (Hand([King, Ten, Jack, Jack, Ten]), 220),
                    (Hand([Queen, Queen, Queen, Jack, Ace]), 483),
                ]
            ))
        );
    }

    #[test]
    fn joker_rule_test() {
        use HandType::*;

        let inputs = [
            ("32T3K", OnePair),
            ("T55J5", FourOfAKind),
            ("KK677", TwoPair),
            ("KTJJT", FourOfAKind),
            ("QQQJA", FourOfAKind),
            ("AAJJJ", FiveOfAKind),
            ("J67KQ", OnePair),
            ("22J33", FullHouse),
        ];

        for (cards, hand_type) in inputs {
            assert_eq!(
                parse_hand(cards, true)
                    .unwrap()
                    .1
                    .get_type_with_joker_rule(),
                hand_type,
                "Expected {hand_type:?} with cards {cards}"
            );
        }
    }
}
