use crate::DanceMove::{Exchange, Partner, Spin};
use adventofcode2017::build_main;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{anychar, char, digit1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::collections::HashSet;

enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char)
}

fn parse_input(input: &str) -> IResult<&str, Vec<DanceMove>> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    separated_list1(
        char(','),
        alt((
            map(preceded(char('s'), number), Spin),
            map(
                preceded(char('x'), separated_pair(number, char('/'), number)),
                |(a, b)| Exchange(a, b)
            ),
            map(
                preceded(char('p'), separated_pair(anychar, char('/'), anychar)),
                |(a, b)| Partner(a, b)
            )
        ))
    )(input)
}

fn part1(input: &str) -> String {
    let moves = parse_input(input).unwrap().1;

    let mut programs = "abcdefghijklmnop".chars().collect_vec();

    for mov in moves {
        match mov {
            Spin(x) => {
                programs.rotate_right(x);
            },
            Exchange(a, b) => {
                programs.swap(a, b);
            },
            Partner(a, b) => {
                let i = programs.iter().position(|&c| c == a).unwrap();
                let j = programs.iter().position(|&c| c == b).unwrap();
                programs.swap(i, j);
            }
        }
    }

    programs.into_iter().join("")
}

fn cycles(perm: &[usize]) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut seen = HashSet::new();

    for i in 0..perm.len() {
        if seen.contains(&i) {
            continue;
        }

        let mut cycle = vec![i];
        let mut j = perm[i];
        while j != i {
            seen.insert(j);
            cycle.push(j);
            j = perm[j];
        }
        result.push(cycle);
    }

    result
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn part2(input: &str) -> String {
    let moves = parse_input(input).unwrap().1;

    let mut perm = (0..16).collect_vec();

    for mov in moves {
        match mov {
            Spin(x) => perm.rotate_right(x),
            Exchange(a, b) => perm.swap(a, b),
            _ => ()
        }
    }

    let c = cycles(&perm);
    let period = c.iter().map(|cycle| cycle.len())
        .filter(|&n| n > 0)
        .reduce(|x, y| lcm(x, y))
        .unwrap();

    let mut result = (0..16).collect_vec();

    for _ in 0..(1000000000 % period) {
        for i in 0..16 {
            result[i] = perm[result[i]];
        }
    }

    result.into_iter()
        .map(|i| ((i as u8) + ('a' as u8)) as char)
        .collect()
}

build_main!("day16.txt", "Part 1" => part1, "Part 2" => part2);