use adventofcode2017::build_main;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, newline};
use nom::combinator::{map, map_res, opt};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use std::collections::HashMap;

#[derive(Clone)]
struct Program<'a> {
    name: &'a str,
    weight: usize,
    holding: Vec<&'a str>
}

fn parse_input(input: &str) -> IResult<&str, Vec<Program>> {
    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    fn program(input: &str) -> IResult<&str, Program> {
        map(
            tuple((
                alpha1,
                delimited(tag(" ("), number, tag(")")),
                opt(
                    preceded(
                        tag(" -> "),
                        separated_list1(tag(", "), alpha1)
                    )
                )
            )),
            |(name, weight, supp)| {
                let holding = supp.unwrap_or_default();
                Program { name, weight, holding }
            }
        )(input)
    }

    separated_list1(newline, program)(input)
}

fn find_root<'a>(programs: &[Program<'a>]) -> &'a str {
    let mut pred: HashMap<&str, Option<&str>> = HashMap::new();

    for Program { name, holding, ..} in programs.iter() {
        pred.entry(*name).or_default();

        for &succ in holding.iter() {
            pred.insert(succ, Some(name));
        }
    }

    pred.into_iter()
        .filter(|&(_, v)| v.is_none())
        .nth(0)
        .unwrap()
        .0
}

fn part1(input: &str) -> &str {
    let programs = parse_input(input).unwrap().1;
    find_root(&programs)
}

fn part2(input: &str) -> usize {
    let programs = parse_input(input).unwrap().1;

    let root = find_root(&programs);

    let map: HashMap<&str, Program> = programs.into_iter()
        .map(|p| (p.name, p))
        .collect();

    let mut weights = HashMap::new();
    let mut unbalanced_children = HashMap::new();

    let mut stack = vec![root];

    while let Some(&name) = stack.last() {
        let mut unsat_children = map[&name].holding.iter()
            .filter(|&child| !weights.contains_key(child))
            .cloned()
            .collect_vec();

        if unsat_children.is_empty() {
            let child_weights: Vec<(&str, usize)> = map[&name].holding.iter()
                .map(|name| (*name, weights[name]))
                .collect_vec();

            let weight = map[&name].weight + child_weights.iter().map(|(_, w)| *w).sum::<usize>();
            weights.insert(name, weight);

            let unique_weight = child_weights.iter()
                .map(|(_, w)| *w)
                .counts()
                .into_iter()
                .find(|(_, v)| *v == 1)
                .map(|(k, _)| k);

            let unbalanced_child = unique_weight.map(|w| {
                child_weights.iter()
                    .find(|(_, v)| *v == w)
                    .map(|(k, _)| *k)
                    .unwrap()
            });

            unbalanced_child.iter().for_each(|&k| { unbalanced_children.insert(name, k); });

            stack.pop();
        }
        else {
            stack.append(&mut unsat_children);
        }
    }

    let mut cur = root;
    loop {
        let kid = unbalanced_children[&cur];
        if unbalanced_children.contains_key(&kid) {
            cur = kid;
        }
        else {
            break;
        }
    }

    let culprit = unbalanced_children[&cur];
    let culprit_total_weight = weights[&culprit];

    let target_total_weight = map[&cur].holding.iter()
        .map(|name| weights[name])
        .find(|w| *w != culprit_total_weight)
        .unwrap();

    let delta = (target_total_weight as isize) - (culprit_total_weight as isize);
    (map[&culprit].weight as isize + delta) as usize
}

build_main!("day07.txt", "Part 1" => part1, "Part 2" => part2);