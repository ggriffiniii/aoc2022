use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf_record| {
            elf_record
                .split('\n')
                .map(|calories| -> usize { calories.parse().unwrap() })
                .sum()
        })
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf_record| {
            elf_record
                .split('\n')
                .map(|calories| -> usize { calories.parse().unwrap() })
                .sum()
        })
        .fold([0, 0, 0], |top3, x| match top3 {
            [a, b, c] if x < c => [a, b, c],
            [a, b, _] if x < b => [a, b, x],
            [a, b, _] if x < a => [a, x, b],
            [a, b, _] => [x, a, b],
        })
        .into_iter()
        .sum()
}
