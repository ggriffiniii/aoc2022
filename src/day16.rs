use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
};

use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct RoomId(u8);

#[derive(Debug)]
struct Room {
    id: RoomId,
    valve_rate: usize,
    neighbors: u64,
}
impl Room {
    fn neighbors_iter(&self) -> impl Iterator<Item = RoomId> {
        struct NeighborsIter(u64);
        impl Iterator for NeighborsIter {
            type Item = RoomId;
            fn next(&mut self) -> Option<Self::Item> {
                if self.0 == 0 {
                    return None;
                }
                let lsb = self.0.trailing_zeros();
                // x & x.wrapping_neg() results in x with only the least significant bit set
                // x ^ (x & x.wrapping_neg()) results in clearing only the least significant bit
                self.0 ^= self.0 & self.0.wrapping_neg();
                Some(RoomId(lsb as u8))
            }
        }
        NeighborsIter(self.neighbors)
    }
}

// Store a path of rooms. Can only track paths that include a maximum of 15 rooms with id's 0-15.
#[derive(Debug, Copy, Clone)]
struct Path(u64, u16);
impl Path {
    fn empty() -> Self {
        Path(0, 0)
    }
    fn num_rooms(self) -> u8 {
        16 - self.0.leading_zeros() as u8 / 4
    }
    #[must_use]
    fn append(self, room_id: RoomId) -> Self {
        Path(
            self.0 | (room_id.0 as u64 + 1) << (self.num_rooms() * 4),
            self.1 | (1 << room_id.0),
        )
    }
    fn overlaps_with(self, other: Path) -> bool {
        self.1 & other.1 != 0
    }
    fn iter(self) -> impl Iterator<Item = RoomId> {
        struct Iter(u64);
        impl Iterator for Iter {
            type Item = RoomId;
            fn next(&mut self) -> Option<Self::Item> {
                if self.0 == 0 {
                    return None;
                }
                let room = RoomId((self.0 & 0xf) as u8 - 1);
                self.0 >>= 4;
                Some(room)
            }
        }
        Iter(self.0)
    }
}

fn pressure_released(
    path: Path,
    starting_room: RoomId,
    rooms: &[Room],
    distances: &[Vec<u8>],
    mut time_rem: u8,
) -> usize {
    let mut prev_room = starting_room;
    let mut sum = 0;
    for room in path.iter() {
        let dist = distances[prev_room.0 as usize][room.0 as usize];
        time_rem -= dist + 1;
        sum += time_rem as usize * rooms[room.0 as usize].valve_rate;
        prev_room = room;
    }
    sum
}

fn parse_rooms(input: &str) -> (Box<[Room]>, RoomId) {
    let mut rooms: Vec<_> = input
        .lines()
        .map(|input| {
            let str_id = &input[6..8];
            let input = &input[23..];
            let rate = &input[..input.chars().position(|c| c == ';').unwrap()];
            let neighbors =
                &input[input.chars().position(|c| c == ',').unwrap_or(input.len()) - 2..];
            (str_id, rate.parse::<usize>().unwrap(), neighbors)
        })
        .collect();
    assert!(rooms.len() <= 64);
    // Sort the rooms with valves first so that they have the lowest id's.
    rooms.sort_by_key(|(_id, rate, _neighbors)| Reverse(*rate));
    let id_to_idx: HashMap<_, _> = rooms
        .iter()
        .enumerate()
        .map(|(room_idx, (str_id, _, _))| (str_id.to_owned(), RoomId(room_idx as u8)))
        .collect();

    let rooms: Vec<_> = rooms
        .into_iter()
        .enumerate()
        .map(|(room_idx, (_str_id, valve_rate, neighbors))| Room {
            id: RoomId(room_idx as u8),
            valve_rate,
            neighbors: neighbors
                .split(", ")
                .map(|s| id_to_idx[s])
                .fold(0u64, |bitset, room_id| bitset | (1 << room_id.0)),
        })
        .collect();
    // Ensure there are no more than 16 rooms with valves.
    assert!(rooms
        .get(16)
        .map(|room| room.valve_rate == 0)
        .unwrap_or(true));

    (rooms.into(), id_to_idx["AA"])
}

