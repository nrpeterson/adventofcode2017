use std::collections::{HashMap, HashSet};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{anychar, char as ch, digit1, multispace1, newline, space0};
use nom::combinator::{all_consuming, map, map_res, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, tuple};
use adventofcode2017::build_main;

struct Result {
    to_write: bool,
    offset: isize,
    next_state: char
}

struct Machine {
    rules: HashMap<char, (Result, Result)>,
    tape: HashSet<isize>,
    state: char,
    steps_remaining: usize,
    cur_position: isize
}

impl Iterator for Machine {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps_remaining == 0 {
            return None;
        }

        let rule = if self.tape.contains(&self.cur_position) {
            &self.rules[&self.state].1
        }
        else {
            &self.rules[&self.state].0
        };

        if rule.to_write == true {
            self.tape.insert(self.cur_position);
        }
        else {
            self.tape.remove(&self.cur_position);
        }

        self.cur_position += rule.offset;
        self.state = rule.next_state;
        self.steps_remaining -= 1;

        Some(self.tape.len())
    }
}

fn parse_input(input: &str) -> IResult<&str, Machine> {
    fn bit(input: &str) -> IResult<&str, bool> {
        alt((
            value(false, ch('0')),
            value(true, ch('1'))
            ))(input)
    }

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn offset(input: &str) -> IResult<&str, isize> {
        alt((
            value(-1, tag("left")),
            value(1, tag("right"))
            ))(input)
    }

    fn result(input: &str) -> IResult<&str, Result> {
        map(
            tuple((
                delimited(pair(space0, tag("- Write the value ")), bit, pair(ch('.'), newline)),
                delimited(pair(space0, tag("- Move one slot to the ")), offset, pair(ch('.'), newline)),
                delimited(pair(space0, tag("- Continue with state ")), anychar, ch('.'))
            )),
            |(to_write, offset, next_state)| Result { to_write, offset, next_state }
        )(input)
    }

    fn item(input: &str) -> IResult<&str, (char, Result, Result)> {
        tuple((
            delimited(tag("In state "), anychar, pair(ch(':'), newline)),
            preceded(tuple((is_not(":"), ch(':'), newline)), result),
            preceded(tuple((is_not(":"), ch(':'), newline)), result)
        ))(input)
    }

    map(
        all_consuming(
            tuple((
                delimited(tag("Begin in state "), anychar, pair(ch('.'), newline)),
                delimited(tag("Perform a diagnostic checksum after "), number, pair(tag(" steps."), multispace1)),
                separated_list1(multispace1, item)
            ))
        ),
        |(state, steps_remaining, v)| {
            let rules = v.into_iter()
                .map(|(a, r1, r2)| (a, (r1, r2)))
                .collect();

            Machine { rules, tape: HashSet::new(), state, steps_remaining, cur_position: 0 }
        }
    )(input)
}

fn part1(input: &str) -> usize {
    let mut machine = parse_input(input).unwrap().1;
    machine.last().unwrap()
}

build_main!("day25.txt", "Part 1" => part1);