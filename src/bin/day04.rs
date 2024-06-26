use std::{collections::HashSet, fs::read_to_string, str::FromStr};

use anyhow::{Context, Error, Result};

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    numbers_you_have: HashSet<u32>,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(raw: &str) -> Result<Self> {
        let mut parts = raw
            .split(":")
            .last()
            .context("should be numbers")?
            .split(" | ");
        let winning_numbers = parse_numbers(parts.next().context("should have next set")?)?;
        let numbers_you_have = parse_numbers(parts.next().context("should have next set")?)?;
        Ok(Self {
            winning_numbers,
            numbers_you_have,
        })
    }
}

fn parse_numbers(raw: &str) -> Result<HashSet<u32>> {
    raw.split_whitespace()
        .map(|s| s.parse().with_context(|| format!("unparseable number {s}")))
        .collect::<Result<_>>()
}

impl Card {
    fn points(&self) -> u64 {
        let wins = self.wins() as u32;
        if wins == 0 {
            0
        } else {
            2_u64.pow(wins - 1)
        }
    }

    fn wins(&self) -> usize {
        self.numbers_you_have
            .intersection(&self.winning_numbers)
            .count()
    }
}

fn total_cards(cards: &[Card]) -> usize {
    let mut adders = vec![0i32; cards.len()];
    let mut running_total = 0;
    let mut current_cards = 1;
    for (i, card) in cards.iter().enumerate() {
        current_cards += adders[i];
        running_total += current_cards;
        let wins = card.wins();
        if wins > 0 {
            if i + 1 < cards.len() {
                adders[i + 1] += current_cards;
            }
            let reset_pos = i + wins + 1;
            if reset_pos < cards.len() {
                adders[reset_pos] -= current_cards;
            }
        }
    }
    running_total as usize
}

fn main() -> Result<()> {
    let contents = read_to_string("inputs/04.txt").expect("Should have been able to read the file");
    let trimmed = contents.trim();
    let cards = trimmed
        .split('\n')
        .map(|line| Card::from_str(line))
        .collect::<Result<Vec<_>>>()?;
    let points = cards.iter().map(Card::points).sum::<u64>();
    println!("part 1: {}", points);
    println!("part 2: {}", total_cards(&cards));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;
        let cards = input
            .trim()
            .split('\n')
            .map(|line| Card::from_str(line))
            .collect::<Result<Vec<_>>>()
            .expect("error parsing cards");

        assert_eq!(30, total_cards(&cards));
    }
}
