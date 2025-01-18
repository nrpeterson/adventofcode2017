use itertools::Itertools;
use adventofcode2017::build_main;
use adventofcode2017::knothash::{knot_hash, reverse, Circle};

fn part1(input: &str) -> usize {
    let mut circle = Circle { values: (0..=255).collect_vec() };
    let mut cur_position = 0;
    let mut skip_size = 0;
    let lengths = input.split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    for length in lengths {
        if length > circle.values.len() {
            continue;
        }

        reverse(&mut circle, cur_position, length);
        cur_position += length + skip_size;
        skip_size += 1;
    }

    (circle[0] as usize) * (circle[1] as usize)
}

fn part2(input: &str) -> String {
    knot_hash(input).into_iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

build_main!("day10.txt", "Part 1" => part1, "Part 2" => part2);