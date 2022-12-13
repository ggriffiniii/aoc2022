use std::cmp::Ordering;

use aoc_runner_derive::aoc;

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketData {
    List(Vec<PacketData>),
    Int(usize),
}
impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketData::Int(left), PacketData::Int(right)) => left.cmp(right),
            (PacketData::List(left), PacketData::List(right)) => left.cmp(right),
            (left @ PacketData::Int(_), PacketData::List(right)) => {
                std::slice::from_ref(left).cmp(right.as_slice())
            }
            (PacketData::List(left), right @ PacketData::Int(_)) => {
                left.as_slice().cmp(std::slice::from_ref(right))
            }
        }
    }
}
impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn parse_list(input: &str) -> Vec<PacketData> {
    fn _parse_list(mut input: &str) -> (Vec<PacketData>, &str) {
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

    fn _parse_list_or_int(input: &str) -> (PacketData, &str) {
        match &input[..1] {
            "[" => {
                let (list, input) = _parse_list(input);
                (PacketData::List(list), input)
            }
            _ => {
                let end = input
                    .as_bytes()
                    .iter()
                    .copied()
                    .position(|b| !b.is_ascii_digit())
                    .unwrap();
                let num: usize = input[..end].parse().unwrap();
                (PacketData::Int(num), &input[end..])
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

macro_rules! packet {
    (@[$($x:tt),+]) => { PacketData::List(vec![$(packet!(@$x)),+]) };
    (@$x:literal) => { (PacketData::Int($x)) };
    [$($x:tt),+] => { vec![$(packet!(@$x)),+] };
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> usize {
    let mut packets: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_list)
        .collect();
    packets.sort();
    (packets.binary_search(&packet!([[2]])).unwrap_err() + 1)
        * (packets.binary_search(&packet!([[6]])).unwrap_err() + 2)
}
