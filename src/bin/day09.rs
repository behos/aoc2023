use std::fs::read_to_string;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let sequences = read_to_string("inputs/09.txt")
        .context("Should have been able to read the file")?
        .trim()
        .split('\n')
        .map(parse)
        .collect::<Result<Vec<_>>>()?;

    let (part_1, part_2) = sequences.iter().fold((0, 0), |(p1, p2), seq| {
        (p1 + op_value(seq, next), p2 + op_value(seq, prev))
    });
    println!("part 1: {part_1}");
    println!("part 2: {part_2}");
    Ok(())
}

fn parse(raw: &str) -> Result<Vec<i32>> {
    raw.split(' ')
        .map(|s| s.parse::<i32>().context("failed to parse"))
        .collect()
}

fn next(sequence: &[i32], diff: i32) -> i32 {
    sequence[sequence.len() - 1] + diff
}

fn prev(sequence: &[i32], diff: i32) -> i32 {
    sequence[0] - diff
}

fn op_value(sequence: &[i32], op: fn(&[i32], i32) -> i32) -> i32 {
    let history: Vec<i32> = sequence.windows(2).map(|w| w[1] - w[0]).collect();
    let diff = if history.iter().all(|d| *d == 0) {
        0
    } else {
        op_value(&history, op)
    };
    op(sequence, diff)
}
