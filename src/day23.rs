use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}
impl Direction {
    fn next(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::East,
            Direction::East => Direction::North,
        }
    }
    fn iter(self) -> impl Iterator<Item = Self> {
        std::iter::successors(Some(self), |dir| Some(dir.next()))
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> usize {
    let mut elves: HashSet<_> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_col, b)| *b == b'#')
                .map(move |(col, _b)| (col as isize, row as isize))
        })
        .collect();
    for dir in Direction::North.iter().take(10) {
        let mut proposals: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
        for (x, y) in elves.iter().copied() {
            let neighbors = [
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
                (x, y + 1),
                (x - 1, y + 1),
                (x - 1, y),
            ];
            if !neighbors.into_iter().any(|pos| elves.contains(&pos)) {
                continue;
            }
            for dir in dir.iter().take(4) {
                let cells_to_check = match dir {
                    Direction::North => {
                        let row_above = y - 1;
                        [(x - 1, row_above), (x, row_above), (x + 1, row_above)]
                    }
                    Direction::South => {
                        let row_below = y + 1;
                        [(x - 1, row_below), (x, row_below), (x + 1, row_below)]
                    }
                    Direction::West => {
                        let col_left = x - 1;
                        [(col_left, y - 1), (col_left, y), (col_left, y + 1)]
                    }
                    Direction::East => {
                        let col_right = x + 1;
                        [(col_right, y - 1), (col_right, y), (col_right, y + 1)]
                    }
                };
                if !cells_to_check.into_iter().any(|pos| elves.contains(&pos)) {
                    proposals.entry(cells_to_check[1]).or_default().push((x, y));
                    break;
                }
            }
        }
        for (to, from) in proposals
            .into_iter()
            .filter_map(|(to, from)| (from.len() == 1).then(|| (to, from[0])))
        {
            elves.remove(&from);
            elves.insert(to);
        }
    }
    let (top_left, bottom_right) = elves.iter().fold(
        ((isize::MAX, isize::MAX), (isize::MIN, isize::MIN)),
        |((x0, y0), (x1, y1)), &(x, y)| ((x0.min(x), y0.min(y)), (x1.max(x), y1.max(y))),
    );
    let total_tiles =
        (top_left.0.abs_diff(bottom_right.0) + 1) * (top_left.1.abs_diff(bottom_right.1) + 1);
    total_tiles - elves.len()
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> usize {
    let mut elves: HashSet<_> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_col, b)| *b == b'#')
                .map(move |(col, _b)| (col as isize, row as isize))
        })
        .collect();
    for (loop_iter, dir) in Direction::North.iter().enumerate() {
        let mut proposals: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
        for (x, y) in elves.iter().copied() {
            let neighbors = [
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
                (x, y + 1),
                (x - 1, y + 1),
                (x - 1, y),
            ];
            if !neighbors.into_iter().any(|pos| elves.contains(&pos)) {
                continue;
            }
            for dir in dir.iter().take(4) {
                let cells_to_check = match dir {
                    Direction::North => {
                        let row_above = y - 1;
                        [(x - 1, row_above), (x, row_above), (x + 1, row_above)]
                    }
                    Direction::South => {
                        let row_below = y + 1;
                        [(x - 1, row_below), (x, row_below), (x + 1, row_below)]
                    }
                    Direction::West => {
                        let col_left = x - 1;
                        [(col_left, y - 1), (col_left, y), (col_left, y + 1)]
                    }
                    Direction::East => {
                        let col_right = x + 1;
                        [(col_right, y - 1), (col_right, y), (col_right, y + 1)]
                    }
                };
                if !cells_to_check.into_iter().any(|pos| elves.contains(&pos)) {
                    proposals.entry(cells_to_check[1]).or_default().push((x, y));
                    break;
                }
            }
        }
        if proposals.is_empty() {
            return loop_iter + 1;
        }
        for (to, from) in proposals
            .into_iter()
            .filter_map(|(to, from)| (from.len() == 1).then(|| (to, from[0])))
        {
            elves.remove(&from);
            elves.insert(to);
        }
    }
    unreachable!()
}
