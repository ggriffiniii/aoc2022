use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct XY {
    x: usize,
    y: usize,
}

/*
 *  Rock shapes
 *
 *  ####
 *
 *  .#.
 *  ###
 *  .#.
 *
 *  ..#
 *  ..#
 *  ###
 *
 *  #
 *  #
 *  #
 *  #
 *
 *  ##
 *  ##
 */

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Rocks {
    Dash,
    Plus,
    L,
    I,
    Square,
}
impl Rocks {
    fn iter() -> impl Iterator<Item = Rocks> {
        [Rocks::Dash, Rocks::Plus, Rocks::L, Rocks::I, Rocks::Square]
            .into_iter()
            .cycle()
    }
    fn height(self) -> usize {
        match self {
            Rocks::Dash => 1,
            Rocks::Plus => 3,
            Rocks::L => 3,
            Rocks::I => 4,
            Rocks::Square => 2,
        }
    }
    fn width(self) -> usize {
        match self {
            Rocks::Dash => 4,
            Rocks::Plus => 3,
            Rocks::L => 3,
            Rocks::I => 1,
            Rocks::Square => 2,
        }
    }
    fn landed(self, XY { x, y }: XY, grid: &mut HashSet<XY>) {
        // x,y represent the lower left corner of each shape
        match self {
            Rocks::Dash => {
                [
                    XY { x, y: y },
                    XY { x: x + 1, y: y },
                    XY { x: x + 2, y: y },
                    XY { x: x + 3, y: y },
                ]
                .into_iter()
                .for_each(|xy| {
                    grid.insert(xy);
                });
            }
            Rocks::Plus => {
                [
                    XY { x, y: y + 1 },
                    XY { x: x + 1, y: y },
                    XY { x: x + 1, y: y + 1 },
                    XY { x: x + 1, y: y + 2 },
                    XY { x: x + 2, y: y + 1 },
                ]
                .into_iter()
                .for_each(|xy| {
                    grid.insert(xy);
                });
            }
            Rocks::L => {
                [
                    XY { x, y: y },
                    XY { x: x + 1, y: y },
                    XY { x: x + 2, y: y },
                    XY { x: x + 2, y: y + 1 },
                    XY { x: x + 2, y: y + 2 },
                ]
                .into_iter()
                .for_each(|xy| {
                    grid.insert(xy);
                });
            }
            Rocks::I => {
                [
                    XY { x, y: y },
                    XY { x, y: y + 1 },
                    XY { x, y: y + 2 },
                    XY { x, y: y + 3 },
                ]
                .into_iter()
                .for_each(|xy| {
                    grid.insert(xy);
                });
            }
            Rocks::Square => {
                [
                    XY { x, y: y },
                    XY { x, y: y + 1 },
                    XY { x: x + 1, y: y },
                    XY { x: x + 1, y: y + 1 },
                ]
                .into_iter()
                .for_each(|xy| {
                    grid.insert(xy);
                });
            }
        }
    }
    fn can_move_down(self, XY { x, y }: XY, grid: &HashSet<XY>) -> bool {
        match self {
            Rocks::Dash => [
                XY { x, y: y - 1 },
                XY { x: x + 1, y: y - 1 },
                XY { x: x + 2, y: y - 1 },
                XY { x: x + 3, y: y - 1 },
            ]
            .into_iter()
            .all(|xy| !grid.contains(&xy)),
            Rocks::Plus => [
                XY { x, y: y },
                XY { x: x + 1, y: y - 1 },
                XY { x: x + 2, y: y },
            ]
            .into_iter()
            .all(|xy| !grid.contains(&xy)),
            Rocks::L => [
                XY { x, y: y - 1 },
                XY { x: x + 1, y: y - 1 },
                XY { x: x + 2, y: y - 1 },
            ]
            .into_iter()
            .all(|xy| !grid.contains(&xy)),
            Rocks::I => !grid.contains(&XY { x, y: y - 1 }),
            Rocks::Square => [XY { x, y: y - 1 }, XY { x: x + 1, y: y - 1 }]
                .into_iter()
                .all(|xy| !grid.contains(&xy)),
        }
    }
    fn can_move_left(self, XY { x, y }: XY, grid: &HashSet<XY>) -> bool {
        if x == 0 {
            return false;
        }
        match self {
            Rocks::Dash => !grid.contains(&XY { x: x - 1, y }),
            Rocks::Plus => [
                XY { x, y: y },
                XY { x: x - 1, y: y + 1 },
                XY { x, y: y + 2 },
            ]
            .into_iter()
            .all(|xy| !grid.contains(&xy)),
            Rocks::L => [
                XY { x: x - 1, y },
                XY { x: x + 1, y: y + 1 },
                XY { x: x + 1, y: y + 2 },
            ]
            .into_iter()
            .all(|xy| !grid.contains(&xy)),
            Rocks::I => [
                XY { x: x - 1, y },
                XY { x: x - 1, y: y + 1 },
                XY { x: x - 1, y: y + 2 },
                XY { x: x - 1, y: y + 3 },
            ]
            .into_iter()
            .all(|xy| !grid.contains(&xy)),
            Rocks::Square => [XY { x: x - 1, y }, XY { x: x - 1, y: y + 1 }]
                .into_iter()
                .all(|xy| !grid.contains(&xy)),
        }
    }
    fn can_move_right(self, XY { x, y }: XY, grid: &HashSet<XY>) -> bool {
        if x + self.width() == 7 {
            return false;
        }
        match self {
            Rocks::Dash => !grid.contains(&XY { x: x + 4, y }),
            Rocks::Plus => [
                XY { x: x + 2, y: y },
                XY { x: x + 3, y: y + 1 },
                XY { x: x + 2, y: y + 2 },
            ]
            .into_iter()
            .all(|xy| !grid.contains(&xy)),
            Rocks::L => [
                XY { x: x + 3, y },
                XY { x: x + 3, y: y + 1 },
                XY { x: x + 3, y: y + 2 },
            ]
            .into_iter()
            .all(|xy| !grid.contains(&xy)),
            Rocks::I => [
                XY { x: x + 1, y },
                XY { x: x + 1, y: y + 1 },
                XY { x: x + 1, y: y + 2 },
                XY { x: x + 1, y: y + 3 },
            ]
            .into_iter()
            .all(|xy| !grid.contains(&xy)),
            Rocks::Square => [XY { x: x + 2, y }, XY { x: x + 2, y: y + 1 }]
                .into_iter()
                .all(|xy| !grid.contains(&xy)),
        }
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> usize {
    let mut jet_stream = input.chars().cycle();
    let rock_stream = Rocks::iter().take(176);
    let mut grid = HashSet::new();
    for x in 0..7 {
        grid.insert(XY { x, y: 0 });
    }
    let mut tallest_point = 0;
    for rock in rock_stream {
        let mut pos = XY {
            x: 2,
            y: tallest_point + 4,
        };
        loop {
            match jet_stream.next().unwrap() {
                '>' if rock.can_move_right(pos, &grid) => pos.x += 1,
                '<' if rock.can_move_left(pos, &grid) => pos.x -= 1,
                _ => {}
            }
            if rock.can_move_down(pos, &grid) {
                pos.y -= 1;
            } else {
                rock.landed(pos, &mut grid);
                tallest_point = tallest_point.max(pos.y + rock.height() - 1);
                break;
            }
        }
    }
    tallest_point
}

fn top_height_delta(tallest_point: usize, grid: &HashSet<XY>) -> Option<u32> {
    let mut heights = 0u32;
    for x in 0..7 {
        let y_offset = (0..16).position(|y_offset| {
            grid.contains(&XY {
                x,
                y: tallest_point - y_offset,
            })
        })?;
        heights |= (y_offset as u32) << (4 * x);
    }
    Some(heights)
}
#[derive(Debug)]
struct RepeatData {
    start_of_repeat_rock_idx: usize,
    end_of_repeat_rock_idx: usize,
    start_of_repeat_tallest_point: usize,
    end_of_repeat_tallest_point: usize,
    height_deltas: u32,
    subsequent_height_increases: Vec<usize>,
}
fn find_repeating_loop(
    rock_stream: impl Iterator<Item = Rocks>,
    mut jet_stream: impl Iterator<Item = char>,
) -> RepeatData {
    let mut grid = HashSet::new();
    struct RepeatRecord {
        rock_idx: usize,
        size_between: Option<usize>,
        tallest_point: usize,
        seen_count: usize,
    }
    enum RepeatState {
        Searching(HashMap<u32, RepeatRecord>),
        TrackingHeights(RepeatData),
    }
    let mut repeat_state = RepeatState::Searching(HashMap::new());
    for x in 0..7 {
        grid.insert(XY { x, y: 0 });
    }
    let mut tallest_point = 0;
    for (rock_idx, rock) in rock_stream.enumerate() {
        let mut pos = XY {
            x: 2,
            y: tallest_point + 4,
        };
        loop {
            match jet_stream.next().unwrap() {
                '>' if rock.can_move_right(pos, &grid) => pos.x += 1,
                '<' if rock.can_move_left(pos, &grid) => pos.x -= 1,
                _ => {}
            }
            if rock.can_move_down(pos, &grid) {
                pos.y -= 1;
            } else {
                rock.landed(pos, &mut grid);
                tallest_point = tallest_point.max(pos.y + rock.height() - 1);
                repeat_state = match repeat_state {
                    RepeatState::Searching(mut tops) if rock == Rocks::Dash => {
                        if let Some(height_deltas) = top_height_delta(tallest_point, &grid) {
                            let new_state = if let Some(RepeatRecord {
                                rock_idx: prior_rock_idx,
                                tallest_point: prior_tallest_point,
                                seen_count,
                                size_between,
                            }) = tops.get_mut(&height_deltas)
                            {
                                let diff = rock_idx - *prior_rock_idx;
                                let tmp_rock_idx = *prior_rock_idx;
                                let tmp_tallest_point = *prior_tallest_point;
                                *prior_rock_idx = rock_idx;
                                *prior_tallest_point = tallest_point;
                                if let Some(between) = size_between {
                                    if diff != *between {
                                        *seen_count = 0;
                                        RepeatState::Searching(tops)
                                    } else {
                                        *seen_count += 1;
                                        if *seen_count > 100 {
                                            RepeatState::TrackingHeights(RepeatData {
                                                start_of_repeat_rock_idx: tmp_rock_idx,
                                                end_of_repeat_rock_idx: rock_idx,
                                                start_of_repeat_tallest_point: tmp_tallest_point,
                                                end_of_repeat_tallest_point: tallest_point,
                                                height_deltas,
                                                subsequent_height_increases: Vec::new(),
                                            })
                                        } else {
                                            RepeatState::Searching(tops)
                                        }
                                    }
                                } else {
                                    *size_between = Some(diff);
                                    RepeatState::Searching(tops)
                                }
                            } else {
                                tops.insert(
                                    height_deltas,
                                    RepeatRecord {
                                        rock_idx,
                                        tallest_point,
                                        seen_count: 0,
                                        size_between: None,
                                    },
                                );
                                RepeatState::Searching(tops)
                            };
                            new_state
                        } else {
                            RepeatState::Searching(tops)
                        }
                    }
                    RepeatState::TrackingHeights(mut repeat_data) => {
                        if Some(repeat_data.height_deltas) == top_height_delta(tallest_point, &grid)
                        {
                            return repeat_data;
                        }
                        repeat_data
                            .subsequent_height_increases
                            .push(tallest_point - repeat_data.end_of_repeat_tallest_point);
                        RepeatState::TrackingHeights(repeat_data)
                    }
                    RepeatState::Searching(x) => RepeatState::Searching(x),
                };
                break;
            }
        }
    }
    unreachable!()
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let jet_stream = input.chars().cycle();
    let rock_stream = Rocks::iter();
    let repeat_record = find_repeating_loop(rock_stream, jet_stream);
    const NUM_ROCKS: usize = 1_000_000_000_000;
    let num_rocks_before_repeat_loop = repeat_record.start_of_repeat_rock_idx + 1;
    let height_before_repeat_loop = repeat_record.start_of_repeat_tallest_point;
    let num_rocks_in_repeat_loop =
        repeat_record.end_of_repeat_rock_idx - repeat_record.start_of_repeat_rock_idx;
    let height_of_repeat_loop =
        repeat_record.end_of_repeat_tallest_point - repeat_record.start_of_repeat_tallest_point;

    let remaining = NUM_ROCKS - num_rocks_before_repeat_loop;
    let loops = remaining / num_rocks_in_repeat_loop;
    let remaining = remaining % num_rocks_in_repeat_loop;
    height_before_repeat_loop
        + (height_of_repeat_loop * loops)
        + repeat_record.subsequent_height_increases[remaining - 1]
}
