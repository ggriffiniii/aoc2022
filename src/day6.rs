use aoc_runner_derive::aoc;

fn find_marker(input: &str, marker_len: usize) -> usize {
    for (idx, window) in input.as_bytes().windows(marker_len).enumerate() {
        let mut set = 0u32;
        for c in window {
            set |= 1 << (c - b'a');
        }
        if set.count_ones() as usize == marker_len {
            return idx + marker_len;
        }
    }
    unreachable!("uhoh");
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    find_marker(input, 4)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    find_marker(input, 14)
}
