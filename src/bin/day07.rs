use std::{cmp::Ordering, collections::HashMap, fs::read_to_string, str::FromStr};

use anyhow::{Context, Error, Result};

type Label = u8;

#[derive(Debug, PartialEq, Eq)]
struct Hand(Vec<Label>);

impl Hand {
    fn valued(&self) -> (Vec<u8>, Vec<Label>) {
        let mut counts: HashMap<Label, u8> = HashMap::new();
        for label in &self.0 {
            (*counts.entry(*label).or_default()) += 1;
        }
        let mut values = counts.into_iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();
        values.sort_by(|a, b| b.cmp(a));
        values.into_iter().unzip()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.valued().cmp(&other.valued())
    }
}

impl FromStr for Hand {
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
        Ok(Hand(labels))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Bid {
    hand: Hand,
    bid: u32,
}

impl FromStr for Bid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let hand = Hand::from_str(parts.next().context("expecting a hand")?)?;
        let bid = parts.next().context("expecting a bid")?.parse::<u32>()?;
        Ok(Self { hand, bid })
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
    let score = get_score(&mut bids);
    println!("part 1: {score}");
    // println!("part 2: {}");
    Ok(())
}

fn get_score(bids: &mut Vec<Bid>) -> usize {
    bids.sort();
    println!("{}", bids.len());
    for bid in bids.iter() {
        println!("{:?} -> {:?}", bid, bid.hand.valued());
    }
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
