use std::cmp::Ordering;
use std::collections::HashMap;

const CARDS: &'static [char] = &[
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'
];
const JOKER: u8 = 0;
const CARDS_WITH_JOKER: &'static [char] = &[
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: &Cards) -> Self {
        use HandType::*;

        let mut cards_with_count = HashMap::new();
        for card in cards {
            cards_with_count.entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let mut cards_with_count = cards_with_count.iter()
            .map(|e| (e.0, e.1))
            .collect::<Vec<_>>();
        cards_with_count.sort_by(|a, b| b.1.cmp(a.1));
        match cards_with_count.len() {
            1 => FiveOfAKind,
            2 => {
                if *cards_with_count[0].1 == 4 {
                    FourOfAKind
                } else {
                    FullHouse
                }
            }
            3 => {
                if *cards_with_count[0].1 == 3 {
                    ThreeOfAKind
                } else {
                    TwoPair
                }
            }
            4 => OnePair,
            5 => HighCard,
            _ => unreachable!("More than 5 cards"),
        }
    }
}

type Cards = [u8; 5];

fn promote_jokers(cards: &Cards) -> Cards {
    use HandType::*;

    let mut cards_with_count = HashMap::new();
    for card in cards {
        cards_with_count.entry(card)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let num_jokers = if let Some(num_jokers) = cards_with_count.remove(&JOKER) {
        num_jokers
    } else {
        return *cards;
    };

    log::debug!("Card counts: {cards_with_count:?}");
    let top_card = cards_with_count.iter()
        .map(|e| (e.0, e.1))
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(c, _)| c);
    if let Some(top_card) = top_card {
        log::debug!("Top card: {top_card}");
        cards.map(|c| if c == JOKER { **top_card } else { c })
    } else {
        *cards
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    pub cards: Cards,
    pub hand_type: HandType,
    pub bid: u64,
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    if card > other_card {
                        return Ordering::Greater;
                    } else if card < other_card {
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct UpgradedHand {
    pub cards: Cards,
    pub origin: Hand,
}

impl PartialOrd<Hand> for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_card_ordinals(cards_order: &[char]) -> HashMap<char, u8> {
    let mut card_to_ord = HashMap::new();
    for (ord, card) in cards_order.iter().enumerate() {
        card_to_ord.insert(*card, ord as u8);
    }
    card_to_ord
}

fn parse_cards(s: &str, card_ords: &HashMap<char, u8>) -> Cards {
    s.chars()
     .map(|c| card_ords[&c] as u8)
     .collect::<Vec<_>>()
        .try_into()
        .expect("5 cards in hand")
}

pub fn solve_1(input: &str) -> String {
    let card_ordinals = build_card_ordinals(CARDS);

    let lines = input.lines();
    let mut hands = vec!();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((cards_str, bid_str)) = line.split_once(' ') {
            let cards_str = cards_str.trim();
            let bid_str = bid_str.trim();
            let bid = bid_str.parse().expect("bid number");
            let cards = parse_cards(cards_str, &card_ordinals);
            let hand_type = HandType::from_cards(&cards);
            hands.push(
                Hand { cards, hand_type, bid, }
            );
        }
    }
    hands.sort();

    let mut res = 0;
    for (score, hand) in hands.iter().enumerate() {
        res += hand.bid * (score as u64 + 1);
    }
    res.to_string()
}

pub fn solve_2(input: &str) -> String {
    let card_ordinals = build_card_ordinals(CARDS_WITH_JOKER);

    let lines = input.lines();
    let mut hands = vec!();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((cards_str, bid_str)) = line.split_once(' ') {
            let cards_str = cards_str.trim();
            let bid_str = bid_str.trim();
            let bid = bid_str.parse().expect("bid number");
            let cards = parse_cards(cards_str, &card_ordinals);
            let upgraded_cards = promote_jokers(&cards);
            let hand_type = HandType::from_cards(&upgraded_cards);
            hands.push(
                UpgradedHand {
                    cards: upgraded_cards,
                    origin: Hand { cards, hand_type, bid }
                }
            );
        }
    }

    hands.sort_by(|a, b| a.origin.cmp(&b.origin));

    let mut res = 0;
    for (score, hand) in hands.iter().enumerate() {
        res += hand.origin.bid * (score as u64 + 1);
    }
    res.to_string()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use test_log::test;
    use crate::util;
    use super::*;

    const EXAMPLE_INPUT: &'static str = indoc!{"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn test_solve_1() {
        assert_eq!(
            solve_1(EXAMPLE_INPUT),
            "6440".to_string()
        );
    }

    #[test]
    fn solve_1_with_user_input() -> Result<(), anyhow::Error> {
        let day = util::day_from_filename(file!())?;
        let input = if let Some(input) = util::fetch_user_input(day)? {
            input
        } else {
            return Ok(());
        };

        log::warn!("{}", solve_1(&input));
        Ok(())
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(
            solve_2(EXAMPLE_INPUT),
            "5905".to_string()
        );
    }

    #[test]
    fn solve_2_with_user_input() -> Result<(), anyhow::Error> {
        let day = util::day_from_filename(file!())?;
        let input = if let Some(input) = util::fetch_user_input(day)? {
            input
        } else {
            return Ok(());
        };

        log::warn!("{}", solve_2(&input));
        Ok(())
    }
}
