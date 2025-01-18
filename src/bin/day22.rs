use std::collections::{HashMap, HashSet};
use std::ops::AddAssign;
use itertools::Itertools;
use adventofcode2017::build_main;
use crate::State::{Flagged, Infected, Weakened};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Pair(isize, isize);

impl AddAssign for Pair {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

struct Board1 {
    infected: HashSet<Pair>,
    carrier_pos: Pair,
    carrier_dir: Pair
}

impl Board1 {
    fn new(input: &str) -> Board1 {
        let data = input.lines()
            .map(|row| row.chars().map(|c| c == '#').collect_vec())
            .collect_vec();
        let rows = data.len() as isize;
        let cols = data[0].len() as isize;
        let carrier_pos = Pair((rows - 1) / 2, (cols - 1) / 2);
        let carrier_dir = Pair(-1, 0);

        let infected = data.into_iter().enumerate()
            .flat_map(|(i, row)| {
                row.into_iter().enumerate()
                    .filter_map(move |(j, b)| {
                        if b { Some(Pair(i as isize, j as isize)) } else { None }
                    })
            })
            .collect();

        Board1 { infected, carrier_pos, carrier_dir }
    }
}

impl Iterator for Board1 {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let Pair(x, y) = self.carrier_dir;
        let did_infect = if self.infected.contains(&self.carrier_pos) {
            self.carrier_dir = Pair(y, -x);
            self.infected.remove(&self.carrier_pos);
            false
        }
        else {
            self.carrier_dir = Pair(-y, x);
            self.infected.insert(self.carrier_pos);
            true
        };

        self.carrier_pos += self.carrier_dir;

        Some(did_infect)
    }
}

fn part1(input: &str) -> usize {
    Board1::new(input)
        .take(10000)
        .filter(|&infected| infected)
        .count()
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum State { Weakened, Infected, Flagged }

struct Board2 {
    infected: HashMap<Pair, State>,
    carrier_pos: Pair,
    carrier_dir: Pair
}

impl Board2 {
    fn new(input: &str) -> Board2 {
        let data = input.lines()
            .map(|row| row.chars().map(|c| c == '#').collect_vec())
            .collect_vec();
        let rows = data.len() as isize;
        let cols = data[0].len() as isize;
        let carrier_pos = Pair((rows - 1) / 2, (cols - 1) / 2);
        let carrier_dir = Pair(-1, 0);

        let infected = data.into_iter().enumerate()
            .flat_map(|(i, row)| {
                row.into_iter().enumerate()
                    .filter_map(move |(j, b)| {
                        if b { Some((Pair(i as isize, j as isize), Infected)) } else { None }
                    })
            })
            .collect();

        Board2 { infected, carrier_pos, carrier_dir }
    }
}

impl Iterator for Board2 {
    type Item = Option<State>;

    fn next(&mut self) -> Option<Self::Item> {
        let Pair(x, y) = self.carrier_dir;

        let result = match self.infected.get(&self.carrier_pos) {
            None => {
                self.carrier_dir = Pair(-y, x);
                Some(Weakened)
            },
            Some(Weakened) => {
                Some(Infected)
            },
            Some(Infected) => {
                self.carrier_dir = Pair(y, -x);
                Some(Flagged)
            },
            Some(Flagged) => {
                self.carrier_dir = Pair(-x, -y);
                None
            }
        };

        match result {
            None => { self.infected.remove(&self.carrier_pos); },
            Some(x) => { self.infected.insert(self.carrier_pos, x); }
        }

        self.carrier_pos += self.carrier_dir;

        Some(result)
    }
}

fn part2(input: &str) -> usize {
    Board2::new(input).take(10000000)
        .filter_map(|state| state)
        .filter(|&s| s == Infected)
        .count()
}

build_main!("day22.txt", "Part 1" => part1, "Part 2" => part2);