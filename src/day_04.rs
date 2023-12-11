use std::collections::HashSet;

use anyhow::Context;

#[derive(Debug)]
struct Card {
    winning_nums: u32,
    copies: u32,
}

pub fn solve_1(input: &str) -> crate::PuzzleResult {
    let res = parse_cards(input)?.iter()
        .filter(|c| c.winning_nums > 0)
        .fold(0, |acc, c| acc + (1 << (c.winning_nums - 1)))
        .to_string();
    Ok(res)
}

fn parse_cards(input: &str) -> anyhow::Result<Vec<Card>> {
    input.lines()
        .map(parse_card)
        .filter_map(Result::transpose)
        .collect()
}

fn parse_card(line: &str) -> anyhow::Result<Option<Card>> {
    if let Some((_, card_str)) = line.split_once(':') {
        let card_str = card_str.trim();
        if let Some((my_nums_str, total_nums_str)) = card_str.split_once('|') {
            let my_nums = parse_nums(my_nums_str.trim())?;
            let total_nums = parse_nums(total_nums_str.trim())?;
            let winning_nums = total_nums.intersection(&my_nums).count();
            return Ok(Some(Card { winning_nums: winning_nums as u32, copies: 1 }));
        }
    }
    Ok(None)
}

fn parse_nums(s: &str) -> anyhow::Result<HashSet<u32>> {
    s.split(' ')
        .filter(|v| !v.is_empty())
        .map(|v| v.parse().context("Expect integer"))
        .collect()
}

pub fn solve_2(input: &str) -> crate::PuzzleResult {
    let mut cards = parse_cards(input)?;
    let mut sum = 0;
    for card_ix in 0..cards.len() {
        let (start_cards, rest_cards) = cards.split_at_mut(card_ix + 1);
        let card = start_cards.last_mut().context("Expect last card")?;
        for (_, following_card) in (0..card.winning_nums).zip(rest_cards) {
            following_card.copies += card.copies;
        }
        sum += card.copies;
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    const EXAMPLE_INPUT: &'static str = indoc!{"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn test_solve_1() -> anyhow::Result<()> {
        assert_eq!(
            solve_1(EXAMPLE_INPUT)?,
            "13".to_string()
        );
        Ok(())
    }

    #[test]
    fn test_solve_2() -> anyhow::Result<()> {
        assert_eq!(
            solve_2(EXAMPLE_INPUT)?,
            "30".to_string()
        );
        Ok(())
    }
}
