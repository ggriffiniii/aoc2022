use aoc_runner_derive::aoc;

struct ElfIter<'a>(std::str::Bytes<'a>);
impl<'a> ElfIter<'a> {
    fn new(input: &'a str) -> Self {
        ElfIter(input.bytes())
    }
}

impl<'a> Iterator for ElfIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = 0;
        loop {
            let mut calories = 0;
            loop {
                match self.0.next() {
                    None => return None,
                    Some(b) if b == b'\n' => {
                        if calories == 0 {
                            return Some(sum);
                        }
                        sum += calories;
                        calories = 0;
                    }
                    Some(b) => {
                        calories *= 10;
                        calories += (b - b'0') as usize;
                    }
                }
            }
        }
    }
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    ElfIter::new(input).max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    ElfIter::new(input)
        .fold([0, 0, 0], |top3, x| match top3 {
            [a, b, c] if x < c => [a, b, c],
            [a, b, _] if x < b => [a, b, x],
            [a, b, _] if x < a => [a, x, b],
            [a, b, _] => [x, a, b],
        })
        .into_iter()
        .sum()
}
