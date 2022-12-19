use std::{convert::Infallible, str::FromStr};

use aoc_runner_derive::aoc;

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

#[derive(Debug)]
struct Blueprint {
    costs: [[u16; 4]; 4],
    max_robots: [u16; 4],
}
impl FromStr for Blueprint {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, input) = input.split_once("Each ore robot costs ").unwrap();
        let (ore_robot_ore_cost, input) = input.split_once(" ore. Each clay robot costs ").unwrap();
        let (clay_robot_ore_cost, input) = input
            .split_once(" ore. Each obsidian robot costs ")
            .unwrap();
        let (obsidian_robot_ore_cost, input) = input.split_once(" ore and ").unwrap();
        let (obsidian_robot_clay_cost, input) =
            input.split_once(" clay. Each geode robot costs ").unwrap();
        let (geode_robot_ore_cost, input) = input.split_once(" ore and ").unwrap();
        let (geode_robot_obsidian_cost, _) = input.split_once(" obsidian.").unwrap();

        let mut costs = [[0; 4]; 4];
        costs[ORE][ORE] = ore_robot_ore_cost.parse().unwrap();
        costs[CLAY][ORE] = clay_robot_ore_cost.parse().unwrap();
        costs[OBSIDIAN][ORE] = obsidian_robot_ore_cost.parse().unwrap();
        costs[OBSIDIAN][CLAY] = obsidian_robot_clay_cost.parse().unwrap();
        costs[GEODE][ORE] = geode_robot_ore_cost.parse().unwrap();
        costs[GEODE][OBSIDIAN] = geode_robot_obsidian_cost.parse().unwrap();

        let mut max_robots = [u16::MAX; 4];
        for mineral_type in [ORE, CLAY, OBSIDIAN] {
            max_robots[mineral_type] = costs
                .iter()
                .copied()
                .map(|robot_costs| robot_costs[mineral_type])
                .max()
                .unwrap();
        }
        Ok(Blueprint { costs, max_robots })
    }
}

fn max_geodes(blueprint: &Blueprint, minutes_remaining: u8) -> u16 {
    #[derive(Debug, Clone)]
    struct State {
        minerals: [u16; 4],
        num_robots: [u16; 4],
    }
    fn _max_geodes(blueprint: &Blueprint, state: State, minutes_remaining: u8) -> u16 {
        let mut max_geodes = 0;
        for robot in 0..4 {
            if state.num_robots[robot] == blueprint.max_robots[robot] {
                continue;
            }

            let costs = &blueprint.costs[robot];

            let minutes_required_to_build_robot = (0..3)
                .map(|mineral_type| {
                    if costs[mineral_type] <= state.minerals[mineral_type] {
                        0
                    } else if state.num_robots[mineral_type] == 0 {
                        minutes_remaining + 1
                    } else {
                        ((costs[mineral_type] - state.minerals[mineral_type]
                            + state.num_robots[mineral_type]
                            - 1)
                            / state.num_robots[mineral_type]) as u8
                    }
                })
                .max()
                .unwrap();

            if minutes_required_to_build_robot + 1 >= minutes_remaining {
                continue;
            }

            let mut state = state.clone();

            for mineral_type in 0..4 {
                state.minerals[mineral_type] += state.num_robots[mineral_type]
                    * (minutes_required_to_build_robot as u16 + 1)
                    - costs[mineral_type];
            }
            state.num_robots[robot] += 1;

            let best_case_scenario = ((minutes_remaining as u16 - 1) * minutes_remaining as u16)
                / 2
                + state.minerals[GEODE]
                + minutes_remaining as u16 * state.num_robots[GEODE];
            if best_case_scenario < max_geodes {
                continue;
            }
            max_geodes = max_geodes.max(_max_geodes(
                blueprint,
                state,
                minutes_remaining - minutes_required_to_build_robot - 1,
            ))
        }
        max_geodes.max(state.minerals[GEODE] + state.num_robots[GEODE] * minutes_remaining as u16)
    }

    _max_geodes(
        blueprint,
        State {
            minerals: [0, 0, 0, 0],
            num_robots: [1, 0, 0, 0],
        },
        minutes_remaining,
    )
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|x| x.parse::<Blueprint>().unwrap())
        .enumerate()
        .map(|(idx, blueprint)| {
            let max_geodes = max_geodes(&blueprint, 24) as usize;
            (idx + 1) * max_geodes
        })
        .sum()
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .take(3)
        .map(|x| x.parse::<Blueprint>().unwrap())
        .map(|blueprint| max_geodes(&blueprint, 32) as usize)
        .product()
}
