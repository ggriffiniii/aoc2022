use std::{cmp::Ordering, collections::BinaryHeap, convert::Infallible, str::FromStr};

use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Tile {
    Open,
    Blizzard(Direction),
}

struct Map {
    width: usize,
    grid: Vec<Tile>,
}
impl FromStr for Map {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let total_width = input.bytes().position(|b| b == b'\n').unwrap() + 1;
        let without_top_or_bottom_walls = &input[total_width..input.len() - total_width];
        let grid: Vec<_> = without_top_or_bottom_walls
            .as_bytes()
            .chunks(total_width)
            .flat_map(|row| {
                //strip walls and newline
                &row[1..][..total_width - 3]
            })
            .map(|b| match b {
                b'.' => Tile::Open,
                b'^' => Tile::Blizzard(Direction::Up),
                b'v' => Tile::Blizzard(Direction::Down),
                b'<' => Tile::Blizzard(Direction::Left),
                b'>' => Tile::Blizzard(Direction::Right),
                x => panic!("invalid input: {:?}", x),
            })
            .collect();
        let width = total_width - 3;
        Ok(Map { grid, width })
    }
}
impl Map {
    fn pos_for(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
    fn dist(&self, start_pos: usize, end_pos: usize) -> usize {
        let start_x = start_pos % self.width;
        let start_y = start_pos / self.width;
        let end_x = end_pos % self.width;
        let end_y = end_pos / self.width;
        start_x.abs_diff(end_x) + start_y.abs_diff(end_y)
    }
    fn is_blizzard_at_pos_at_step(&self, pos: usize, step: usize) -> bool {
        let x = pos % self.width;
        let y = pos / self.width;
        let height = self.grid.len() / self.width;
        let left_moving_blizzard = (x + step) % self.width;
        let right_moving_blizzard =
            (x as isize - step as isize).rem_euclid(self.width as isize) as usize;
        let up_moving_blizzard = (y + step) % height;
        let down_moving_blizzard =
            (y as isize - step as isize).rem_euclid(height as isize) as usize;

        use Direction::*;
        [
            (Tile::Blizzard(Left), (left_moving_blizzard, y)),
            (Tile::Blizzard(Right), (right_moving_blizzard, y)),
            (Tile::Blizzard(Up), (x, up_moving_blizzard)),
            (Tile::Blizzard(Down), (x, down_moving_blizzard)),
        ]
        .into_iter()
        .any(|(blizzard_type, (x, y))| {
            let pos = y * self.width + x;
            self.grid[pos] == blizzard_type
        })
    }
}

fn lcm(a: usize, b: usize) -> usize {
    fn gcd(a: usize, b: usize) -> usize {
        let mut max = a;
        let mut min = b;
        if min > max {
            (min, max) = (max, min);
        }

        loop {
            let r = max % min;
            if r == 0 {
                return min;
            }

            max = min;
            min = r;
        }
    }
    a * b / gcd(a, b)
}

fn shortest_path(map: &Map, start_pos: usize, end_pos: usize, start_steps: usize) -> usize {
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        min_cost: usize,
        steps: usize,
        pos: usize,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            self.min_cost
                .cmp(&other.min_cost)
                .reverse()
                .then_with(|| self.steps.cmp(&other.steps))
                .then_with(|| self.pos.cmp(&other.pos))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let map_height = map.grid.len() / map.width;
    let cycle_len = lcm(map_height, map.width);
    let mut dist = vec![usize::MAX; map.grid.len() * cycle_len];
    let mut pq = BinaryHeap::new();
    for steps in start_steps..start_steps + cycle_len {
        pq.push(State {
            min_cost: steps + 1 + map.dist(start_pos, end_pos),
            steps: steps + 1,
            pos: start_pos,
        });
    }

    while let Some(State {
        min_cost,
        steps,
        pos,
    }) = pq.pop()
    {
        let grid_state_idx = steps % cycle_len;
        if map.is_blizzard_at_pos_at_step(pos, steps) {
            dist[grid_state_idx * map.grid.len() + pos] = 0;
            continue;
        }
        if pos == end_pos {
            return steps + 1;
        }

        if steps > dist[grid_state_idx * map.grid.len() + pos] {
            continue;
        }

        let x = pos % map.width;
        let y = pos / map.width;

        let mut maybe_add_to_pq = |state: State| {
            let grid_state_idx = state.steps % cycle_len;
            if dist[grid_state_idx * map.grid.len() + state.pos] <= state.steps {
                return;
            }
            dist[grid_state_idx * map.grid.len() + state.pos] = state.steps;
            pq.push(state);
        };

        if x < map.width - 1 {
            let pos = map.pos_for(x + 1, y);
            let steps = steps + 1;
            let min_cost = steps + map.dist(pos, end_pos);
            maybe_add_to_pq(State {
                min_cost,
                steps,
                pos,
            });
        }
        if y < map_height - 1 {
            let pos = map.pos_for(x, y + 1);
            let steps = steps + 1;
            let min_cost = steps + map.dist(pos, end_pos);
            maybe_add_to_pq(State {
                min_cost,
                steps,
                pos,
            });
        }
        if x > 0 {
            let pos = map.pos_for(x - 1, y);
            let steps = steps + 1;
            let min_cost = steps + map.dist(pos, end_pos);
            maybe_add_to_pq(State {
                min_cost,
                steps,
                pos,
            });
        }
        if y > 0 {
            let pos = map.pos_for(x, y - 1);
            let steps = steps + 1;
            let min_cost = steps + map.dist(pos, end_pos);
            maybe_add_to_pq(State {
                min_cost,
                steps,
                pos,
            });
        }
        maybe_add_to_pq(State {
            min_cost: min_cost + 1,
            steps: steps + 1,
            pos,
        });
    }
    panic!("no path found");
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> usize {
    let map: Map = input.parse().unwrap();
    shortest_path(&map, 0, map.grid.len() - 1, 0)
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> usize {
    let map: Map = input.parse().unwrap();
    let steps_to_finish = shortest_path(&map, 0, map.grid.len() - 1, 0);
    let steps_back_to_start = shortest_path(&map, map.grid.len() - 1, 0, steps_to_finish);
    shortest_path(&map, 0, map.grid.len() - 1, steps_back_to_start)
}
