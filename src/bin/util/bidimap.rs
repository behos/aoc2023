#![allow(unused)]

use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

use anyhow::{anyhow, Context, Error};

use super::direction::Direction;

pub(crate) enum Parsed<T> {
    Item(T),
    Skip,
}

#[derive(Hash)]
pub(crate) struct BidiMap<T> {
    x: BTreeMap<usize, BTreeSet<usize>>,
    y: BTreeMap<usize, BTreeSet<usize>>,
    items: BTreeMap<(usize, usize), T>,
    w: usize,
    h: usize,
}

impl<T> BidiMap<T> {
    pub(crate) fn new() -> Self {
        Self {
            x: BTreeMap::new(),
            y: BTreeMap::new(),
            items: BTreeMap::new(),
            w: 0,
            h: 0,
        }
    }

    pub(crate) fn insert(&mut self, x: usize, y: usize, item: T) {
        self.x.entry(x).or_default().insert(y);
        self.y.entry(y).or_default().insert(x);
        self.items.insert((x, y), item);
    }

    pub(crate) fn remove(&mut self, x: usize, y: usize) {
        self.x.entry(x).or_default().remove(&y);
        self.y.entry(y).or_default().remove(&x);
        self.items.remove(&(x, y));
    }

    pub(crate) fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.items.get(&(x, y))
    }

    pub(crate) fn iter_by_x(&self) -> impl DoubleEndedIterator<Item = (usize, usize)> + '_ {
        self.x
            .iter()
            .flat_map(|(x, ys)| ys.iter().map(|y| (*x, *y)))
    }

    pub(crate) fn iter_by_y(&self) -> impl DoubleEndedIterator<Item = (usize, usize)> + '_ {
        self.y
            .iter()
            .flat_map(|(y, xs)| xs.iter().map(|x| (*x, *y)))
    }

    pub(crate) fn first_after(
        &self,
        (x, y): (usize, usize),
        direction: Direction,
    ) -> Option<(usize, usize)> {
        match direction {
            Direction::N => self
                .x
                .get(&x)
                .and_then(|e| e.range(..y).next_back().map(|y| (x, *y))),
            Direction::S => self
                .x
                .get(&x)
                .and_then(|e| e.range(y + 1..).next().map(|y| (x, *y))),
            Direction::E => self
                .y
                .get(&y)
                .and_then(|e| e.range(x + 1..).next().map(|x| (*x, y))),
            Direction::W => self
                .y
                .get(&y)
                .and_then(|e| e.range(..x).next_back().map(|x| (*x, y))),
        }
    }
}

impl<T> FromStr for BidiMap<T>
where
    Parsed<T>: TryFrom<char>,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Self::new();
        let mut w = 0;
        let mut h = 0;
        for (y, line) in s.trim().split('\n').enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Parsed::Item(item) =
                    Parsed::<T>::try_from(c).map_err(|_| anyhow! {"conversion failed"})?
                {
                    map.x.entry(x).or_default().insert(y);
                    map.y.entry(y).or_default().insert(x);
                    map.items.insert((x, y), item);
                }
                map.w = x + 1;
            }
            map.h = y + 1;
        }
        Ok(map)
    }
}
