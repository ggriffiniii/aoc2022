use std::{convert::Infallible, str::FromStr};

use aoc_runner_derive::aoc;

#[derive(Debug, Default)]
struct Dir {
    idx: usize,
    parent_dir: Option<usize>,
    child_dirs: Vec<(String, usize)>,
    files: Vec<(String, usize)>,
}

struct Filesystem(Vec<Dir>);
impl FromStr for Filesystem {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut dirs = vec![Dir {
            idx: 0,
            parent_dir: None,
            child_dirs: Vec::new(),
            files: Vec::new(),
        }];
        let mut current_dir = 0;
        for line in input.lines() {
            match line.split_once(' ').unwrap() {
                ("$", cmd) => {
                    if cmd == "ls" {
                        // do nothing
                    } else if let Some(dirname) = cmd.strip_prefix("cd ") {
                        current_dir = match dirname {
                            "/" => 0,
                            ".." => dirs[current_dir].parent_dir.unwrap(),
                            _ => {
                                dirs[current_dir]
                                    .child_dirs
                                    .iter()
                                    .find(|(name, _idx)| name == dirname)
                                    .unwrap()
                                    .1
                            }
                        };
                    }
                }
                ("dir", name) => {
                    let idx = dirs.len();
                    dirs.push(Dir {
                        idx,
                        parent_dir: Some(current_dir),
                        child_dirs: Vec::new(),
                        files: Vec::new(),
                    });
                    dirs[current_dir].child_dirs.push((name.to_owned(), idx));
                }
                (size, filename) => {
                    dirs[current_dir]
                        .files
                        .push((filename.to_owned(), size.parse().unwrap()));
                }
            }
        }
        Ok(Filesystem(dirs))
    }
}

impl Filesystem {
    fn dirs(&self) -> std::slice::Iter<'_, Dir> {
        self.0.iter()
    }

    fn total_size(&self) -> usize {
        self.size_of_dir(&self.0[0])
    }

    fn size_of_dir(&self, dir: &Dir) -> usize {
        self.0[dir.idx]
            .files
            .iter()
            .map(|(_name, size)| size)
            .sum::<usize>()
            + self.0[dir.idx]
                .child_dirs
                .iter()
                .map(|(_name, idx)| self.size_of_dir(&self.0[*idx]))
                .sum::<usize>()
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let fs: Filesystem = input.parse().unwrap();
    fs.dirs()
        .map(|dir| fs.size_of_dir(dir))
        .filter(|&size| size <= 100000)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let fs: Filesystem = input.parse().unwrap();
    let space_used = fs.total_size();
    let unused_space = 70000000 - space_used;
    let min_to_delete = 30000000 - unused_space;
    fs.dirs()
        .map(|dir| fs.size_of_dir(dir))
        .filter(|&size| size >= min_to_delete)
        .min()
        .unwrap()
}
