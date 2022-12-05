use std::{convert::Infallible, str::FromStr};

use aoc_runner_derive::aoc;

#[derive(Debug)]
struct Stacks(Vec<Vec<char>>);
impl Stacks {
    fn get_from_and_to_stacks(
        &mut self,
        from_idx: usize,
        to_idx: usize,
    ) -> (&mut Vec<char>, &mut Vec<char>) {
        assert_ne!(from_idx, to_idx);
        let (first, second) = self.0.split_at_mut(from_idx.max(to_idx));
        if from_idx < to_idx {
            (&mut first[from_idx], &mut second[0])
        } else {
            (&mut second[0], &mut first[to_idx])
        }
    }

    fn top_crates(&self) -> String {
        self.0.iter().map(|v| v.last().copied().unwrap()).collect()
    }
}

impl FromStr for Stacks {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut stacks = Vec::with_capacity(10);
        for line in input.lines().rev().skip(1) {
            for (stack_idx, crt) in line.as_bytes().chunks(4).map(|c| c[1] as char).enumerate() {
                if crt == ' ' {
                    continue;
                }
                if stack_idx >= stacks.len() {
                    stacks.resize(stack_idx + 1, Vec::with_capacity(10));
                }
                stacks[stack_idx].push(crt);
            }
        }
        Ok(Stacks(stacks))
    }
}

#[derive(Debug)]
struct Move {
    quantity: usize,
    from_stack_idx: usize,
    to_stack_idx: usize,
}

struct MovesIter<'a>(NumIter<'a>);
impl<'a> MovesIter<'a> {
    fn new(input: &'a str) -> Self {
        MovesIter(NumIter(input.bytes()))
    }
}
impl<'a> Iterator for MovesIter<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Move {
            quantity: self.0.next()?,
            from_stack_idx: self.0.next()? - 1,
            to_stack_idx: self.0.next()? - 1,
        })
    }
}

struct NumIter<'a>(std::str::Bytes<'a>);
impl<'a> Iterator for NumIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let mut found_digit = false;
        let mut num = 0usize;
        loop {
            match self.0.next() {
                None => {
                    if found_digit {
                        return Some(num);
                    }
                    return None;
                }
                Some(c) if c.is_ascii_digit() => {
                    found_digit = true;
                    num *= 10;
                    num += (c - b'0') as usize;
                }
                Some(_) => {
                    if found_digit {
                        return Some(num);
                    }
                }
            }
        }
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> String {
    let (stack_input, moves) = input.split_once("\n\n").unwrap();
    let mut stacks: Stacks = stack_input.parse().unwrap();
    for mov in MovesIter::new(moves) {
        let (from_stack, to_stack) =
            stacks.get_from_and_to_stacks(mov.from_stack_idx, mov.to_stack_idx);
        for _ in 0..mov.quantity {
            to_stack.push(from_stack.pop().unwrap());
        }
    }
    stacks.top_crates()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> String {
    let (stack_input, moves) = input.split_once("\n\n").unwrap();
    let mut stacks: Stacks = stack_input.parse().unwrap();
    for mov in MovesIter::new(moves) {
        let (from_stack, to_stack) =
            stacks.get_from_and_to_stacks(mov.from_stack_idx, mov.to_stack_idx);
        let offset = from_stack.len() - mov.quantity;
        to_stack.extend_from_slice(&from_stack[offset..]);
        from_stack.truncate(offset);
    }
    stacks.top_crates()
}
