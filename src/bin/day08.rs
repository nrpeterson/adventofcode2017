use std::cmp::max;
use std::collections::HashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, newline, space1};
use nom::combinator::{map, map_res, opt, recognize, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, tuple};
use adventofcode2017::build_main;

#[derive(Copy, Clone)]
enum Comp { Le, Lt, Eq, Ne, Gt, Ge }
use Comp::*;

enum Instruction {
    Inc(isize),
    Dec(isize)
}
use Instruction::*;

struct Condition<'a> {
    register: &'a str,
    comp: Comp,
    target: isize
}

struct Rule<'a> {
    register: &'a str,
    instruction: Instruction,
    condition: Condition<'a>
}

fn parse_input(input: &str) -> IResult<&str, Vec<Rule>> {
    fn number(input: &str) -> IResult<&str, isize> {
        map_res(
            recognize(tuple((opt(char('-')), digit1))),
            |s: &str| s.parse::<isize>()
        )(input)
    }

    fn instruction(input: &str) -> IResult<&str, Instruction> {
        alt((
            map(preceded(tag("inc "), number), Inc),
            map(preceded(tag("dec "), number), Dec)
        ))(input)
    }

    fn comp(input: &str) -> IResult<&str, Comp> {
        alt((
            value(Le, tag("<=")),
            value(Lt, tag("<")),
            value(Eq, tag("==")),
            value(Ne, tag("!=")),
            value(Ge, tag(">=")),
            value(Gt, tag(">")),
        ))(input)
    }

    fn condition(input: &str) -> IResult<&str, Condition> {
        map(
            preceded(
                tag("if "),
                tuple((
                    alpha1,
                    delimited(space1, comp, space1),
                    number
                ))
            ),
            |(register, comp, target)| Condition { comp, register, target }
        )(input)
    }

    fn rule(input: &str) -> IResult<&str, Rule> {
        map(
            tuple((
                alpha1,
                delimited(space1, instruction, space1),
                condition
            )),
            |(register, instruction, condition)| Rule { register, instruction, condition }
        )(input)
    }

    separated_list1(newline, rule)(input)
}

fn part1(input: &str) -> isize {
    let rules = parse_input(input).unwrap().1;
    let mut registers = HashMap::new();

    for Rule { register, instruction, condition } in rules {
        let Condition { register: r, comp: c, target: t } = condition;
        let cond_value = *registers.entry(r).or_insert(0);
        let test = match c {
            Le => cond_value <= t,
            Lt => cond_value < t,
            Eq => cond_value == t,
            Ne => cond_value != t,
            Gt => cond_value > t,
            Ge => cond_value >= t
        };

        if test {
            let target = registers.entry(register).or_insert(0);
            match instruction {
                Inc(i) => *target += i,
                Dec(i) => *target -= i
            }
        }
    }

    registers.into_values().max().unwrap()
}

fn part2(input: &str) -> isize {
    let rules = parse_input(input).unwrap().1;
    let mut registers = HashMap::new();

    let mut best = isize::MIN;

    for Rule { register, instruction, condition } in rules {
        let Condition { register: r, comp: c, target: t } = condition;
        let cond_value = *registers.entry(r).or_insert(0);
        let test = match c {
            Le => cond_value <= t,
            Lt => cond_value < t,
            Eq => cond_value == t,
            Ne => cond_value != t,
            Gt => cond_value > t,
            Ge => cond_value >= t
        };

        if test {
            let target = registers.entry(register).or_insert(0);
            match instruction {
                Inc(i) => *target += i,
                Dec(i) => *target -= i
            }

            best = max(best, *target);
        }
    }

    best
}

build_main!("day08.txt", "Part 1" => part1, "Part 2" => part2);