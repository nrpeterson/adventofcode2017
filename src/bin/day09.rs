use adventofcode2017::build_main;
use itertools::Itertools;

fn evaluate(input: &str) -> (usize, usize) {
    let mut in_garbage = false;
    let mut score = 0;
    let mut garbage_count = 0;
    let mut depth = 0;

    let input = input.chars().collect_vec();
    let mut i = 0;

    while i < input.len() {
        match (input[i], in_garbage) {
            ('{', false) => {
                depth += 1;
                score += depth;
            },
            ('}', false) => { depth -= 1; },
            ('<', false) => { in_garbage = true; },
            ('>', true) => { in_garbage = false; },
            ('!', true) => { i += 1; },
            (_, true) => { garbage_count += 1; },
            _ => ()
        };
        i += 1;
    }

    (score, garbage_count)
}

fn part1(input: &str) -> usize {
    evaluate(input).0
}

fn part2(input: &str) -> usize {
    evaluate(input).1
}

build_main!("day09.txt", "Part 1" => part1, "Part 2" => part2);