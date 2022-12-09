use std::{collections::HashSet, convert::Infallible, str::FromStr};

use aoc_runner_derive::aoc;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Motion {
    num_steps: usize,
    direction: Direction,
}

impl FromStr for Motion {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (direction, num_steps) = input.split_once(' ').unwrap();
        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("uh oh"),
        };
        let num_steps: usize = num_steps.parse().unwrap();
        Ok(Motion {
            num_steps,
            direction,
        })
    }
}

#[derive(Debug)]
struct Rope<const N: usize> {
    knots: [Pos; N],
    tail_visited: HashSet<Pos>,
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Rope {
            knots: [Pos { x: 0, y: 0 }; N],
            tail_visited: HashSet::new(),
        }
    }

    fn walk(&mut self, motion: Motion) {
        for _ in 0..motion.num_steps {
            self.move_head(motion.direction);
            for i in 1..self.knots.len() {
                Self::adjust_tail(self.knots[i - 1], &mut self.knots[i]);
            }
            self.tail_visited
                .insert(self.knots.last().copied().unwrap());
        }
    }

    fn move_head(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.knots[0].y += 1,
            Direction::Down => self.knots[0].y -= 1,
            Direction::Left => self.knots[0].x -= 1,
            Direction::Right => self.knots[0].x += 1,
        }
    }

    fn adjust_tail(head: Pos, tail: &mut Pos) {
        let x_diff = head.x - tail.x;
        let y_diff = head.y - tail.y;
        if x_diff.abs() > 1 || y_diff.abs() > 1 {
            tail.x += x_diff.signum();
            tail.y += y_diff.signum();
        }
    }

    fn get_tail_visited(&self) -> &HashSet<Pos> {
        &self.tail_visited
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let mut rope = Rope::<2>::new();
    for motion in input
        .lines()
        .map(|motion| motion.parse::<Motion>().unwrap())
    {
        rope.walk(motion);
    }
    rope.get_tail_visited().len()
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    let mut rope = Rope::<10>::new();
    for motion in input
        .lines()
        .map(|motion| motion.parse::<Motion>().unwrap())
    {
        rope.walk(motion);
    }
    rope.get_tail_visited().len()
}
