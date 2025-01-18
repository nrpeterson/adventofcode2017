use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use adventofcode2017::build_main;

struct Scanner {
    depth: usize,
    range: usize
}

fn parse_input(input: &str) -> IResult<&str, Vec<Scanner>> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    separated_list1(
        newline,
        map(
            separated_pair(number, tag(": "), number),
            |(depth, range)| Scanner { depth, range }
        )
    )(input)
}

fn part1(input: &str) -> usize {
    let scanners = parse_input(input).unwrap().1;

    scanners.into_iter()
        .filter(|scanner| scanner.depth % (2 * scanner.range - 2) == 0)
        .map(|scanner| scanner.depth * scanner.range)
        .sum()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn part2(input: &str) -> usize {
    let scanners = parse_input(input).unwrap().1;

    (0..).filter(|&i| {
        scanners.iter().all(|scanner| (i + scanner.depth) % (2 * scanner.range - 2) != 0)
    }).nth(0).unwrap()

}

build_main!("day13.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0: 3
1: 2
4: 4
6: 4";

        assert_eq!(part1(input), 24);
    }
}