use std::{cmp::Ordering, collections::BinaryHeap, convert::Infallible, str::FromStr};

use aoc_runner_derive::aoc;
fn shortest_path<F>(grid: &Grid, end_cond: F) -> Option<usize>
where
    F: Fn(usize) -> bool,
{
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: usize,
        pos: usize,
    }

    // The priority queue depends on `Ord`.
    // Explicitly implement the trait so the queue becomes a min-heap
    // instead of a max-heap.
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            self.cost
                .cmp(&other.cost)
                .reverse()
                .then_with(|| self.pos.cmp(&other.pos))
        }
    }

    // `PartialOrd` needs to be implemented as well.
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..grid.data.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[grid.end] = 0;
    heap.push(State {
        cost: 0,
        pos: grid.end,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State {
        cost,
        pos: position,
    }) = heap.pop()
    {
        // Alternatively we could have continued to find all shortest paths
        if end_cond(position) {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for adjacency in grid
            .neighbors_of(position)
            .filter(|&neighbor_pos| grid.data[neighbor_pos] >= grid.data[position] - 1)
        {
            let next = State {
                cost: cost + 1,
                pos: adjacency,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.pos] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.pos] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

struct Grid {
    data: Vec<u8>,
    num_cols: usize,
    start: usize,
    end: usize,
}
impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let num_cols = input.chars().position(|c| c == '\n').unwrap();
        let mut data: Vec<u8> = input.bytes().filter(|&b| b != b'\n').collect();
        let start = data.iter().position(|&c| c == b'S').unwrap();
        let end = data.iter().position(|&c| c == b'E').unwrap();
        data[start] = b'a';
        data[end] = b'z';
        Ok(Grid {
            data,
            num_cols,
            start,
            end,
        })
    }
}
impl Grid {
    fn neighbors_of(&self, pos: usize) -> impl Iterator<Item = usize> + '_ {
        enum Neighbor {
            Above,
            Right,
            Below,
            Left,
            Done,
        }
        struct Iter<'a> {
            grid: &'a Grid,
            pos: usize,
            neighbor: Neighbor,
        }
        impl<'a> Iterator for Iter<'a> {
            type Item = usize;
            fn next(&mut self) -> Option<Self::Item> {
                let col = self.pos % self.grid.num_cols;
                let row = self.pos / self.grid.num_cols;
                let last_col = self.grid.num_cols - 1;
                let last_row = (self.grid.data.len() / self.grid.num_cols) - 1;
                loop {
                    match self.neighbor {
                        Neighbor::Above => {
                            self.neighbor = Neighbor::Right;
                            if row > 0 {
                                return Some(self.pos - self.grid.num_cols);
                            }
                        }
                        Neighbor::Right => {
                            self.neighbor = Neighbor::Below;
                            if col < last_col {
                                return Some(self.pos + 1);
                            }
                        }
                        Neighbor::Below => {
                            self.neighbor = Neighbor::Left;
                            if row < last_row {
                                return Some(self.pos + self.grid.num_cols);
                            }
                        }
                        Neighbor::Left => {
                            self.neighbor = Neighbor::Done;
                            if col > 0 {
                                return Some(self.pos - 1);
                            }
                        }
                        Neighbor::Done => return None,
                    }
                }
            }
        }
        Iter {
            grid: self,
            pos,
            neighbor: Neighbor::Above,
        }
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let grid: Grid = input.parse().unwrap();
    shortest_path(&grid, |pos| pos == grid.start).unwrap()
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    let grid: Grid = input.parse().unwrap();
    shortest_path(&grid, |pos| grid.data[pos] == b'a').unwrap()
}
