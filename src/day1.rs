use aoc_runner_derive::aoc;

struct ElfIter<'a>(usize, std::str::Lines<'a>);
impl<'a> ElfIter<'a> {
    fn new(input: &'a str) -> Self {
        ElfIter(0, input.lines())
    }
}

impl<'a> Iterator for ElfIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.0 = 0;
        loop {
            match self.1.next() {
                None => return None,
                Some(line) => {
                    if line.is_empty() {
                        return Some(self.0);
                    } else {
                        self.0 += line.parse::<usize>().unwrap();
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
