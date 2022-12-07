use std::{cell::RefCell, rc::Rc};

use aoc_runner_derive::aoc;

#[derive(Debug, Clone)]
struct Dir(Rc<RefCell<InnerDir>>);
impl Dir {
    fn dirs(&self) -> DirIter {
        DirIter {
            to_visit: vec![self.clone()],
        }
    }

    fn total_size(&self) -> usize {
        let inner = self.0.borrow();
        inner.files.iter().map(|(_name, size)| size).sum::<usize>()
            + inner
                .child_dirs
                .iter()
                .map(|(_name, dir)| dir.total_size())
                .sum::<usize>()
    }
}

struct DirIter {
    to_visit: Vec<Dir>,
}
impl Iterator for DirIter {
    type Item = Dir;
    fn next(&mut self) -> Option<Self::Item> {
        let next_dir = self.to_visit.pop()?;
        self.to_visit.extend(
            next_dir
                .0
                .borrow()
                .child_dirs
                .iter()
                .map(|(_child_name, dir)| dir.clone()),
        );
        Some(next_dir)
    }
}

#[derive(Debug)]
struct InnerDir {
    parent_dir: Option<Dir>,
    child_dirs: Vec<(String, Dir)>,
    files: Vec<(String, usize)>,
}

fn parse_filesystem(input: &str) -> Dir {
    let root = Dir(Rc::new(RefCell::new(InnerDir {
        parent_dir: None,
        child_dirs: Vec::new(),
        files: Vec::new(),
    })));
    let mut current_dir = root.clone();
    for line in input.lines() {
        match line.split_once(' ').unwrap() {
            ("$", cmd) => {
                if cmd == "ls" {
                    // do nothing
                } else if let Some(dirname) = cmd.strip_prefix("cd ") {
                    current_dir = match dirname {
                        "/" => root.clone(),
                        ".." => current_dir.0.borrow().parent_dir.clone().unwrap(),
                        _ => current_dir
                            .0
                            .borrow()
                            .child_dirs
                            .iter()
                            .find(|(name, _idx)| name == dirname)
                            .unwrap()
                            .1
                            .clone(),
                    };
                }
            }
            ("dir", name) => {
                let child_dir = Dir(Rc::new(RefCell::new(InnerDir {
                    parent_dir: Some(current_dir.clone()),
                    child_dirs: Vec::new(),
                    files: Vec::new(),
                })));
                current_dir
                    .0
                    .borrow_mut()
                    .child_dirs
                    .push((name.to_owned(), child_dir));
            }
            (size, filename) => {
                current_dir
                    .0
                    .borrow_mut()
                    .files
                    .push((filename.to_owned(), size.parse().unwrap()));
            }
        }
    }
    root
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let root_dir = parse_filesystem(input);
    root_dir
        .dirs()
        .map(|dir| dir.total_size())
        .filter(|&size| size <= 100000)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let root_dir = parse_filesystem(input);
    let space_used = root_dir.total_size();
    let unused_space = 70000000 - space_used;
    let min_to_delete = 30000000 - unused_space;
    root_dir
        .dirs()
        .map(|dir| dir.total_size())
        .filter(|&size| size >= min_to_delete)
        .min()
        .unwrap()
}
