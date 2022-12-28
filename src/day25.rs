use std::{
    convert::Infallible,
    fmt::{self, Display},
    iter::Sum,
    str::FromStr,
};

use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Snafu(isize);
impl Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut i = self.0;
        let mut s = Vec::new();
        while i > 0 {
            let val = i % 5;
            s.push(match val {
                0 => {
                    i -= 0;
                    b'0'
                }
                1 => {
                    i -= 1;
                    b'1'
                }
                2 => {
                    i -= 2;
                    b'2'
                }
                3 => {
                    i -= -2;
                    b'='
                }
                4 => {
                    i -= -1;
                    b'-'
                }
                _ => panic!("invalid"),
            });
            i /= 5;
        }
        s.reverse();
        write!(f, "{}", String::from_utf8(s).unwrap())
    }
}

impl FromStr for Snafu {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut sum = 0;
        for b in input.bytes() {
            let b5_val = match b {
                b'2' => 2,
                b'1' => 1,
                b'0' => 0,
                b'-' => -1,
                b'=' => -2,
                x => panic!("invalid input: {}", x as char),
            };
            sum *= 5;
            sum += b5_val;
        }
        Ok(Snafu(sum))
    }
}

impl Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Snafu(iter.map(|snafu| snafu.0).sum())
    }
}

#[aoc(day25, part1)]
pub fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<Snafu>().unwrap())
        .sum::<Snafu>()
        .to_string()
}
