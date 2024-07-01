use std::{cmp::Ordering, collections::HashMap, fs::read_to_string, str::FromStr};

use anyhow::{Context, Error, Result};

type Label = u8;

trait Cards: PartialEq + Eq {
    fn valued(&self) -> (Vec<u8>, Vec<Label>);
}

#[derive(Debug, PartialEq, Eq)]
struct CardsPt1(Vec<Label>);

#[derive(Debug, PartialEq, Eq)]
struct CardsPt2(Vec<Label>);

impl Cards for CardsPt1 {
    fn valued(&self) -> (Vec<u8>, Vec<Label>) {
        let mut counts: HashMap<Label, u8> = HashMap::new();
        for label in &self.0 {
            (*counts.entry(*label).or_default()) += 1;
        }
        let mut values = counts.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
        values.sort_by(|a, b| b.cmp(a));
        (values, self.0.clone())
    }
}

impl Cards for CardsPt2 {
    fn valued(&self) -> (Vec<u8>, Vec<Label>) {
        let mut counts: HashMap<Label, u8> = HashMap::new();
        for label in &self.0 {
            (*counts.entry(*label).or_default()) += 1;
        }
        let joker = counts.remove(&11).unwrap_or_default();
        let mut values = counts.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
        values.sort_by(|a, b| b.cmp(a));
        if values.len() > 0 {
            values[0] += joker;
        } else {
            values.push(joker)
        }
        (
            values,
            self.0.iter().cloned().map(|l| if l == 11 { 1 } else { l }).collect(),
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<T: Cards>(T);

impl<T: Cards> PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl<T: Cards> Ord for Hand<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.valued().cmp(&other.0.valued())
    }
}

impl FromStr for Hand<CardsPt1> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let labels = s
            .chars()
            .map(|c| {
                Ok(match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    n => n
                        .to_digit(10)
                        .with_context(|| format!("unrecognized char {n}"))?
                        as u8,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Hand(CardsPt1(labels)))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Bid<T: Cards> {
    hand: Hand<T>,
    bid: u32,
}

impl<T: Cards> PartialOrd for Bid<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Cards> Ord for Bid<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl FromStr for Bid<CardsPt1> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let hand = Hand::<CardsPt1>::from_str(parts.next().context("expecting a hand")?)?;
        let bid = parts.next().context("expecting a bid")?.parse::<u32>()?;
        Ok(Self { hand, bid })
    }
}

impl From<Bid<CardsPt1>> for Bid<CardsPt2> {
    fn from(value: Bid<CardsPt1>) -> Self {
        let Hand(CardsPt1(cards)) = value.hand;
        Self {
            hand: Hand(CardsPt2(cards)),
            bid: value.bid,
        }
    }
}

fn main() -> Result<()> {
    let contents =
        read_to_string("inputs/07.txt").context("Should have been able to read the file")?;
    let mut bids = contents
        .trim()
        .split('\n')
        .map(Bid::from_str)
        .collect::<Result<Vec<_>>>()
        .context("failed to make bids")?;
    let score_pt1 = get_score(&mut bids);
    let mut bids = bids.into_iter().map(Bid::<CardsPt2>::from).collect();
    let score_pt2 = get_score(&mut bids);

    println!("part 1: {score_pt1}");
    println!("part 2: {score_pt2}");
    Ok(())
}

fn get_score<T: Cards>(bids: &mut Vec<Bid<T>>) -> usize {
    bids.sort();
    bids.iter()
        .enumerate()
        .map(|(rank, bid)| (rank + 1) * bid.bid as usize)
        .sum::<usize>()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn example() {
        let contents = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;
        let mut bids = contents
            .trim()
            .split('\n')
            .map(Bid::from_str)
            .collect::<Result<Vec<_>>>()
            .expect("failed to make bids");
        bids.sort();
        let score = bids
            .iter()
            .enumerate()
            .map(|(rank, bid)| (rank + 1) * bid.bid as usize)
            .sum::<usize>();
        assert_eq!(score, 6440);
    }
}
