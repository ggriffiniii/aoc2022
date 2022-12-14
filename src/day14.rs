use std::{collections::HashSet, convert::Infallible, str::FromStr};

use aoc_runner_derive::aoc;

#[derive(Debug)]
struct Grid {
    lowest_rock: usize,
    rocks: HashSet<(usize, usize)>,
}
impl Grid {
    fn drop_rock_p1(&mut self) -> Option<(usize, usize)> {
        let mut falling = (500, 0);
        while falling.1 <= self.lowest_rock {
            if !self.rocks.contains(&(falling.0, falling.1 + 1)) {
                falling.1 += 1;
            } else if !self.rocks.contains(&(falling.0 - 1, falling.1 + 1)) {
                falling.0 -= 1;
                falling.1 += 1;
            } else if !self.rocks.contains(&(falling.0 + 1, falling.1 + 1)) {
                falling.0 += 1;
                falling.1 += 1;
            } else {
                self.rocks.insert(falling);
                return Some(falling);
            }
        }
        None
    }

    fn drop_rock_p2(&mut self) -> Option<(usize, usize)> {
        let mut falling = (500, 0);
        if self.rocks.contains(&falling) {
            return None;
        };

        while falling.1 < self.lowest_rock + 1 {
            if !self.rocks.contains(&(falling.0, falling.1 + 1)) {
                falling.1 += 1;
            } else if !self.rocks.contains(&(falling.0 - 1, falling.1 + 1)) {
                falling.0 -= 1;
                falling.1 += 1;
            } else if !self.rocks.contains(&(falling.0 + 1, falling.1 + 1)) {
                falling.0 += 1;
                falling.1 += 1;
            } else {
                self.rocks.insert(falling);
                return Some(falling);
            }
        }
        self.rocks.insert(falling);
        Some(falling)
    }
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut rocks = HashSet::new();
        let mut lowest_rock = 0;
        for path in input.lines() {
            let points: Vec<_> = path
                .split(" -> ")
                .map(|xy| {
                    let (x, y) = xy.split_once(',').unwrap();
                    let x: usize = x.parse().unwrap();
                    let y: usize = y.parse().unwrap();
                    (x, y)
                })
                .collect();
            for line in points.windows(2) {
                let mut x_range = [line[0].0, line[1].0];
                x_range.sort();
                let mut y_range = [line[0].1, line[1].1];
                y_range.sort();
                for x in x_range[0]..=x_range[1] {
                    for y in y_range[0]..=y_range[1] {
                        rocks.insert((x, y));
                        lowest_rock = lowest_rock.max(y);
                    }
                }
            }
        }
        Ok(Grid { lowest_rock, rocks })
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    let mut grid: Grid = input.parse().unwrap();
    for i in 0.. {
        if grid.drop_rock_p1().is_none() {
            return i;
        }
    }
    unreachable!();
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let mut grid: Grid = input.parse().unwrap();
    for i in 0.. {
        if grid.drop_rock_p2().is_none() {
            return i;
        }
    }
    unreachable!();
}
