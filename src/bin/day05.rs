use itertools::Itertools;
use adventofcode2017::build_main;

fn part1(input: &str) -> usize {
    let mut jumps = input.lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect_vec();

    let mut count = 0;
    let mut i = 0isize;
    let mut j = i as usize;

    while i >= 0 && (i as usize) < jumps.len() {
        j = i as usize;
        let cur = jumps[j];
        jumps[j] += 1;
        i += cur;
        count += 1;
    }

    count
}

fn part2(input: &str) -> usize {
    let mut jumps = input.lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect_vec();

    let mut count = 0;
    let mut i = 0isize;

    while i >= 0 && (i as usize) < jumps.len() {
        let j = i as usize;
        let cur = jumps[j];

        if jumps[j] >= 3 {
            jumps[j] -= 1;
        }
        else {
            jumps[j] += 1;
        }
        i += cur;
        count += 1;
    }

    count
}

build_main!("day05.txt", "Part 1" => part1, "Part 2" => part2);