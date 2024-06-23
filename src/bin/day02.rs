use std::{fs::read_to_string, str::FromStr, cmp::max};

use anyhow::{bail, Context, Error, Result};

fn main() -> Result<()> {
    let contents = read_to_string("inputs/02.txt").expect("Should have been able to read the file");
    let trimmed = contents.trim();
    let games = trimmed
        .split('\n')
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()
        .context("failed to make games")?;
    println!("part 1: {}", possible_games(&games));
    println!("part 2: {}", minimum_cubes(&games));
    Ok(())
}

struct GamePick {
    red: usize,
    blue: usize,
    green: usize,
}

impl GamePick {
    fn is_possible(&self, pick: &Self) -> bool {
        self.red <= pick.red && self.green <= pick.green && self.blue <= pick.blue
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl FromStr for GamePick {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colors = s.split(", ");
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        for pair in colors {
            let mut parts = pair.split(' ');
            let number = parts
                .next()
                .context("should have a number")?
                .parse::<usize>()
                .context("should be a number")?;
            let color = parts.next().context("should have a color")?;
            match color {
                "red" => red += number,
                "blue" => blue += number,
                "green" => green += number,
                _ => bail!("got unknown color"),
            }
        }
        Ok(Self { red, blue, green })
    }
}

struct Game {
    id: usize,
    picks: Vec<GamePick>,
}

impl Game {
    fn is_possible(&self, pick: &GamePick) -> bool {
        self.picks.iter().all(|p| p.is_possible(pick))
    }

    fn min_pick(&self) -> GamePick {
        let (red, green, blue) = self.picks.iter().fold((0, 0, 0), |(r, g, b), pick| {
            (max(r, pick.red), max(g, pick.green), max(b, pick.blue))
        });
        GamePick { red, green, blue }
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        let id = parts
            .next()
            .context("should have next")?
            .strip_prefix("Game ")
            .context("should strip prefix")?
            .parse::<usize>()
            .context("should be a number")?;
        let picks = parts
            .next()
            .context("should have picks")?
            .split("; ")
            .map(GamePick::from_str)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { id, picks })
    }
}

fn possible_games(games: &[Game]) -> usize {
    let all_cubes = GamePick {
        red: 12,
        blue: 14,
        green: 13,
    };
    games
        .iter()
        .filter_map(|g| {
            if g.is_possible(&all_cubes) {
                Some(g.id)
            } else {
                None
            }
        })
        .sum()
}

fn minimum_cubes(games: &[Game]) -> usize {
    games.iter().map(Game::min_pick).map(|gp| gp.power()).sum()
}
