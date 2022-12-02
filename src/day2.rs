use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    input
        .as_bytes()
        .chunks(4)
        .map(|chunk| ((chunk[0] - b'A'), (chunk[2] - b'X')))
        .map(|(them, me)| (me + 3 - them + 1) % 3 * 3 + me + 1)
        .map(|x| x as usize)
        .sum::<usize>()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    input
        .as_bytes()
        .chunks(4)
        .map(|chunk| ((chunk[0] - b'A'), (chunk[2] - b'X')))
        .map(|(them, outcome)| (outcome + them + 2) % 3 + 3 * outcome + 1)
        .map(|x| x as usize)
        .sum()
}
