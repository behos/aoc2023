use std::{collections::BTreeMap, fs::read_to_string};

use anyhow::{anyhow, Context, Error, Result};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(anyhow!("unexpected character")),
        }
    }
}

type Directions = Vec<Direction>;
type Node = String;

#[derive(Debug)]
struct Paths {
    left: Node,
    right: Node,
}
type Network = BTreeMap<Node, Paths>;

struct Navigator {
    network: Network,
    directions: Directions,
}

impl Navigator {
    fn navigate(&self) -> usize {
        let mut idx = 0;
        let mut steps = 0;
        let mut current = "AAA";
        let destination = "ZZZ";
        while current != destination {
            let next_direction = &self.directions[idx];
            idx = (idx + 1) % self.directions.len();
            steps += 1;
            let node = &self.network[current];
            current = match next_direction {
                Direction::Left => &node.left,
                Direction::Right => &node.right,
            };
        }
        steps
    }
}

fn main() -> Result<()> {
    let contents =
        read_to_string("inputs/08.txt").context("Should have been able to read the file")?;
    let navigator = parse(contents.trim())?;
    println!("part 1: {}", navigator.navigate());
    // println!("part 2: {score_pt2}");
    Ok(())
}

fn parse(content: &str) -> Result<Navigator> {
    let mut parts = content.split("\n\n");
    let directions = parts
        .next()
        .context("missing directions")?
        .trim()
        .chars()
        .map(Direction::try_from)
        .collect::<Result<Vec<_>>>()
        .context("failed to parse direction")?;
    let network = parts
        .next()
        .expect("missing network")
        .split('\n')
        .map(|line| match line.chars().collect::<Vec<_>>()[..] {
            [n, w, k, ' ', '=', ' ', '(', l, f, t, ',', ' ', r, g, h, ')'] => Ok((
                format!("{n}{w}{k}"),
                Paths {
                    left: format!("{l}{f}{t}"),
                    right: format!("{r}{g}{h}"),
                },
            )),
            ref line => Err(anyhow!("unparseable line {line:?}")),
        })
        .collect::<Result<Network>>()?;
    Ok(Navigator {
        directions,
        network,
    })
}
