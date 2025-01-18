use std::collections::HashSet;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{eof, map_res};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated, tuple};
use adventofcode2017::build_main;

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn line(input: &str) -> IResult<&str, Vec<usize>> {
        preceded(
            tuple((digit1, tag(" <-> "))),
            separated_list1(tag(", "), number)
        )(input)
    }

    terminated(separated_list1(newline, line), eof)(input)
}

fn part1(input: &str) -> usize {
    let edges = parse_input(input).unwrap().1;
    let mut stack = vec![0];
    let mut seen = HashSet::new();
    seen.insert(0);

    while let Some(id) = stack.pop() {
        for &neighbor in edges[id].iter() {
            if seen.insert(neighbor) {
                stack.push(neighbor);
            }
        }
    }

    seen.len()
}

fn part2(input: &str) -> usize {
    let edges = parse_input(input).unwrap().1;
    let mut seen = HashSet::new();

    let mut count = 0;

    for i in 0..edges.len() {
        if !seen.contains(&i) {
            count += 1;
            let mut component = HashSet::new();
            component.insert(i);

            let mut stack = vec![i];
            while let Some(id) = stack.pop() {
                for &neighbor in edges[id].iter() {
                    if component.insert(neighbor) {
                        stack.push(neighbor);
                        seen.insert(neighbor);
                    }
                }
            }
        }
    }

    count
}

build_main!("day12.txt", "Part 1" => part1, "Part 2" => part2);