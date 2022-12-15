use std::{convert::Infallible, str::FromStr};

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

    fn quadrants(&self) -> impl Iterator<Item = Region> {
        #[derive(Debug, Copy, Clone)]
        enum Split {
            None,
            Horizontal,
            Vertical,
            Both,
        }
        struct QuadIter {
            idx: usize,
            region: Region,
            split: Split,
            mid: XY,
        }
        impl Iterator for QuadIter {
            type Item = Region;
            fn next(&mut self) -> Option<Self::Item> {
                let r = match self.split {
                    Split::None => return None,
                    Split::Horizontal => {
                        if self.idx == 0 {
                            Some(Region {
                                top_left: self.region.top_left,
                                bottom_right: self.mid,
                            })
                        } else {
                            self.split = Split::None;
                            Some(Region {
                                top_left: XY {
                                    x: self.mid.x + 1,
                                    y: self.region.top_left.y,
                                },
                                bottom_right: self.region.bottom_right,
                            })
                        }
                    }
                    Split::Vertical => {
                        if self.idx == 0 {
                            Some(Region {
                                top_left: self.region.top_left,
                                bottom_right: self.mid,
                            })
                        } else {
                            self.split = Split::None;
                            Some(Region {
                                top_left: XY {
                                    x: self.region.top_left.x,
                                    y: self.mid.y + 1,
                                },
                                bottom_right: self.region.bottom_right,
                            })
                        }
                    }
                    Split::Both => {
                        if self.idx == 0 {
                            Some(Region {
                                top_left: self.region.top_left,
                                bottom_right: self.mid,
                            })
                        } else if self.idx == 1 {
                            Some(Region {
                                top_left: XY {
                                    x: self.mid.x + 1,
                                    y: self.region.top_left.y,
                                },
                                bottom_right: XY {
                                    x: self.region.bottom_right.x,
                                    y: self.mid.y,
                                },
                            })
                        } else if self.idx == 2 {
                            Some(Region {
                                top_left: XY {
                                    x: self.region.top_left.x,
                                    y: self.mid.y + 1,
                                },
                                bottom_right: XY {
                                    x: self.mid.x,
                                    y: self.region.bottom_right.y,
                                },
                            })
                        } else {
                            self.split = Split::None;
                            Some(Region {
                                top_left: XY {
                                    x: self.mid.x + 1,
                                    y: self.mid.y + 1,
                                },
                                bottom_right: self.region.bottom_right,
                            })
                        }
                    }
                };
                self.idx += 1;
                r
            }
        }
        let mid = XY {
            x: self.top_left.x.abs_diff(self.bottom_right.x) as isize / 2 + self.top_left.x,
            y: self.top_left.y.abs_diff(self.bottom_right.y) as isize / 2 + self.top_left.y,
        };
        let split = match (mid.x == self.bottom_right.x, mid.y == self.bottom_right.y) {
            (true, true) => {
                // region is a single square.
                Split::None
            }
            (true, false) => {
                // region is one square wide, multiple squares tall.
                Split::Vertical
            }
            (false, true) => {
                // region is multiple squares wide, one square tall.
                Split::Horizontal
            }
            (false, false) => {
                // region is multiple squares wide, multiple squares tall.
                Split::Both
            }
        };
        QuadIter {
            idx: 0,
            region: self.clone(),
            split,
            mid,
        }
    }
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    const TARGET_ROW: isize = 2_000_000;
    let mut target_row_cols = Vec::new();
    for Sensor { sensor, beacon } in input.lines().map(|line| line.parse::<Sensor>().unwrap()) {
        let dist = sensor.x.abs_diff(beacon.x) + sensor.y.abs_diff(beacon.y);
        let y_offset = TARGET_ROW.abs_diff(sensor.y);
        if y_offset < dist {
            let x_cols =
                sensor.x - (dist - y_offset) as isize..sensor.x + (dist - y_offset) as isize + 1;
            if beacon.y == TARGET_ROW {
                target_row_cols.push(x_cols.start..beacon.y);
                target_row_cols.push(beacon.y + 1..x_cols.end);
            } else {
                target_row_cols.push(x_cols);
            }
        }
    }
    target_row_cols.sort_by(|a, b| a.start.cmp(&b.start).then(a.end.cmp(&b.end)));
    let mut num_cols = 0;
    let mut row_cols_iter = target_row_cols.into_iter();
    let first = row_cols_iter.next().unwrap();
    num_cols += first.len();
    let mut prev_end = first.end;
    for col_range in row_cols_iter {
        let std::ops::Range { start, end } = col_range;
        num_cols += (start.max(prev_end)..end.max(prev_end)).len();
        prev_end = end.max(prev_end);
    }
    num_cols
}

fn divide_and_conquer(sensors: &[Sensor], region: Region) -> Option<XY> {
    if sensors
        .iter()
        .any(|sensor| sensor.covers_entire_region(region.clone()))
    {
        return None;
    }
    if region.area() == 1 {
        return Some(region.top_left);
    }
    for quadrant in region.quadrants() {
        if let Some(xy) = divide_and_conquer(sensors, quadrant) {
            return Some(xy);
        }
    }
    None
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> isize {
    let bounds = XY {
        x: 4_000_000,
        y: 4_000_000,
    };
    let sensors: Vec<Sensor> = input
        .lines()
        .map(|line| line.parse::<Sensor>().unwrap())
        .collect();
    let XY { x, y } = divide_and_conquer(
        &sensors,
        Region {
            top_left: XY { x: 0, y: 0 },
            bottom_right: bounds,
        },
    )
    .unwrap();
    x * 4_000_000 + y
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
      #[test]
      fn test_quadrants(x in -100isize..100, y in -100isize..100, width in 2isize..100, height in 2isize..100) {
        let region = Region{
          top_left: XY{x, y},
          bottom_right: XY{x: x+width as isize-1, y: y+height as isize-1},
        };
        let area = region.area();
        let quadrants: Vec<_> = region.quadrants().collect();
        dbg!(&region, &quadrants);
        assert_eq!(quadrants.iter().map(|r| r.area()).sum::<usize>(), area);
        assert!(quadrants.len() == 2 || quadrants.len() == 4);
      }
    }
}
