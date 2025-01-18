use std::cmp::{max, min};
use itertools::Itertools;
use nom::character::complete::{digit1, newline, space1};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::separated_list1;
use adventofcode2017::build_main;

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    separated_list1(
        newline,
        separated_list1(
            space1,
            number
        )
    )(input)
}

fn minmax(input: &[usize]) -> usize {
    let (low, high) = input.iter()
        .fold((usize::MAX, 0), |(low, high), &next| {
            (min(low, next), max(high, next))
        });

    high - low
}

fn part1(input: &str) -> usize {
    let (_, v) = parse_input(input).unwrap();
    v.into_iter()
        .map(|v| minmax(&v))
        .sum()
}

fn part2(input: &str) -> usize {
    let (_, mut v) = parse_input(input).unwrap();

    v.into_iter()
        .map(|mut v| { v.sort(); v })
        .flat_map(|v| v.into_iter().combinations(2).map(|pair| (pair[0], pair[1])))
        .filter(|&(a, b)| b % a == 0)
        .map(|(a, b)| b / a)
        .sum()
}

build_main!("day02.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "5 1 9 5
7 5 3
2 4 6 8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 18);
    }
}