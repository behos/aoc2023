mod util;

use anyhow::{bail, Context, Error, Result};
use std::{array, collections::HashMap, fs::read_to_string, str::FromStr};

type Label = String;
type Slot = usize;
type Focal = u32;
type AsciiHash = usize;

fn main() -> Result<()> {
    let raw = read_to_string("inputs/15.txt").context("Should have been able to read the file")?;
    let raw = raw.trim();
    let hash_sum = raw.split(',').map(ascii_hash).sum::<usize>();
    println!("part 1: {hash_sum}");
    let boxes = arrange_boxes(raw)?;
    let focal_sum = focal_sum(boxes);
    println!("part 2: {focal_sum}");
    Ok(())
}

fn ascii_hash(input: &str) -> AsciiHash {
    input
        .as_bytes()
        .iter()
        .fold(0, |acc, c| ((acc + *c as usize) * 17 % 256))
}

enum Command {
    Set(Label, Focal),
    Remove(Label),
}

impl Command {
    fn ascii_hash(&self) -> AsciiHash {
        let label = match self {
            Self::Set(l, _) => l,
            Self::Remove(l) => l,
        };
        ascii_hash(label)
    }
}

#[derive(Default, Debug)]
struct LensBox {
    order_id: Slot,
    contents: HashMap<Label, (Slot, Focal)>,
}

impl LensBox {
    fn set(&mut self, label: Label, focal: Focal) {
        let order = if let Some((order, _)) = self.contents.remove(&label) {
            order
        } else {
            self.order_id += 1;
            self.order_id
        };
        self.contents.insert(label, (order, focal));
    }

    fn remove(&mut self, label: Label) {
        self.contents.remove(&label);
    }

    fn iter_ordered(&self) -> impl Iterator<Item = (Slot, Focal)> + '_ {
        let mut slots = self.contents.values().collect::<Vec<_>>();
        slots.sort();
        slots
            .into_iter()
            .enumerate()
            .map(|(idx, (_, focal))| (idx + 1, *focal))
    }
}

type LensBoxes = [LensBox; 256];

fn arrange_boxes(raw: &str) -> Result<LensBoxes> {
    let mut boxes = array::from_fn(|_| LensBox::default());
    for command in parse_commands(raw)? {
        let hash = command.ascii_hash();
        let used_box = &mut boxes[hash];
        match command {
            Command::Set(label, focal) => used_box.set(label, focal),
            Command::Remove(label) => used_box.remove(label),
        }
    }
    Ok(boxes)
}

fn parse_commands(input: &str) -> Result<Vec<Command>> {
    input.split(',').map(Command::from_str).collect()
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let command = match &s.chars().collect::<Vec<_>>()[..] {
            [p @ .., '-'] => Command::Remove(p.iter().collect()),
            [p @ .., '=', f] => Command::Set(
                p.iter().collect(),
                f.to_digit(10).context("non-digit focal")?,
            ),
            u => bail!("unexpected format {u:?}"),
        };
        Ok(command)
    }
}

fn focal_sum(boxes: LensBoxes) -> usize {
    boxes
        .iter()
        .enumerate()
        .flat_map(|(idx, lens_box)| {
            lens_box
                .iter_ordered()
                .map(move |(slot, focal)| (idx + 1) * slot * (focal as usize))
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(30, ascii_hash("rn=1"))
    }

    #[test]
    fn example_pt2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let boxes = arrange_boxes(input).expect("invalid input");
        assert_eq!(145, focal_sum(dbg!(boxes)))
    }
}
