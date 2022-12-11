use std::{convert::Infallible, str::FromStr};

use aoc_runner_derive::aoc;

#[derive(Debug, Clone)]
enum LitOrOld {
    Lit(usize),
    Old,
}
#[derive(Debug, Clone)]
enum Arith {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
struct Operation {
    arith: Arith,
    val: LitOrOld,
}
impl FromStr for Operation {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let arith = match input.as_bytes()[4] {
            b'+' => Arith::Add,
            b'-' => Arith::Sub,
            b'*' => Arith::Mul,
            b'/' => Arith::Div,
            _ => panic!("uh oh"),
        };
        let val = match &input[6..] {
            "old" => LitOrOld::Old,
            i => LitOrOld::Lit(i.parse().unwrap()),
        };
        Ok(Operation { arith, val })
    }
}
impl Operation {
    fn apply(&self, old_value: usize) -> usize {
        let op_value = match self.val {
            LitOrOld::Lit(v) => v,
            LitOrOld::Old => old_value,
        };
        match self.arith {
            Arith::Add => old_value + op_value,
            Arith::Sub => old_value - op_value,
            Arith::Mul => old_value * op_value,
            Arith::Div => old_value / op_value,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    test_divisor: usize,
    throw_dest: [usize; 2],
    num_inspected_items: usize,
}
impl FromStr for Monkey {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();
        let _monkey = lines.next().unwrap();
        let items = &lines.next().unwrap()[18..];
        let op = &lines.next().unwrap()[19..];
        let test = &lines.next().unwrap()[21..];
        let true_throw_dest = &lines.next().unwrap()[29..];
        let false_throw_dest = &lines.next().unwrap()[30..];
        Ok(Monkey {
            num_inspected_items: 0,
            items: items
                .split(", ")
                .map(|i| i.parse::<usize>().unwrap())
                .collect(),
            op: op.parse().unwrap(),
            test_divisor: test.parse().unwrap(),
            throw_dest: [
                false_throw_dest.parse().unwrap(),
                true_throw_dest.parse().unwrap(),
            ],
        })
    }
}

fn observe_round(monkeys: &mut [Monkey], modulus: usize, relief_level: usize) {
    for i in 0..monkeys.len() {
        let monkey = &mut monkeys[i];
        monkey.num_inspected_items += monkey.items.len();
        let items = std::mem::take(&mut monkey.items);
        let op = monkey.op.clone();
        let test_divisor = monkey.test_divisor;
        let throw_dest = monkey.throw_dest;
        for worry_level in items {
            let worry_level = op.apply(worry_level) / relief_level % modulus;
            monkeys[throw_dest[(worry_level % test_divisor == 0) as usize]]
                .items
                .push(worry_level);
        }
    }
}
fn monkey_business_level(monkeys: &mut [Monkey], num_rounds: usize, relief_level: usize) -> usize {
    let modulus = monkeys
        .iter()
        .map(|monkey| monkey.test_divisor)
        .product::<usize>();
    for _ in 0..num_rounds {
        observe_round(monkeys, modulus, relief_level);
    }
    monkeys.sort_by_key(|monkey| monkey.num_inspected_items);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.num_inspected_items)
        .product::<usize>()
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let mut monkeys: Vec<_> = input
        .split("\n\n")
        .map(|monkey_record| monkey_record.parse::<Monkey>().unwrap())
        .collect();
    monkey_business_level(&mut monkeys, 20, 3)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let mut monkeys: Vec<_> = input
        .split("\n\n")
        .map(|monkey_record| monkey_record.parse::<Monkey>().unwrap())
        .collect();
    monkey_business_level(&mut monkeys, 10_000, 1)
}
