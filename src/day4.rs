use std::ops::RangeInclusive;

use aoc_runner_derive::aoc;

fn parse_pair(input: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let (first, second) = input.split_once(',').unwrap();
    (parse_range(first), parse_range(second))
}

fn parse_range(input: &str) -> RangeInclusive<usize> {
    let (begin, end) = input.split_once('-').unwrap();
    let begin: usize = begin.parse().unwrap();
    let end: usize = end.parse().unwrap();
    begin..=end
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_pair)
        .filter(|(first, second)| {
            (first.start() <= second.start() && first.end() >= second.end())
                || (second.start() <= first.start() && second.end() >= first.end())
        })
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_pair)
        .filter(|(first, second)| first.start() <= second.end() && second.start() <= first.end())
        .count()
}
