use std::cmp::Ordering;

use aoc_runner_derive::aoc;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ListOrInt {
    List(Vec<ListOrInt>),
    Int(usize),
}
impl Ord for ListOrInt {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ListOrInt::Int(left), ListOrInt::Int(right)) => left.cmp(right),
            (ListOrInt::List(left), ListOrInt::List(right)) => left.cmp(right),
            (left @ ListOrInt::Int(_), ListOrInt::List(right)) => {
                std::slice::from_ref(left).cmp(right.as_slice())
            }
            (ListOrInt::List(left), right @ ListOrInt::Int(_)) => {
                left.as_slice().cmp(std::slice::from_ref(right))
            }
        }
    }
}
impl PartialOrd for ListOrInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn parse_list(input: &str) -> Vec<ListOrInt> {
    fn _parse_list(mut input: &str) -> (Vec<ListOrInt>, &str) {
        let mut list = Vec::new();
        assert!(input.starts_with('['));
        input = &input[1..];
        while !input.starts_with(']') {
            let (list_or_int, rem) = _parse_list_or_int(input);
            list.push(list_or_int);
            input = rem;

            if input.starts_with(',') {
                input = &input[1..];
            }
        }
        assert!(input.starts_with(']'));
        (list, &input[1..])
    }

    fn _parse_list_or_int(input: &str) -> (ListOrInt, &str) {
        match &input[..1] {
            "[" => {
                let (list, input) = _parse_list(input);
                (ListOrInt::List(list), input)
            }
            _ => {
                let end = input
                    .as_bytes()
                    .iter()
                    .copied()
                    .position(|b| !b.is_ascii_digit())
                    .unwrap();
                let num: usize = input[..end].parse().unwrap();
                (ListOrInt::Int(num), &input[end..])
            }
        }
    }
    let (l, rem) = _parse_list(input);
    assert!(rem.is_empty());
    l
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(idx, packet_pair)| {
            let (first, second) = packet_pair.split_once('\n').unwrap();
            let first = parse_list(first);
            let second = parse_list(second);
            (first < second).then_some(idx + 1)
        })
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> usize {
    let mut packets: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_list)
        .collect();
    let divider1 = vec![ListOrInt::List(vec![ListOrInt::Int(2)])];
    let divider2 = vec![ListOrInt::List(vec![ListOrInt::Int(6)])];
    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort();
    (packets.binary_search(&divider1).unwrap() + 1)
        * (packets.binary_search(&divider2).unwrap() + 1)
}
