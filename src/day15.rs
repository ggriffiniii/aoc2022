use std::{collections::HashSet, convert::Infallible, str::FromStr};

use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
struct XY {
    x: isize,
    y: isize,
}
#[derive(Debug)]
struct Sensor {
    sensor: XY,
    beacon: XY,
}
impl Sensor {
    fn mdist(&self) -> usize {
        self.sensor.x.abs_diff(self.beacon.x) + self.sensor.y.abs_diff(self.beacon.y)
    }

    fn covers_entire_region(&self, region: Region) -> bool {
        let required_mdist = region
            .corners()
            .map(|XY { x, y }| self.sensor.x.abs_diff(x) + self.sensor.y.abs_diff(y))
            .max()
            .unwrap();
        required_mdist <= self.mdist()
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
struct Region {
    top_left: XY,
    bottom_right: XY,
}
impl Region {
    fn area(&self) -> usize {
        (self.top_left.x.abs_diff(self.bottom_right.x) + 1)
            * (self.top_left.y.abs_diff(self.bottom_right.y) + 1)
    }
    fn corners(&self) -> impl Iterator<Item = XY> {
        [
            XY {
                x: self.top_left.x,
                y: self.top_left.y,
            },
            XY {
                x: self.top_left.x,
                y: self.bottom_right.y,
            },
            XY {
                x: self.bottom_right.x,
                y: self.bottom_right.y,
            },
            XY {
                x: self.bottom_right.x,
                y: self.top_left.y,
            },
        ]
        .into_iter()
    }
    fn split_vertical(&self) -> Option<(Region, Region)> {
        if self.top_left.y == self.bottom_right.y {
            return None;
        }
        let mid_y = (self.top_left.y + self.bottom_right.y) / 2;
        Some((
            Region{
                top_left: self.top_left,
                bottom_right: XY{x: self.bottom_right.x, y: mid_y},
            },
            Region{
                top_left: XY{x: self.top_left.x, y: mid_y+1},
                bottom_right: self.bottom_right,
            },
        ))
    }
    fn split_horizontal(&self) -> Option<(Region, Region)> {
        if self.top_left.x == self.bottom_right.x {
            return None;
        }
        let mid_x = (self.top_left.x + self.bottom_right.x) / 2;
        Some((
            Region{
                top_left: self.top_left,
                bottom_right: XY{x: mid_x, y: self.bottom_right.y},
            },
            Region{
                top_left: XY{x: mid_x+1, y: self.top_left.y},
                bottom_right: self.bottom_right,
            },
        ))
    }

    fn quadrants(&self) -> Option<impl Iterator<Item = Region>> {
        let mut quads = vec![];
        match self.split_vertical() {
            Some((upper, lower)) => {
                match upper.split_horizontal() {
                    Some((upper_left, upper_right)) => {
                        quads.push(upper_left);
                        quads.push(upper_right);
                        let (lower_left, lower_right) = lower.split_horizontal().unwrap();
                        quads.push(lower_left);
                        quads.push(lower_right);
                        Some(quads.into_iter())
                    },
                    None => {
                        quads.push(upper);
                        quads.push(lower);
                        Some(quads.into_iter())
                    }
                }
            },
            None => {
                match self.split_horizontal() {
                    Some((left, right)) => {
                        quads.push(left);
                        quads.push(right);
                        Some(quads.into_iter())
                    },
                    None => {
                        None
                    }
                }
            },
        }
    }
}

impl FromStr for Sensor {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.strip_prefix("Sensor at x=").unwrap();
        let (sensor_x, rem) = input.split_once(", y=").unwrap();
        let (sensor_y, rem) = rem.split_once(": closest beacon is at x=").unwrap();
        let (beacon_x, beacon_y) = rem.split_once(", y=").unwrap();
        Ok(Sensor {
            sensor: XY {
                x: sensor_x.parse().unwrap(),
                y: sensor_y.parse().unwrap(),
            },
            beacon: XY {
                x: beacon_x.parse().unwrap(),
                y: beacon_y.parse().unwrap(),
            },
        })
    }
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let target_row = 2_000_000;
    let mut not_beacon = HashSet::new();
    let mut beacons_in_target_row = Vec::new();
    for Sensor { sensor, beacon } in input.lines().map(|line| line.parse::<Sensor>().unwrap()) {
        if beacon.y == target_row {
            beacons_in_target_row.push(beacon.x);
        }
        let dist = (sensor.x - beacon.x).abs() as usize + (sensor.y - beacon.y).abs() as usize;
        let y_offset = (target_row - sensor.y).abs() as usize;
        if y_offset < dist {
            for x in sensor.x - (dist - y_offset) as isize..=sensor.x + (dist - y_offset) as isize {
                not_beacon.insert(x);
            }
        }
    }
    for beacon in beacons_in_target_row {
        not_beacon.remove(&beacon);
    }
    not_beacon.len()
}

fn divide_and_conquer(sensors: &[Sensor], region: Region, depth: usize) -> Option<XY> {
    if sensors
        .iter()
        .any(|sensor| sensor.covers_entire_region(region.clone()))
    {
        return None;
    }
    if region.area() == 1 {
        return Some(region.top_left);
    }
    for quadrant in region.quadrants()? {
        if let Some(xy) = divide_and_conquer(sensors, quadrant, depth + 1) {
            return Some(xy);
        }
    }
    None
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> isize {
    let bounds = XY{x: 4_000_000, y: 4_000_000};
    let sensors: Vec<Sensor> = input
        .lines()
        .map(|line| line.parse::<Sensor>().unwrap())
        .collect();
    let XY{x, y} = divide_and_conquer(
        &sensors,
        Region {
            top_left: XY { x: 0, y: 0 },
            bottom_right: bounds,
        },
        0,
    )
    .unwrap();
    x * 4_000_000 + y
}