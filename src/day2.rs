use std::str::FromStr;

use aoc_runner_derive::aoc;

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Choice {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => Err("invalid input"),
        }
    }
}
enum GameResult {
    Lose,
    Draw,
    Win,
}
impl FromStr for GameResult {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err("invalid input"),
        }
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|game| {
            (
                game[..1].parse::<Choice>().unwrap(),
                game[2..].parse::<Choice>().unwrap(),
            )
        })
        .map(|game| match game {
            (Choice::Rock, Choice::Rock) => 1 + 3,
            (Choice::Rock, Choice::Paper) => 2 + 6,
            (Choice::Rock, Choice::Scissors) => 3 + 0,
            (Choice::Paper, Choice::Rock) => 1 + 0,
            (Choice::Paper, Choice::Paper) => 2 + 3,
            (Choice::Paper, Choice::Scissors) => 3 + 6,
            (Choice::Scissors, Choice::Rock) => 1 + 6,
            (Choice::Scissors, Choice::Paper) => 2 + 0,
            (Choice::Scissors, Choice::Scissors) => 3 + 3,
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|game| {
            (
                game[..1].parse::<Choice>().unwrap(),
                game[2..].parse::<GameResult>().unwrap(),
            )
        })
        .map(|game| match game {
            (Choice::Rock, GameResult::Lose) => 0 + 3,
            (Choice::Rock, GameResult::Draw) => 3 + 1,
            (Choice::Rock, GameResult::Win) => 6 + 2,
            (Choice::Paper, GameResult::Lose) => 0 + 1,
            (Choice::Paper, GameResult::Draw) => 3 + 2,
            (Choice::Paper, GameResult::Win) => 6 + 3,
            (Choice::Scissors, GameResult::Lose) => 0 + 2,
            (Choice::Scissors, GameResult::Draw) => 3 + 3,
            (Choice::Scissors, GameResult::Win) => 6 + 1,
        })
        .sum()
}
