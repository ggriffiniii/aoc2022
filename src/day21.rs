use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[derive(Debug)]
enum Job {
    UnknownVariable,
    Num(usize),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

fn eval(monkeys: &HashMap<String, Job>, job: &Job) -> Option<usize> {
    match job {
        Job::UnknownVariable => None,
        Job::Num(num) => Some(*num),
        Job::Add(a, b) => {
            let (a, b) = (monkeys.get(a.as_str())?, monkeys.get(b.as_str())?);
            Some(eval(monkeys, a)? + eval(monkeys, b)?)
        }
        Job::Sub(a, b) => {
            let (a, b) = (monkeys.get(a.as_str())?, monkeys.get(b.as_str())?);
            Some(eval(monkeys, a)? - eval(monkeys, b)?)
        }
        Job::Mul(a, b) => {
            let (a, b) = (monkeys.get(a.as_str())?, monkeys.get(b.as_str())?);
            Some(eval(monkeys, a)? * eval(monkeys, b)?)
        }
        Job::Div(a, b) => {
            let (a, b) = (monkeys.get(a.as_str())?, monkeys.get(b.as_str())?);
            Some(eval(monkeys, a)? / eval(monkeys, b)?)
        }
    }
}

fn solve_equation(monkeys: &HashMap<String, Job>, lhs: &Job, rhs: &Job) -> usize {
    fn _known_unknown<'a, 'b>(
        monkeys: &'a HashMap<String, Job>,
        a: &'b Job,
        b: &'b Job,
    ) -> (usize, bool, &'b Job) {
        match (eval(monkeys, a), eval(monkeys, b)) {
            (Some(known), None) => (known, true, b),
            (None, Some(known)) => (known, false, a),
            (Some(a), Some(b)) => panic!("neither side is unknown: {:?} {:?}", a, b),
            (None, None) => panic!("both sides have variables!"),
        }
    }
    let (known, _, unknown) = _known_unknown(monkeys, lhs, rhs);
    match unknown {
        Job::UnknownVariable => known,
        Job::Num(num) => *num,
        Job::Add(a, b) => {
            let (_known, _known_is_lhs, _unknown) =
                _known_unknown(monkeys, &monkeys[a.as_str()], &monkeys[b.as_str()]);
            solve_equation(monkeys, _unknown, &Job::Num(known - _known))
        }
        Job::Sub(a, b) => {
            let (_known, _known_is_lhs, _unknown) =
                _known_unknown(monkeys, &monkeys[a.as_str()], &monkeys[b.as_str()]);
            if _known_is_lhs {
                solve_equation(monkeys, _unknown, &Job::Num(_known - known))
            } else {
                solve_equation(monkeys, _unknown, &Job::Num(_known + known))
            }
        }
        Job::Mul(a, b) => {
            let (_known, _known_is_lhs, _unknown) =
                _known_unknown(monkeys, &monkeys[a.as_str()], &monkeys[b.as_str()]);
            solve_equation(monkeys, _unknown, &Job::Num(known / _known))
        }
        Job::Div(a, b) => {
            let (_known, _known_is_lhs, _unknown) =
                _known_unknown(monkeys, &monkeys[a.as_str()], &monkeys[b.as_str()]);
            if _known_is_lhs {
                solve_equation(monkeys, _unknown, &Job::Num(_known / known))
            } else {
                solve_equation(monkeys, _unknown, &Job::Num(_known * known))
            }
        }
    }
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
    let monkeys: HashMap<String, Job> = input
        .lines()
        .map(|line| {
            let (monkey, job) = line.split_once(": ").unwrap();
            let job = if job.len() == 11 {
                let a = job[..4].to_owned();
                let b = job[7..].to_owned();
                let job = job.as_bytes();
                match job[5] {
                    b'+' => Job::Add(a, b),
                    b'-' => Job::Sub(a, b),
                    b'*' => Job::Mul(a, b),
                    b'/' => Job::Div(a, b),
                    x => panic!("invalid op: {}", x as char),
                }
            } else {
                Job::Num(job.parse().unwrap())
            };
            (monkey.to_owned(), job)
        })
        .collect();
    eval(&monkeys, &monkeys["root"]).unwrap()
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> usize {
    let mut monkeys: HashMap<String, Job> = input
        .lines()
        .map(|line| {
            let (monkey, job) = line.split_once(": ").unwrap();
            let job = if monkey == "humn" {
                Job::UnknownVariable
            } else if job.len() == 11 {
                let a = job[..4].to_owned();
                let b = job[7..].to_owned();
                let job = job.as_bytes();
                match job[5] {
                    b'+' => Job::Add(a, b),
                    b'-' => Job::Sub(a, b),
                    b'*' => Job::Mul(a, b),
                    b'/' => Job::Div(a, b),
                    x => panic!("invalid op: {}", x as char),
                }
            } else {
                Job::Num(job.parse().unwrap())
            };
            (monkey.to_owned(), job)
        })
        .collect();
    *monkeys.get_mut("humn").unwrap() = Job::UnknownVariable;
    let root = &monkeys["root"];
    let (lhs, rhs) = match root {
        Job::Num(_) | Job::UnknownVariable => panic!("uh oh"),
        Job::Add(a, b) | Job::Sub(a, b) | Job::Mul(a, b) | Job::Div(a, b) => (a.clone(), b.clone()),
    };
    solve_equation(&monkeys, &monkeys[lhs.as_str()], &monkeys[rhs.as_str()])
}