fn calculate_distances(rooms: &[Room]) -> Vec<Vec<u8>> {
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: u8,
        pos: RoomId,
    }

    // The priority queue depends on `Ord`.
    // Explicitly implement the trait so the queue becomes a min-heap
    // instead of a max-heap.
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            self.cost
                .cmp(&other.cost)
                .reverse()
                .then_with(|| self.pos.cmp(&other.pos))
        }
    }

    // `PartialOrd` needs to be implemented as well.
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut distances = vec![vec![u8::MAX; rooms.len()]; rooms.len()];

    for room in rooms {
        let dist = distances[room.id.0 as usize].as_mut_slice();
        let mut heap = BinaryHeap::new();
        let mut visited = 0u64;

        // We're at `start`, with a zero cost
        dist[room.id.0 as usize] = 0;
        heap.push(State {
            cost: 0,
            pos: room.id,
        });

        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(State {
            cost,
            pos: position,
        }) = heap.pop()
        {
            if visited & (1 << position.0) != 0 {
                continue;
            }
            visited |= 1 << position.0;

            // Important as we may have already found a better way
            if cost > dist[position.0 as usize] {
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for neighbor in rooms[position.0 as usize].neighbors_iter() {
                let next = State {
                    cost: cost + 1,
                    pos: neighbor,
                };

                // If so, add it to the frontier and continue
                if next.cost < dist[next.pos.0 as usize] {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    dist[next.pos.0 as usize] = next.cost;
                }
            }
        }
    }
    distances
}

fn max_pressure_relief(rooms: &[Room], distances: &[Vec<u8>], starting_room: RoomId) -> usize {
    fn _max_pressure_relief(
        rooms: &[Room],
        distances: &[Vec<u8>],
        pos: RoomId,
        time_rem: u8,
        valves_open: u16,
    ) -> usize {
        rooms
            .iter()
            .filter(|room| room.id != pos)
            .map(|room| (room, distances[pos.0 as usize][room.id.0 as usize]))
            .filter(|(_room, cost)| *cost < time_rem)
            .filter(|(room, _cost)| valves_open & (1 << room.id.0) == 0)
            .map(|(room, cost)| {
                let time_rem = time_rem - cost - 1;
                room.valve_rate * (time_rem as usize)
                    + _max_pressure_relief(
                        rooms,
                        distances,
                        room.id,
                        time_rem,
                        valves_open | (1 << room.id.0),
                    )
            })
            .max()
            .unwrap_or(0)
    }
    _max_pressure_relief(rooms, distances, starting_room, 30, 0)
}

fn collect_paths(rooms: &[Room], distances: &[Vec<u8>], starting_room: RoomId) -> Vec<Path> {
    fn _collect_paths(
        rooms: &[Room],
        distances: &[Vec<u8>],
        pos: RoomId,
        time_rem: u8,
        valves_open: u16,
        parent_path: Path,
        paths: &mut Vec<Path>,
    ) {
        rooms
            .iter()
            .filter(|room| room.id != pos)
            .map(|room| (room, distances[pos.0 as usize][room.id.0 as usize]))
            .filter(|(_room, cost)| *cost < time_rem)
            .filter(|(room, _cost)| valves_open & (1 << room.id.0) == 0)
            .for_each(|(room, cost)| {
                let path = parent_path.append(room.id);
                paths.push(path);
                let time_rem = time_rem - cost - 1;
                _collect_paths(
                    rooms,
                    distances,
                    room.id,
                    time_rem,
                    valves_open | (1 << room.id.0),
                    path,
                    paths,
                );
            });
    }
    let mut paths = Vec::new();
    _collect_paths(
        rooms,
        distances,
        starting_room,
        26,
        0,
        Path::empty(),
        &mut paths,
    );
    paths
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let (rooms, starting_room) = parse_rooms(input);
    let distances = calculate_distances(&rooms);
    let rooms_with_valves = &rooms[..rooms.iter().position(|room| room.valve_rate == 0).unwrap()];
    max_pressure_relief(rooms_with_valves, distances.as_slice(), starting_room)
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let (rooms, starting_room) = parse_rooms(input);
    let distances = calculate_distances(&rooms);
    let rooms_with_valves = &rooms[..rooms.iter().position(|room| room.valve_rate == 0).unwrap()];
    let paths = collect_paths(rooms_with_valves, distances.as_slice(), starting_room);
    paths
        .iter()
        .copied()
        .filter_map(|path| {
            paths
                .iter()
                .copied()
                .filter_map(|elephant_path| {
                    if path.overlaps_with(elephant_path) {
                        return None;
                    }
                    let released =
                        pressure_released(path, starting_room, rooms_with_valves, &distances, 26)
                            + pressure_released(
                                elephant_path,
                                starting_room,
                                rooms_with_valves,
                                &distances,
                                26,
                            );
                    Some(released)
                })
                .max()
        })
        .max()
        .unwrap()
}
