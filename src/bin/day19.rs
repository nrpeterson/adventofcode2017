use std::ops::{Add, AddAssign, Index};
use itertools::Itertools;
use adventofcode2017::build_main;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pair(isize, isize);

impl Pair {
    fn dot(&self, other: Pair) -> isize {
        self.0 * other.0 + self.1 * other.1
    }
}

impl AddAssign for Pair {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Add for Pair {
    type Output = Pair;
    fn add(self, rhs: Self) -> Self::Output {
        Pair(self.0 + rhs.0, self.1 + rhs.1)
    }
}

struct Diagram(Vec<Vec<char>>);

impl Diagram {
    fn get(&self, Pair(i, j): Pair) -> char {
        self.0.get(i as usize).and_then(|row| row.get(j as usize).cloned()).unwrap_or(' ')
    }
}

fn solve(input: &str) -> (String, usize) {
    let diagram = Diagram(
        input.lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec()
    );

    let j_init = diagram.0[0].iter().position(|&c| c == '|').unwrap();
    let mut pos = Pair(0, j_init as isize);
    let mut dir = Pair(1, 0);

    let mut seen = Vec::new();

    let mut move_counts = 0;

    loop {
        match diagram.get(pos) {
            '+' => {
                // Do we change direction?
                dir = {
                    if dir.0 == 0 {
                        if diagram.get(pos + Pair(1, 0)) != ' ' {
                            Pair(1, 0)
                        }
                        else {
                            Pair(-1, 0)
                        }
                    }
                    else {
                        if diagram.get(pos + Pair(0, 1)) != ' ' {
                            Pair(0, 1)
                        }
                        else {
                            Pair(0, -1)
                        }
                    }
                };
                pos += dir;
                move_counts += 1;
            },
            c if c.is_alphabetic() => {
                seen.push(c);
                pos += dir;
                move_counts += 1;
            },
            _ => {
                pos += dir;
                move_counts += 1;
            }
        }

        if diagram.get(pos) == ' ' {
            return (seen.into_iter().collect(), move_counts)
        }
    }
}

fn part1(input: &str) -> String {
    solve(input).0
}

fn part2(input: &str) -> usize {
    solve(input).1
}

build_main!("day19.txt", "Part 1" => part1, "Part 2" => part2);