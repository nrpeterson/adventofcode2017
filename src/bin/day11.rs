use std::ops::AddAssign;
use itertools::Itertools;
use adventofcode2017::build_main;

#[derive(Debug, Copy, Clone)]
struct HexAxial(isize, isize);

impl HexAxial {
    fn num_steps(&self) -> isize {
        let &HexAxial(q, r) = self;
        (q.abs() + (q + r).abs() + r.abs()) / 2
    }
}

impl AddAssign for HexAxial {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

struct Path<'a> {
    cur: Option<HexAxial>,
    directions: Vec<&'a str>,
    i: usize
}

impl<'a> Path<'a> {
    fn new(directions: Vec<&'a str>) -> Path<'a> {
        let cur = Some(HexAxial(0, 0));
        let i = 0;

        Path { cur, directions, i }
    }
}

impl Iterator for Path<'_> {
    type Item = HexAxial;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.cur;

        if self.i == self.directions.len() {
            self.cur = None;
        }
        else {
            let step = self.directions[self.i];
            let delta = match step {
                "n" => HexAxial(0, -1),
                "s" => HexAxial(0, 1),
                "ne" => HexAxial(1, -1),
                "sw" => HexAxial(-1, 1),
                "se" => HexAxial(1, 0),
                "nw" => HexAxial(-1, 0),
                _ => unreachable!(),
            };

            self.cur.iter_mut().for_each( |c| *c += delta);
            self.i += 1;
        }

        result
    }
}

fn part1(input: &str) -> isize {
    let directions = input.split(",").collect_vec();
    let last = Path::new(directions).last().unwrap();

    last.num_steps()
}

fn part2(input: &str) -> isize {
    let directions = input.split(",").collect_vec();

    Path::new(directions)
        .map(|hex| hex.num_steps())
        .max()
        .unwrap()
}

build_main!("day11.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        for (i, o) in [("ne,ne,ne", 3), ("ne,ne,sw,sw", 0), ("ne,ne,s,s", 2)] {
            assert_eq!(part1(i), o);
        }
    }
}