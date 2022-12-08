use std::{convert::Infallible, str::FromStr};

use aoc_runner_derive::aoc;

struct TreeMap {
    tree_heights: Vec<u8>,
    num_cols: usize,
}
impl TreeMap {
    fn iter(&self) -> impl Iterator<Item = (usize, u8)> + '_ {
        self.tree_heights.iter().copied().enumerate()
    }

    fn trees_above(&self, idx: usize) -> impl Iterator<Item = u8> + '_ {
        let tree_row = idx / self.num_cols;
        let tree_col = idx % self.num_cols;
        (0..tree_row)
            .rev()
            .map(move |ri| self.tree_heights[(ri * self.num_cols + tree_col)])
    }
    fn trees_below(&self, idx: usize) -> impl Iterator<Item = u8> + '_ {
        let num_rows = self.tree_heights.len() / self.num_cols;
        let tree_row = idx / self.num_cols;
        let tree_col = idx % self.num_cols;
        ((tree_row + 1).min(num_rows)..num_rows)
            .map(move |ri| self.tree_heights[(ri * self.num_cols + tree_col)])
    }
    fn trees_left(&self, idx: usize) -> impl Iterator<Item = u8> + '_ {
        let tree_row = idx / self.num_cols;
        let tree_col = idx % self.num_cols;
        (0..tree_col)
            .rev()
            .map(move |ci| self.tree_heights[tree_row * self.num_cols + ci])
    }
    fn trees_right(&self, idx: usize) -> impl Iterator<Item = u8> + '_ {
        let tree_row = idx / self.num_cols;
        let tree_col = idx % self.num_cols;
        ((tree_col + 1).min(self.num_cols)..self.num_cols)
            .map(move |ci| self.tree_heights[tree_row * self.num_cols + ci])
    }
}
impl FromStr for TreeMap {
    type Err = Infallible;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let num_cols = input.find('\n').unwrap();
        let tree_heights: Vec<u8> = input
            .bytes()
            .filter(|b| b.is_ascii_digit())
            .map(|b| b - b'0')
            .collect();
        Ok(TreeMap {
            tree_heights,
            num_cols,
        })
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let trees: TreeMap = input.parse().unwrap();
    trees
        .iter()
        .filter(|&(idx, height)| {
            trees
                .trees_above(idx)
                .all(|neighbor_height| height > neighbor_height)
                || trees
                    .trees_below(idx)
                    .all(|neighbor_height| height > neighbor_height)
                || trees
                    .trees_left(idx)
                    .all(|neighbor_height| height > neighbor_height)
                || trees
                    .trees_right(idx)
                    .all(|neighbor_height| height > neighbor_height)
        })
        .count()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let trees: TreeMap = input.parse().unwrap();
    trees
        .iter()
        .map(|(idx, height)| {
            // we want an iterator that yields elements until it encounters a tree
            // taller than height, but it should include the first taller tree encountered.
            // take_while will be off by one because it wouldn't count the first taller tree.
            let yield_until_blocked = |blocked: &mut bool, neighbor_height| {
                if *blocked {
                    return None;
                }
                *blocked = neighbor_height >= height;
                Some(neighbor_height)
            };

            trees
                .trees_above(idx)
                .scan(false, yield_until_blocked)
                .count()
                * trees
                    .trees_below(idx)
                    .scan(false, yield_until_blocked)
                    .count()
                * trees
                    .trees_left(idx)
                    .scan(false, yield_until_blocked)
                    .count()
                * trees
                    .trees_right(idx)
                    .scan(false, yield_until_blocked)
                    .count()
        })
        .max()
        .unwrap()
}
