use aoc_runner_derive::aoc;

fn wrap_idx(max_len: usize, value: isize) -> usize {
    value.rem_euclid(max_len as isize) as usize
}

fn mix(input: &[isize], positions: &mut Vec<usize>) {
    for (input_idx, value) in input.iter().copied().enumerate() {
        print_line(input, &*positions);
        let pos = positions.iter().position(|&pos| pos == input_idx).unwrap();
        positions.remove(pos);
        let new_pos = wrap_idx(positions.len(), pos as isize + value);
        positions.insert(new_pos, input_idx);
    }
}

fn print_line(input: &[isize], positions: &[usize]) {
    let mut v = vec![0; input.len()];
    for idx in 0..input.len() {
        v[positions[idx]] = input[idx];
    }
    println!("{:?}", v);
}
#[aoc(day20, part1)]
pub fn part1(input: &str) -> isize {
    let input = EXAMPLE;

    let input: Vec<isize> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut positions: Vec<usize> = (0usize..input.len()).collect();
    mix(&input, &mut positions);
    let original_zero_idx = input.iter().copied().position(|value| value == 0).unwrap();
    let current_zero_idx = positions
        .iter()
        .copied()
        .position(|idx| idx == original_zero_idx)
        .unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|offset| input[positions[wrap_idx(input.len(), current_zero_idx as isize + offset)]])
        .sum()
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> isize {
    let input: Vec<isize> = input
        .lines()
        .map(|line| line.parse::<isize>().unwrap() * 811589153)
        .collect();
    let mut positions: Vec<usize> = (0usize..input.len()).collect();
    for _ in 0..10 {
        mix(&input, &mut positions);
    }
    let original_zero_idx = input.iter().copied().position(|value| value == 0).unwrap();
    let current_zero_idx = positions
        .iter()
        .copied()
        .position(|idx| idx == original_zero_idx)
        .unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|offset| input[positions[wrap_idx(input.len(), current_zero_idx as isize + offset)]])
        .sum()
}

const EXAMPLE: &str = "1
2
-3
3
-2
0
4";
