use std::{
    convert::Infallible,
    fmt::{self, Write},
    str::FromStr,
};

use aoc_runner_derive::aoc;

#[derive(Debug)]
enum Step {
    Walk(usize),
    TurnRight,
    TurnLeft,
}
fn parse_steps(input: &str) -> Vec<Step> {
    let mut steps = Vec::new();
    let mut walk_dist = 0;
    for b in input.bytes() {
        match b {
            b'L' => {
                steps.push(Step::Walk(walk_dist));
                walk_dist = 0;
                steps.push(Step::TurnLeft);
            }
            b'R' => {
                steps.push(Step::Walk(walk_dist));
                walk_dist = 0;
                steps.push(Step::TurnRight);
            }
            b'0'..=b'9' => {
                walk_dist *= 10;
                walk_dist += (b - b'0') as usize;
            }
            _ => panic!("invalid input"),
        }
    }
    if walk_dist != 0 {
        steps.push(Step::Walk(walk_dist));
    }
    steps
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum MapTile {
    Empty,
    Open,
    Wall,
}
impl fmt::Display for MapTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            MapTile::Empty => ' ',
            MapTile::Open => '.',
            MapTile::Wall => '#',
        })
    }
}
#[derive(Debug)]
struct Map {
    data: Vec<MapTile>,
    width: usize,
    height: usize,
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.data.chunks(self.width) {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl FromStr for Map {
    type Err = Infallible;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let width = input.lines().map(|l| l.len()).max().unwrap();
        let height = input.lines().count();
        let mut data = vec![MapTile::Empty; width * height];
        for (row_idx, line) in input.lines().enumerate() {
            for (col_idx, tile) in line.bytes().enumerate() {
                let pos = row_idx * width + col_idx;
                data[pos] = match tile {
                    b'.' => MapTile::Open,
                    b'#' => MapTile::Wall,
                    b' ' => MapTile::Empty,
                    _ => panic!("invalid input"),
                };
            }
        }
        Ok(Map {
            data,
            width,
            height,
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right = 0,
    Down,
    Left,
    Up,
}

struct Person<'a> {
    pos: usize,
    dir: Direction,
    map: &'a Map,
}
impl<'a> Person<'a> {
    fn start(map: &'a Map) -> Self {
        Person {
            pos: map
                .data
                .iter()
                .position(|&tile| tile == MapTile::Open)
                .unwrap(),
            dir: Direction::Right,
            map,
        }
    }
    // xy position with silly 1-based indexing
    fn xy1(&self) -> (usize, usize) {
        let row = self.pos / self.map.width;
        let col = self.pos % self.map.width;
        (col + 1, row + 1)
    }
    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        };
    }
    fn turn_left(&mut self) {
        self.dir = match self.dir {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        };
    }
    fn walk_iter(&self) -> impl Iterator<Item = (usize, MapTile)> + '_ {
        #[derive(Debug, Clone)]
        struct WalkIter<'a> {
            dir: Direction,
            pos: usize,
            map: &'a Map,
        }
        impl<'a> Iterator for WalkIter<'a> {
            type Item = (usize, MapTile);
            fn next(&mut self) -> Option<Self::Item> {
                let new_pos = match self.dir {
                    Direction::Right => {
                        let row_start = self.pos / self.map.width * self.map.width;
                        row_start + ((self.pos + 1) % self.map.width)
                    }
                    Direction::Down => (self.pos + self.map.width) % self.map.data.len(),
                    Direction::Left => {
                        let row_start = (self.pos / self.map.width * self.map.width) as isize;
                        let mut p = self.pos as isize - 1;
                        if p < row_start as isize {
                            p = row_start + self.map.width as isize - 1;
                        }
                        p.try_into().unwrap()
                    }
                    Direction::Up => (self.pos as isize - self.map.width as isize)
                        .rem_euclid(self.map.data.len() as isize)
                        as usize,
                };
                self.pos = new_pos.try_into().unwrap();
                Some((self.pos, self.map.data[self.pos]))
            }
        }
        WalkIter {
            pos: self.pos,
            dir: self.dir,
            map: self.map,
        }
        .cycle()
    }

    fn walk(&mut self, dist: usize) {
        if let Some((pos, tile)) = self
            .walk_iter()
            .filter(|(_idx, tile)| *tile != MapTile::Empty)
            .take_while(|(_, tile)| {
                if *tile == MapTile::Wall {
                    return false;
                }
                true
            })
            .take(dist)
            .last()
        {
            self.pos = pos;
        }
    }
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> usize {
    let (map, steps) = input.split_once("\n\n").unwrap();
    let map: Map = map.parse().unwrap();
    dbg!(&map);
    let steps = parse_steps(steps);
    let mut person = Person::start(&map);
    dbg!(person.xy1());
    for step in steps {
        match step {
            Step::Walk(dist) => person.walk(dist),
            Step::TurnLeft => person.turn_left(),
            Step::TurnRight => person.turn_right(),
        }
    }
    let (col, row) = person.xy1();
    1000 * row + 4 * col + person.dir as usize
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> usize {
    let input = EXAMPLE;
    let (map, steps) = input.split_once("\n\n").unwrap();
    let map: Map = map.parse().unwrap();
    dbg!(map);

    0
}

const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
