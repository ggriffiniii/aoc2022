use aoc_runner_derive::aoc;

fn find_marker(input: &str, marker_len: usize) -> usize {
    input
        .as_bytes()
        .windows(marker_len)
        .position(|window| {
            window
                .iter()
                .fold(0u32, |accum, b| accum | 1 << (b - b'a'))
                .count_ones()
                == marker_len as u32
        })
        .unwrap()
        + marker_len
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    find_marker(input, 4)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    find_marker(input, 14)
}
