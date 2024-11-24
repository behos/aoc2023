mod util;

use anyhow::{bail, Context, Error, Result};
use std::{collections::HashSet, fs::read_to_string, str::FromStr};
use util::{
    bidimap::{BidiMap, Parsed},
    direction::Direction,
};

enum Mirror {
    Forward,  // \
    Backward, // /
    Horizontal,
    Vertical,
}

impl TryFrom<char> for Parsed<Mirror> {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let item = match value {
            '\\' => Parsed::Item(Mirror::Forward),
            '/' => Parsed::Item(Mirror::Backward),
            '-' => Parsed::Item(Mirror::Horizontal),
            '|' => Parsed::Item(Mirror::Vertical),
            '.' => Parsed::Skip,
            _ => bail!("unknown char"),
        };
        Ok(item)
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Beam {
    point: (usize, usize),
    direction: Direction,
}

impl Beam {
    fn move_forward(self, dimensions: (usize, usize)) -> Option<Beam> {
        self.direction
            .move_fowrard(self.point, dimensions)
            .map(|point| Beam {
                point,
                direction: self.direction,
            })
    }
}

type Grid = BidiMap<Mirror>;
type Beams = HashSet<Beam>;
type Energized = HashSet<(usize, usize)>;

fn main() -> Result<()> {
    let raw = read_to_string("inputs/16.txt").context("Should have been able to read the file")?;
    let grid = Grid::from_str(&raw)?;
    let energized = process_beams(
        &grid,
        Beam {
            point: (0, 0),
            direction: Direction::E,
        },
    );
    println!("part 1: {}", energized.len());

    let (w, h) = grid.dimensions();
    let max = (0..w)
        .map(|x| {
            [
                Beam {
                    point: (x, 0),
                    direction: Direction::S,
                },
                Beam {
                    point: (x, h - 1),
                    direction: Direction::N,
                },
            ]
        })
        .chain((0..h).map(|y| {
            [
                Beam {
                    point: (0, y),
                    direction: Direction::E,
                },
                Beam {
                    point: (w - 1, y),
                    direction: Direction::W,
                },
            ]
        }))
        .flatten()
        .map(|initial| process_beams(&grid, initial).len())
        .max();
    println!("part 2: {max:?}");
    Ok(())
}

enum BeamRes {
    One(Beam),
    Two((Beam, Beam)),
}

fn process_beams(grid: &Grid, initial: Beam) -> Energized {
    let mut active_beams = vec![initial];
    let mut visited = Beams::new();
    let mut energized = Energized::new();

    while let Some(beam) = active_beams.pop() {
        energized.insert(beam.point);
        if visited.contains(&beam) {
            continue;
        }
        visited.insert(beam);
        let new_beams = match (beam.direction, grid.get(beam.point.0, beam.point.1)) {
            (Direction::N | Direction::S, Some(Mirror::Horizontal)) => BeamRes::Two((
                Beam {
                    direction: Direction::E,
                    point: beam.point,
                },
                Beam {
                    direction: Direction::W,
                    point: beam.point,
                },
            )),
            (Direction::E | Direction::W, Some(Mirror::Vertical)) => BeamRes::Two((
                Beam {
                    direction: Direction::N,
                    point: beam.point,
                },
                Beam {
                    direction: Direction::S,
                    point: beam.point,
                },
            )),
            (Direction::E, Some(Mirror::Forward)) | (Direction::W, Some(Mirror::Backward)) => {
                BeamRes::One(Beam {
                    direction: Direction::S,
                    point: beam.point,
                })
            }
            (Direction::E, Some(Mirror::Backward)) | (Direction::W, Some(Mirror::Forward)) => {
                BeamRes::One(Beam {
                    direction: Direction::N,
                    point: beam.point,
                })
            }
            (Direction::N, Some(Mirror::Forward)) | (Direction::S, Some(Mirror::Backward)) => {
                BeamRes::One(Beam {
                    direction: Direction::W,
                    point: beam.point,
                })
            }
            (Direction::N, Some(Mirror::Backward)) | (Direction::S, Some(Mirror::Forward)) => {
                BeamRes::One(Beam {
                    direction: Direction::E,
                    point: beam.point,
                })
            }
            _ => BeamRes::One(beam),
        };
        match new_beams {
            BeamRes::One(beam) => push_forward(grid, &mut active_beams, beam),
            BeamRes::Two((b1, b2)) => {
                push_forward(grid, &mut active_beams, b1);
                push_forward(grid, &mut active_beams, b2);
            }
        }
    }
    energized
}

fn push_forward(grid: &Grid, beams: &mut Vec<Beam>, beam: Beam) {
    if let Some(beam) = beam.move_forward(grid.dimensions()) {
        beams.push(beam)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#
        .trim();
        let grid = Grid::from_str(input).expect("failed to parse");
        let energized = process_beams(
            &grid,
            Beam {
                point: (0, 0),
                direction: Direction::E,
            },
        );
        let (w, h) = grid.dimensions();
        for y in 0..h {
            for x in 0..w {
                if energized.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".")
                }
            }
            println!()
        }
        assert_eq!(46, energized.len());
    }
}
