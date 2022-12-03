use aoc_runner_derive::aoc;

struct GroupsIter<'a>(std::str::Lines<'a>);
impl<'a> GroupsIter<'a> {
    fn new(input: &'a str) -> Self {
        Self(input.lines())
    }
}
impl<'a> Iterator for GroupsIter<'a> {
    type Item = [&'a str; 3];

    fn next(&mut self) -> Option<Self::Item> {
        Some([self.0.next()?, self.0.next()?, self.0.next()?])
    }
}

struct Compartment(u64);
impl Compartment {
    fn new(input: &str) -> Self {
        Self(input.bytes().fold(0u64, |rucksack, b| {
            let pri = if b >= b'a' {
                b - b'a' + 1
            } else {
                b - b'A' + 27
            };
            rucksack | (1 << pri)
        }))
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|rucksack| {
            let mid = rucksack.len() / 2;
            let compartment1 = Compartment::new(&rucksack[..mid]);
            let compartment2 = Compartment::new(&rucksack[mid..]);
            (compartment1.0 & compartment2.0).trailing_zeros() as usize
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    GroupsIter::new(input)
        .map(|rucksacks| {
            rucksacks
                .iter()
                .map(|rucksack| {
                    let mid = rucksack.len() / 2;
                    let compartment1 = Compartment::new(&rucksack[..mid]);
                    let compartment2 = Compartment::new(&rucksack[mid..]);
                    (compartment1.0 | compartment2.0) as usize
                })
                .fold(!0, |common_items, group_items| common_items & group_items)
                .trailing_zeros() as usize
        })
        .sum()
}
