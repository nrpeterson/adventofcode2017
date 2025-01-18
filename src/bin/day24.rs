use std::cmp::max;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use adventofcode2017::build_main;

#[derive(Clone)]
struct Bridge {
    used: HashSet<(usize, usize)>,
    last: usize,
    strength: usize
}

impl Bridge {
    fn new() -> Bridge {
        Bridge { used: HashSet::new(), last: 0, strength: 0 }
    }

    fn extend(&mut self, (a, b): (usize, usize)) {
        self.used.insert((a, b));
        self.strength += a + b;

        if a == self.last {
            self.last = b;
        }
        else if b == self.last {
            self.last = a;
        }
        else {
            panic!("Ruh-roh.");
        }
    }
}

fn parse_input(input: &str) -> HashMap<usize, Vec<(usize, usize)>> {
    input.lines()
        .map(|line| {
            let parts = line.split('/').collect_vec();
            (parts[0].parse::<usize>().unwrap(), parts[1].parse::<usize>().unwrap())
        })
        .flat_map(|(a, b)| [(a, (a, b)), (b, (a, b))])
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.entry(k).or_default().push(v);
            acc
        })
}

fn part1(input: &str) -> usize {
    let parts = parse_input(input);

    let mut stack = Vec::new();
    stack.push(Bridge::new());

    let mut best = 0;

    while let Some(bridge) = stack.pop() {
        if let Some(ps) = parts.get(&bridge.last) {
            ps.iter().cloned()
                .filter(|part| !bridge.used.contains(part))
                .for_each(|part| {
                    let mut b = bridge.clone();
                    b.extend(part);
                    stack.push(b);
                })
        }

        best = max(best, bridge.strength);
    }

    best
}

fn part2(input: &str) -> usize {
    let parts = parse_input(input);

    let mut stack = Vec::new();
    stack.push(Bridge::new());

    let mut best = 0;
    let mut best_len = 0;

    while let Some(bridge) = stack.pop() {
        if let Some(ps) = parts.get(&bridge.last) {
            ps.iter().cloned()
                .filter(|part| !bridge.used.contains(part))
                .for_each(|part| {
                    let mut b = bridge.clone();
                    b.extend(part);
                    stack.push(b);
                })
        }
        if bridge.used.len() > best_len {
            best_len = bridge.used.len();
            best = bridge.strength;
        }
        else if bridge.used.len() == best_len {
            best = max(best, bridge.strength);
        }
    }

    best
}

build_main!("day24.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        assert_eq!(part2(input), 19);
    }
}