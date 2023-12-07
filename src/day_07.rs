use std::collections::HashMap;

use counter::Counter;

const CARDS: &'static [char] = &[
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'
];
const JOKER: u8 = 0;
const CARDS_WITH_JOKER: &'static [char] = &[
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'
];

type Cards = [u8; 5];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

        let card_counts = cards.iter().collect::<Counter<_>>().most_common();
        match card_counts.len() {
            1 => FiveOfAKind,
            2 if card_counts[0].1 == 4 => FourOfAKind,
            2 => FullHouse,
            3 if card_counts[0].1 == 3 => ThreeOfAKind,
            3 => TwoPair,
            4 => OnePair,
            5 => HighCard,
            _ => unreachable!("More than 5 cards"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    pub hand_type: HandType,
    pub cards: Cards,
}

pub fn solve_1(input: &str) -> String {
    let card_ordinals = build_card_ordinals(CARDS);

    let lines = input.lines();
    let mut hands_with_bids = vec!();
    for line in lines {
        if let Some((cards, bid)) = parse_cards_and_bid(line, &card_ordinals) {
            let hand_type = HandType::from_cards(&cards);
            hands_with_bids.push(
                (Hand { cards, hand_type }, bid)
            );
        }
    }
    hands_with_bids.sort_by_key(|(h, _)| *h);

    calc_total_score(&hands_with_bids).to_string()
}

pub fn solve_2(input: &str) -> String {
    let card_ordinals = build_card_ordinals(CARDS_WITH_JOKER);

    let lines = input.lines();
    let mut hands_with_bids = vec!();
    for line in lines {
        if let Some((cards, bid)) = parse_cards_and_bid(line, &card_ordinals) {
            let upgraded_cards = promote_jokers(&cards);
            let hand_type = HandType::from_cards(&upgraded_cards);
            hands_with_bids.push(
                (Hand { cards, hand_type }, bid)
            );
        }
    }
    hands_with_bids.sort_by_key(|(h, _)| *h);

    calc_total_score(&hands_with_bids).to_string()
}

fn build_card_ordinals(cards_order: &[char]) -> HashMap<char, u8> {
    let mut card_to_ord = HashMap::new();
    for (ord, card) in cards_order.iter().enumerate() {
        card_to_ord.insert(*card, ord as u8);
    }
    card_to_ord
}

fn parse_cards_and_bid(line: &str, card_ords: &HashMap<char, u8>) -> Option<(Cards, u64)> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    if let Some((cards_str, bid_str)) = line.split_once(' ') {
        let cards_str = cards_str.trim();
        let cards = parse_cards(cards_str, &card_ords);
        let bid_str = bid_str.trim();
        let bid = bid_str.parse().expect("bid number");
        Some((cards, bid))
    } else {
        None
    }
}

fn parse_cards(s: &str, card_ords: &HashMap<char, u8>) -> Cards {
    s.chars()
     .map(|c| card_ords[&c] as u8)
     .collect::<Vec<_>>()
        .try_into()
        .expect("5 cards in hand")
}

fn promote_jokers(cards: &Cards) -> Cards {
    let mut card_counts = cards.iter().collect::<Counter<_>>();
    if let None = card_counts.remove(&JOKER) {
        return *cards;
    };

    let top_cards = card_counts.k_most_common_ordered(1);
    let top_card = top_cards.first();
    if let Some((top_card, _)) = top_card {
        cards.map(|c| if c == JOKER { **top_card } else { c })
    } else {
        *cards
    }
}

fn calc_total_score(hands: &Vec<(Hand, u64)>) -> u64 {
    let mut res = 0;
    for (score, (_, bid)) in hands.iter().enumerate() {
        res += bid * (score as u64 + 1);
    }
    res
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
