use std::collections::HashMap;
use itertools::Itertools;
use adventofcode2017::build_main;

fn find_loop(mut cur: Vec<usize>) -> (usize, usize) {
    let mut seen = HashMap::new();
    let mut loop_count = 0;

    seen.insert(cur.clone(), loop_count);

    loop {
        let (max_pos, max_val) = cur.iter().enumerate()
            .fold((0, 0), |(max_pos, max_val), (i, &next)| {
                if next > max_val {
                    (i, next)
                }
                else {
                    (max_pos, max_val)
                }
            });

        cur[max_pos] = 0;
        (0..max_val).for_each(|delta| {
            let j = (max_pos + 1 + delta) % cur.len();
            cur[j] += 1;
        });

        loop_count += 1;

        let orig = seen.insert(cur.clone(), loop_count);
        match orig {
            None => continue,
            Some(orig) => return (seen.len(), loop_count - orig)
        }
    }
}

fn part1(input: &str) -> usize {
    let cur = input.split('\t')
        .map(|w| w.parse::<usize>().unwrap())
        .collect_vec();

    find_loop(cur).0
}

fn part2(input: &str) -> usize {
    let cur = input.split('\t')
        .map(|w| w.parse::<usize>().unwrap())
        .collect_vec();

    find_loop(cur).1
}

build_main!("day06.txt", "Part 1" => part1, "Part 2" => part2);