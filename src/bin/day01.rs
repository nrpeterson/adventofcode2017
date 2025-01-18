use itertools::Itertools;
use adventofcode2017::build_main;

fn part1(input: &str) -> usize {
    let mut vals = input.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

    vals.push(vals[0]);

    vals.into_iter()
        .tuple_windows()
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| a)
        .sum()
}

fn part2(input: &str) -> usize {
    let vals = input.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

    let n = vals.len() / 2;


    vals.iter().enumerate()
        .map(|(i, x)| (*x, vals[(i + n) % vals.len()]))
        .filter(|&(x, y)| x == y)
        .map(|(x, _)| x)
        .sum()
}

build_main!("day01.txt", "Part 1" => part1, "Part 2" => part2);