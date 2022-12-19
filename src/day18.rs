use crate::bitset::RadixBitSet;

use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Pos(u16);
impl Pos {
    fn new((x, y, z): (u16, u16, u16)) -> Self {
        debug_assert!(x < 20);
        debug_assert!(y < 20);
        debug_assert!(z < 20);
        Pos(x | (y << 5) | (z << 10))
    }

    fn x(self) -> u16 {
        self.0 & 0x1f
    }
    fn y(self) -> u16 {
        (self.0 >> 5) & 0x1f
    }
    fn z(self) -> u16 {
        self.0 >> 10
    }
}

#[derive(Debug)]
struct World(RadixBitSet);
impl World {
    fn new() -> Self {
        World(RadixBitSet::new())
    }
    fn insert(&mut self, pos: Pos) {
        self.0.set_bit(pos.0 as u32);
    }
    fn contains(&self, pos: Pos) -> bool {
        self.0.test_bit(pos.0 as u32)
    }
    fn iter(&self) -> impl Iterator<Item = Pos> + '_ {
        self.0.iter().map(|i| Pos(i as u16))
    }
}

fn neighbors_of(pos: Pos) -> impl Iterator<Item = Pos> {
    [
        Pos::new((pos.x() - 1, pos.y(), pos.z())),
        Pos::new((pos.x() + 1, pos.y(), pos.z())),
        Pos::new((pos.x(), pos.y() - 1, pos.z())),
        Pos::new((pos.x(), pos.y() + 1, pos.z())),
        Pos::new((pos.x(), pos.y(), pos.z() - 1)),
        Pos::new((pos.x(), pos.y(), pos.z() + 1)),
    ]
    .into_iter()
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    let world = input
        .lines()
        .map(|cube| {
            let mut iter = cube.split(',').map(|v| v.parse::<u16>().unwrap());
            Pos::new((
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ))
        })
        .fold(World::new(), |mut world, pos| {
            world.insert(pos);
            world
        });
    world
        .iter()
        .map(|cube| {
            neighbors_of(cube)
                .filter(|&neighbor| !world.contains(neighbor))
                .count()
        })
        .sum()
}

fn can_reach_edge_of_world(
    pos: Pos,
    world: &World,
    visited: &mut World,
    cache: &mut World,
) -> bool {
    if visited.contains(pos) {
        return cache.contains(pos);
    }
    if [pos.x(), pos.y(), pos.z()]
        .into_iter()
        .any(|v| v == 0 || v == 20)
    {
        visited.insert(pos);
        cache.insert(pos);
        return true;
    }
    visited.insert(pos);
    let can_reach_edge = neighbors_of(pos).any(|neighbor| {
        if world.contains(neighbor) {
            return false;
        }
        can_reach_edge_of_world(neighbor, world, visited, cache)
    });
    if can_reach_edge {
        cache.insert(pos);
    }
    can_reach_edge
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> usize {
    let world = input
        .lines()
        .map(|cube| {
            let mut iter = cube.split(',').map(|v| v.parse::<u16>().unwrap());
            Pos::new((
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ))
        })
        .fold(World::new(), |mut world, pos| {
            world.insert(pos);
            world
        });
    let mut visited = World::new();
    let mut cache = World::new();
    let sum = world
        .iter()
        .map(|cube| {
            neighbors_of(cube)
                .filter(|&neighbor| {
                    if world.contains(neighbor) {
                        return false;
                    }
                    can_reach_edge_of_world(neighbor, &world, &mut visited, &mut cache)
                })
                .count()
        })
        .sum();
    dbg!(visited.0.space_used());
    dbg!(cache.0.space_used());
    sum
}
