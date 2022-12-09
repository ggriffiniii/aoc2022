use std::{cmp::Ordering, collections::HashSet};

use aoc_runner_derive::aoc;

fn adjust_tail(head: (isize, isize), tail: &mut (isize, isize)) {
    let x_diff = head.0 - tail.0;
    let y_diff = head.1 - tail.1;
    if x_diff.abs() > 1 || y_diff.abs() > 1 {
        tail.0 += x_diff.signum();
        tail.1 += y_diff.signum();
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(tail);
    for (dir, count) in input.lines().map(|line| line.split_once(' ').unwrap()) {
        let count: usize = count.parse().unwrap();
        for _ in 0..count {
            match dir {
                "U" => head.1 += 1,
                "R" => head.0 += 1,
                "D" => head.1 -= 1,
                "L" => head.0 -= 1,
                _ => panic!("uh oh"),
            }
            adjust_tail(head, &mut tail);
            visited.insert(tail);
        }
    }
    visited.len()
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    let mut knots = [(0, 0); 10];
    let mut visited = HashSet::new();
    visited.insert(knots[9]);
    for (dir, count) in input.lines().map(|line| line.split_once(' ').unwrap()) {
        let count: usize = count.parse().unwrap();
        for _ in 0..count {
            match dir {
                "U" => knots[0].1 += 1,
                "R" => knots[0].0 += 1,
                "D" => knots[0].1 -= 1,
                "L" => knots[0].0 -= 1,
                _ => panic!("uh oh"),
            }
            for i in 0..9 {
                let (a, b) = knots.split_at_mut(i + 1);
                adjust_tail(a[i], &mut b[0]);
            }
            visited.insert(knots[9]);
        }
    }
    visited.len()
}
