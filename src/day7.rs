use std::{cell::RefCell, rc::Rc};

use aoc_runner_derive::aoc;

#[derive(Debug, Default)]
struct Dir {
    parent_dir: Option<Rc<RefCell<Dir>>>,
    child_dirs: Vec<(String, Rc<RefCell<Dir>>)>,
    files: Vec<(String, usize)>,
}

fn dirs(top_dir: Rc<RefCell<Dir>>) -> Vec<Rc<RefCell<Dir>>> {
    // this would be better as an iterator, but I don't have the energy
    let mut all_dirs = vec![Rc::clone(&top_dir)];
    for (_name, dir) in top_dir.borrow().child_dirs.iter() {
        all_dirs.extend(dirs(Rc::clone(dir)));
    }
    all_dirs
}
impl Dir {
    fn total_size(&self) -> usize {
        self.files.iter().map(|(_name, size)| size).sum::<usize>()
            + self
                .child_dirs
                .iter()
                .map(|(_name, dir)| dir.borrow().total_size())
                .sum::<usize>()
    }
}

fn parse_filesystem(input: &str) -> Rc<RefCell<Dir>> {
    let root = Rc::new(RefCell::new(Dir {
        parent_dir: None,
        child_dirs: Vec::new(),
        files: Vec::new(),
    }));
    let mut current_dir = Rc::clone(&root);
    for line in input.lines() {
        match line.split_once(' ').unwrap() {
            ("$", cmd) => {
                if cmd == "ls" {
                    // do nothing
                } else if let Some(dirname) = cmd.strip_prefix("cd ") {
                    current_dir = match dirname {
                        "/" => Rc::clone(&root),
                        ".." => Rc::clone(current_dir.borrow().parent_dir.as_ref().unwrap()),
                        _ => Rc::clone(
                            &current_dir
                                .borrow()
                                .child_dirs
                                .iter()
                                .find(|(name, _idx)| name == dirname)
                                .unwrap()
                                .1,
                        ),
                    };
                }
            }
            ("dir", name) => {
                let child_dir = Rc::new(RefCell::new(Dir {
                    parent_dir: Some(Rc::clone(&current_dir)),
                    child_dirs: Vec::new(),
                    files: Vec::new(),
                }));
                current_dir
                    .borrow_mut()
                    .child_dirs
                    .push((name.to_owned(), child_dir));
            }
            (size, filename) => {
                current_dir
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
    dirs(root_dir)
        .iter()
        .map(|dir| dir.borrow().total_size())
        .filter(|&size| size <= 100000)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let root_dir = parse_filesystem(input);
    let space_used = root_dir.borrow().total_size();
    let unused_space = 70000000 - space_used;
    let min_to_delete = 30000000 - unused_space;
    dirs(root_dir)
        .iter()
        .map(|dir| dir.borrow().total_size())
        .filter(|&size| size >= min_to_delete)
        .min()
        .unwrap()
}
