mod util;

use anyhow::{Context, Result};
use std::fs::read_to_string;

fn main() -> Result<()> {
    let raw = read_to_string("inputs/15.txt").context("Should have been able to read the file")?;
    let raw = raw.trim();
    let hash_sum = raw.split(',').map(|s| ascii_hash(s) as u32).sum::<u32>();
    println!("part 1: {hash_sum}");
    // println!("part 2: {total_load}");
    Ok(())
}

fn ascii_hash(input: &str) -> u8 {
    input
        .as_bytes()
        .iter()
        .fold(0, |acc, c| ((acc as u16 + *c as u16) * 17 % 256) as u8)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(30, ascii_hash("rn=1"))
    }
}
