use std::collections::HashMap;
use std::ops::{Add, AddAssign};
use adventofcode2017::build_main;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos(isize, isize);

impl Pos {
    fn rotate(&self) -> Pos {
        Pos(-self.1, self.0)
    }

    fn neighbors(&self) -> [Pos; 8] {
        [
            *self + Pos(1, 0),
            *self + Pos(1, 1),
            *self + Pos(0, 1),
            *self + Pos(-1, 1),
            *self + Pos(-1, 0),
            *self + Pos(-1, -1),
            *self + Pos(0, -1),
            *self + Pos(1, -1)
        ]

    }
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

struct RingIter {
    i: usize,
    pos: Pos,
    delta: Pos
}

impl RingIter {
    fn new() -> RingIter {
        RingIter { i: 1, pos: Pos(0, 0), delta: Pos(0, -1) }
    }
}

impl Iterator for RingIter {
    type Item = (usize, Pos);
    fn next(&mut self) -> Option<Self::Item> {
        let result = Some((self.i, self.pos));

        let Pos(x, y) = self.pos;
        if x == y || (x < 0 && y == -x) || (x > 0 && x == 1 - y ){
            self.delta = self.delta.rotate();
        }

        self.pos += self.delta;
        self.i += 1;

        result
    }
}

fn part1(input: &str) -> isize {
    let n = input.parse::<usize>().unwrap();
    let Pos(x, y) = RingIter::new().nth(n - 1).unwrap().1;
    x.abs() + y.abs()
}

fn part2(input: &str) -> usize {
    let n = input.parse::<usize>().unwrap();
    let mut results = HashMap::new();

    for (i, pos) in RingIter::new() {
        if i == 1 {
            results.insert(pos, 1usize);
            continue;
        }

        let result = pos.neighbors().into_iter()
            .filter_map(|p| results.get(&p).map(|&n| n))
            .sum::<usize>();

        if result >= n {
            return result;
        }

        results.insert(pos, result);
    }

    unreachable!()
}

build_main!("day03.txt", "Part 1" => part1, "Part 2" => part2);