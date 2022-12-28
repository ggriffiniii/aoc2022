use std::{
    collections::HashMap,
    convert::Infallible,
    fmt::{self, Write},
    iter::Sum,
    ops::{Add, Rem},
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
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.data.chunks(self.width) {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
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
        Ok(Map { data, width })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
                        if p < row_start {
                            p = row_start + self.map.width as isize - 1;
                        }
                        p.try_into().unwrap()
                    }
                    Direction::Up => (self.pos as isize - self.map.width as isize)
                        .rem_euclid(self.map.data.len() as isize)
                        as usize,
                };
                self.pos = new_pos;
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
        self.pos = self
            .walk_iter()
            .filter(|(_idx, tile)| *tile != MapTile::Empty)
            .take_while(|(_, tile)| *tile != MapTile::Wall)
            .take(dist)
            .last()
            .map(|(pos, _)| pos)
            .unwrap_or(self.pos);
    }
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> usize {
    let (map, steps) = input.split_once("\n\n").unwrap();
    let map: Map = map.parse().unwrap();
    let steps = parse_steps(steps);
    let mut person = Person::start(&map);
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct XYZ {
    x: usize,
    y: usize,
    z: usize,
}
impl Add for XYZ {
    type Output = XYZ;

    fn add(self, rhs: Self) -> Self::Output {
        XYZ {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Rem<usize> for XYZ {
    type Output = XYZ;

    fn rem(self, rhs: usize) -> Self::Output {
        XYZ {
            x: self.x % rhs,
            y: self.y % rhs,
            z: self.z % rhs,
        }
    }
}
impl Sum for XYZ {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|sum, e| sum + e)
            .unwrap_or(XYZ { x: 0, y: 0, z: 0 })
    }
}

#[derive(Debug, Clone)]
struct FaceCoords([XYZ; 4]);
impl FaceCoords {
    fn top_left(&self) -> XYZ {
        self.0[0]
    }
    fn top_right(&self) -> XYZ {
        self.0[1]
    }
    fn bottom_right(&self) -> XYZ {
        self.0[2]
    }
    fn bottom_left(&self) -> XYZ {
        self.0[3]
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct CubePos {
    face_idx: usize,
    face_pos: usize,
}

#[derive(Debug)]
struct Cube {
    width_in_faces: usize,
    face_len: usize,
    faces: Vec<Option<Vec<MapTile>>>,
    corner_coords: Vec<Option<FaceCoords>>,
}
impl FromStr for Cube {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let height = input.lines().count();
        let width = input.lines().map(|l| l.len()).max().unwrap();
        assert!(width != height, "not expecting square input");
        let width_in_faces = if width > height {
            assert!(width / 4 == height / 3);
            4
        } else {
            assert!(height / 4 == width / 3);
            3
        };
        let face_len = width / width_in_faces;
        let mut faces = vec![None; 12];
        for (row_idx, line) in input.lines().enumerate() {
            for (col_idx, tile) in line.bytes().enumerate().filter(|&(_idx, b)| b != b' ') {
                let face_idx = (row_idx / face_len) * width_in_faces + col_idx / face_len;
                let pos_within_face = row_idx % face_len * face_len + col_idx % face_len;
                faces[face_idx].get_or_insert_with(|| vec![MapTile::Empty; face_len * face_len])
                    [pos_within_face] = match tile {
                    b'.' => MapTile::Open,
                    b'#' => MapTile::Wall,
                    _ => panic!("invalid input"),
                };
            }
        }
        let corner_coords = calculate_corner_coords(&faces, width_in_faces);
        Ok(Cube {
            width_in_faces,
            face_len,
            faces,
            corner_coords,
        })
    }
}

impl Cube {
    fn start_pos(&self) -> CubePos {
        self.faces
            .iter()
            .enumerate()
            .find_map(|(idx, face)| {
                if face.is_none() {
                    return None;
                }
                Some(CubePos {
                    face_idx: idx,
                    face_pos: 0,
                })
            })
            .unwrap()
    }
    fn face_iter(
        &self,
        face_idx: usize,
        row_or_col: usize,
        dir: Direction,
    ) -> Box<dyn Iterator<Item = (usize, MapTile)> + '_> {
        let face = self.faces[face_idx].as_ref().unwrap().as_slice();
        match dir {
            Direction::Right => {
                let row_start = row_or_col * self.face_len;
                Box::new(
                    face.iter()
                        .copied()
                        .enumerate()
                        .skip(row_start)
                        .take(self.face_len),
                )
            }
            Direction::Down => {
                let col_start = row_or_col;
                Box::new(std::iter::successors(
                    Some((col_start, face[col_start])),
                    |prev| {
                        let next = prev.0 + self.face_len;
                        if next < face.len() {
                            Some((next, face[next]))
                        } else {
                            None
                        }
                    },
                ))
            }
            Direction::Left => {
                let row_start = row_or_col * self.face_len;
                let row_end = row_start + self.face_len - 1;
                Box::new(std::iter::successors(
                    Some((row_end, face[row_end])),
                    move |prev| {
                        if prev.0 > row_start {
                            let next = prev.0 - 1;
                            Some((next, face[next]))
                        } else {
                            None
                        }
                    },
                ))
            }
            Direction::Up => {
                let col_start = face.len() - self.face_len + row_or_col;
                Box::new(std::iter::successors(
                    Some((col_start, face[col_start])),
                    |prev| {
                        if prev.0 >= self.face_len {
                            let next = prev.0 - self.face_len;
                            Some((next, face[next]))
                        } else {
                            None
                        }
                    },
                ))
            }
        }
    }

    fn cube_iter(&self, pos: CubePos, dir: Direction) -> CubeIter {
        let (skip, row_or_col) = match dir {
            Direction::Right => (
                pos.face_pos % self.face_len + 1,
                pos.face_pos / self.face_len,
            ),
            Direction::Down => (
                pos.face_pos / self.face_len + 1,
                pos.face_pos % self.face_len,
            ),
            Direction::Left => (
                (self.face_len - 1) - pos.face_pos % self.face_len + 1,
                pos.face_pos / self.face_len,
            ),
            Direction::Up => (
                (self.face_len - 1) - pos.face_pos / self.face_len + 1,
                pos.face_pos % self.face_len,
            ),
        };
        let face_iter = Box::new(self.face_iter(pos.face_idx, row_or_col, dir).skip(skip));
        CubeIter {
            cube: self,
            face_iter,
            face_idx: pos.face_idx,
            row_or_col,
            dir,
        }
    }

    fn next_face(
        &self,
        face_idx: usize,
        row_or_col: usize,
        dir: Direction,
    ) -> (usize, usize, Direction) {
        let corners = self.corner_coords[face_idx].as_ref().unwrap();
        let edge = match dir {
            Direction::Right => (corners.top_right(), corners.bottom_right()),
            Direction::Down => (corners.bottom_left(), corners.bottom_right()),
            Direction::Left => (corners.top_left(), corners.bottom_left()),
            Direction::Up => (corners.top_left(), corners.top_right()),
        };
        self.corner_coords
            .iter()
            .enumerate()
            .filter(|&(idx, _face)| face_idx != idx)
            .filter_map(|(idx, face)| face.as_ref().map(|face| (idx, face)))
            .find_map(|(other_idx, other_face)| {
                if edge == (other_face.top_left(), other_face.top_right()) {
                    Some((other_idx, row_or_col, Direction::Down))
                } else if edge == (other_face.top_right(), other_face.top_left()) {
                    Some((other_idx, self.face_len - 1 - row_or_col, Direction::Down))
                } else if edge == (other_face.top_right(), other_face.bottom_right()) {
                    Some((other_idx, row_or_col, Direction::Left))
                } else if edge == (other_face.bottom_right(), other_face.top_right()) {
                    Some((other_idx, self.face_len - 1 - row_or_col, Direction::Left))
                } else if edge == (other_face.bottom_right(), other_face.bottom_left()) {
                    Some((other_idx, self.face_len - 1 - row_or_col, Direction::Up))
                } else if edge == (other_face.bottom_left(), other_face.bottom_right()) {
                    Some((other_idx, row_or_col, Direction::Up))
                } else if edge == (other_face.bottom_left(), other_face.top_left()) {
                    Some((other_idx, self.face_len - 1 - row_or_col, Direction::Right))
                } else if edge == (other_face.top_left(), other_face.bottom_left()) {
                    Some((other_idx, row_or_col, Direction::Right))
                } else {
                    None
                }
            })
            .unwrap()
    }
}
struct CubeIter<'a> {
    cube: &'a Cube,
    face_iter: Box<dyn Iterator<Item = (usize, MapTile)> + 'a>,
    face_idx: usize,
    row_or_col: usize,
    dir: Direction,
}
impl<'a> Iterator for CubeIter<'a> {
    type Item = (CubePos, MapTile);
    fn next(&mut self) -> Option<Self::Item> {
        match self.face_iter.next() {
            Some((face_pos, MapTile::Open)) => {
                return Some((
                    CubePos {
                        face_idx: self.face_idx,
                        face_pos,
                    },
                    MapTile::Open,
                ))
            }
            Some((_, MapTile::Wall)) => return None,
            Some((_, MapTile::Empty)) => panic!("unexpected"),
            None => {}
        }
        let (next_face, next_row_or_col, next_dir) =
            self.cube
                .next_face(self.face_idx, self.row_or_col, self.dir);
        self.face_iter = self.cube.face_iter(next_face, next_row_or_col, next_dir);
        match self.face_iter.next() {
            Some((face_pos, MapTile::Open)) => {
                self.face_idx = next_face;
                self.row_or_col = next_row_or_col;
                self.dir = next_dir;
                Some((
                    CubePos {
                        face_idx: self.face_idx,
                        face_pos,
                    },
                    MapTile::Open,
                ))
            }
            Some((_, MapTile::Wall)) => None,
            Some((_, MapTile::Empty)) | None => panic!("unexpected"),
        }
    }
}

fn calculate_corner_coords(
    faces: &[Option<Vec<MapTile>>],
    width: usize,
) -> Vec<Option<FaceCoords>> {
    enum NeighborRelation {
        Above,
        Left,
        Right,
        Below,
    }
    fn _fold_neighbors(
        faces: &[Option<Vec<MapTile>>],
        width: usize,
        coords: &mut [Option<FaceCoords>],
        current: usize,
    ) {
        if current >= width && faces[current - width].is_some() && coords[current - width].is_none()
        {
            _fold(
                faces,
                coords,
                coords[current].clone().unwrap(),
                current - width,
                NeighborRelation::Below,
            );
            _fold_neighbors(faces, width, coords, current - width);
        }
        if current % width > 0 && faces[current - 1].is_some() && coords[current - 1].is_none() {
            _fold(
                faces,
                coords,
                coords[current].clone().unwrap(),
                current - 1,
                NeighborRelation::Right,
            );
            _fold_neighbors(faces, width, coords, current - 1);
        }
        if current % width + 1 < width
            && faces[current + 1].is_some()
            && coords[current + 1].is_none()
        {
            _fold(
                faces,
                coords,
                coords[current].clone().unwrap(),
                current + 1,
                NeighborRelation::Left,
            );
            _fold_neighbors(faces, width, coords, current + 1);
        }
        if current + width < faces.len()
            && faces[current + width].is_some()
            && coords[current + width].is_none()
        {
            _fold(
                faces,
                coords,
                coords[current].clone().unwrap(),
                current + width,
                NeighborRelation::Above,
            );
            _fold_neighbors(faces, width, coords, current + width);
        }
    }
    fn _fold(
        faces: &[Option<Vec<MapTile>>],
        coords: &mut [Option<FaceCoords>],
        neighbor: FaceCoords,
        current: usize,
        neighbor_relation: NeighborRelation,
    ) {
        if faces[current].is_none() {
            return;
        }
        assert!(coords[current].is_none());
        let sum: XYZ = neighbor.0.iter().cloned().sum();
        let mask = XYZ {
            x: (sum.x == 0 || sum.x == 4) as usize,
            y: (sum.y == 0 || sum.y == 4) as usize,
            z: (sum.z == 0 || sum.z == 4) as usize,
        };
        let current_coords = match neighbor_relation {
            NeighborRelation::Above => FaceCoords([
                neighbor.bottom_left(),
                neighbor.bottom_right(),
                (neighbor.bottom_right() + mask) % 2,
                (neighbor.bottom_left() + mask) % 2,
            ]),
            NeighborRelation::Left => FaceCoords([
                neighbor.top_right(),
                (neighbor.top_right() + mask) % 2,
                (neighbor.bottom_right() + mask) % 2,
                neighbor.bottom_right(),
            ]),
            NeighborRelation::Right => FaceCoords([
                (neighbor.top_left() + mask) % 2,
                neighbor.top_left(),
                neighbor.bottom_left(),
                (neighbor.bottom_left() + mask) % 2,
            ]),
            NeighborRelation::Below => FaceCoords([
                (neighbor.top_left() + mask) % 2,
                (neighbor.top_right() + mask) % 2,
                neighbor.top_right(),
                neighbor.top_left(),
            ]),
        };
        coords[current] = Some(current_coords);
    }

    let first = faces.iter().position(|face| face.is_some()).unwrap();
    let mut coords = vec![None; 12];
    coords[first] = Some(FaceCoords([
        XYZ { x: 0, y: 0, z: 0 },
        XYZ { x: 1, y: 0, z: 0 },
        XYZ { x: 1, y: 1, z: 0 },
        XYZ { x: 0, y: 1, z: 0 },
    ]));
    _fold_neighbors(faces, width, &mut coords, first);
    assert!(coords.iter().filter(|x| x.is_some()).count() == 6);
    let points = coords
        .iter()
        .filter_map(|x| x.as_ref())
        .fold(HashMap::new(), |mut hm, x| {
            *hm.entry(x.top_left()).or_insert(0) += 1;
            *hm.entry(x.top_right()).or_insert(0) += 1;
            *hm.entry(x.bottom_right()).or_insert(0) += 1;
            *hm.entry(x.bottom_left()).or_insert(0) += 1;
            hm
        });
    assert!(points.len() == 8);
    assert!(points.values().all(|&x| x == 3));
    coords
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> usize {
    let (cube, steps) = input.split_once("\n\n").unwrap();
    let cube: Cube = cube.parse().unwrap();
    let steps = parse_steps(steps);
    let mut pos = cube.start_pos();
    let mut dir = Direction::Right;
    for step in steps {
        match step {
            Step::Walk(dist) => {
                let mut iter = cube.cube_iter(pos, dir);
                pos = iter
                    .by_ref()
                    .take(dist)
                    .last()
                    .map(|(pos, _)| pos)
                    .unwrap_or(pos);
                dir = iter.dir;
            }
            Step::TurnLeft => {
                dir = match dir {
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Left,
                };
            }
            Step::TurnRight => {
                dir = match dir {
                    Direction::Down => Direction::Left,
                    Direction::Right => Direction::Down,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                };
            }
        }
    }
    let row =
        (pos.face_idx / cube.width_in_faces * cube.face_len + pos.face_pos / cube.face_len) + 1;
    let col =
        (pos.face_idx % cube.width_in_faces * cube.face_len + pos.face_pos % cube.face_len) + 1;
    1000 * row + 4 * col + dir as usize
}
