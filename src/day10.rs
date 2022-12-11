use std::{convert::Infallible, str::FromStr};

use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Noop,
    Addx(isize),
}
impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match &input[..4] {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(input[5..].parse().unwrap()),
            _ => panic!("uh oh"),
        })
    }
}

struct CPU<I> {
    reg: isize,
    current_instruction: Instruction,
    current_instruction_cycles_remaining: usize,
    instruction_stream: I,
}
impl<I> CPU<I>
where
    I: Iterator<Item = Instruction>,
{
    fn new(instruction_stream: I) -> Self {
        CPU {
            reg: 1,
            current_instruction: Instruction::Noop,
            current_instruction_cycles_remaining: 0,
            instruction_stream,
        }
    }
    fn tick(&mut self) {
        if self.current_instruction_cycles_remaining == 0 {
            if let Instruction::Addx(add) = self.current_instruction {
                self.reg += add;
            }
            self.current_instruction = self.instruction_stream.next().unwrap();
            self.current_instruction_cycles_remaining = match self.current_instruction {
                Instruction::Noop => 1,
                Instruction::Addx(_) => 2,
            }
        }
        self.current_instruction_cycles_remaining -= 1;
    }
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> isize {
    let mut cpu = CPU::new(input.lines().map(|x| x.parse::<Instruction>().unwrap()));
    let mut sum = 0;
    for cycle in 1..=220 {
        cpu.tick();
        if (cycle - 20) % 40 == 0 {
            sum += cycle * cpu.reg;
        }
    }
    sum
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> String {
    let mut cpu = CPU::new(input.lines().map(|x| x.parse::<Instruction>().unwrap()));
    let mut screen = String::new();
    for cycle in 0..240 {
        cpu.tick();
        let screen_col = cycle % 40;
        if screen_col == 0 {
            screen.push('\n');
        }
        if screen_col >= cpu.reg - 1 && screen_col <= cpu.reg + 1 {
            screen.push('#');
        } else {
            screen.push('.');
        }
    }
    screen
}
