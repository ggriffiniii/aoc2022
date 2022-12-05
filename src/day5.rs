use aoc_runner_derive::aoc;

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    for line in input.lines().rev().skip(1) {
        for (stack_idx, crt) in line.as_bytes().chunks(4).map(|c| c[1] as char).enumerate() {
            if crt == ' ' {
                continue;
            }
            if stack_idx >= stacks.len() {
                stacks.resize(stack_idx+1, Vec::new());
            }
            stacks[stack_idx].push(crt);
        }
    }
    stacks
}

#[derive(Debug)]
struct Move {
    quantity: usize,
    from_stack_idx: usize,
    to_stack_idx: usize,
}
impl Move {
    fn parse(input: &str) -> Self {
        let input = input.strip_prefix("move ").unwrap_or(input);
        let (quantity, input) = input.split_once(" from ").unwrap();
        let (from_stack, to_stack) = input.split_once(" to ").unwrap();
        Move{
            quantity: quantity.parse().unwrap(),
            from_stack_idx: from_stack.parse::<usize>().unwrap() - 1,
            to_stack_idx: to_stack.parse::<usize>().unwrap() - 1,
        }
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> String {
    let (stack_input, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stack_input);
    for mov in moves.lines().map(Move::parse) {
        for _ in 0..mov.quantity {
            let crt = stacks[mov.from_stack_idx].pop().unwrap();
            stacks[mov.to_stack_idx].push(crt);
        }
    }
    stacks.into_iter().map(|v| v.last().copied().unwrap()).collect()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> String {
    let (stack_input, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stack_input);
    for mov in moves.lines().map(Move::parse) {
        let from_stack = &mut stacks[mov.from_stack_idx];
        let rm_idx = from_stack.len() - mov.quantity;
        let r = from_stack.split_off(rm_idx);
        stacks[mov.to_stack_idx].extend(r);
    }
    stacks.into_iter().map(|v| v.last().copied().unwrap()).collect()
}
