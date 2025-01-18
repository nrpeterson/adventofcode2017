use std::collections::HashSet;
use adventofcode2017::build_main;
use adventofcode2017::knothash::knot_hash;

fn part1(input: &str) -> usize {
    (0..128).map(|i| format!("{input}-{i}"))
        .flat_map(|key| knot_hash(&key))
        .map(|b| b.count_ones() as usize)
        .sum()
}

fn neighbors((i, j): (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = vec![];

    if i > 0 {
        result.push((i-1, j));
    }
    if i < 127 {
        result.push((i+1, j));
    }
    if j > 0 {
        result.push((i, j-1));
    }
    if j < 127 {
        result.push((i, j+1));
    }

    result
}

fn part2(input: &str) -> usize {
    let used: HashSet<(usize, usize)> = (0..128).map(|i| (i, format!("{input}-{i}")))
        .flat_map(|(i, seed)| {
            let hash = knot_hash(&seed);
            (0..128).map(move |j| {
                let index = j / 8;
                let bit = j % 8;
                let mask = 1 << (7 - bit);
                ((i, j), hash[index] & mask != 0)
            })
        })
        .filter(|(_, is_set)| *is_set)
        .map(|(pos, _)| pos)
        .collect();

    let mut seen = HashSet::new();
    let mut component_count = 0;

    for &(i, j) in used.iter() {
        if seen.contains(&(i, j)) { continue; }

        component_count += 1;

        seen.insert((i, j));
        let mut component = HashSet::new();
        component.insert((i, j));
        let mut stack = Vec::new();
        stack.push((i, j));

        while let Some((u, v)) = stack.pop() {
            for neighbor in neighbors((u, v)) {
                if used.contains(&neighbor) && component.insert(neighbor) {
                    seen.insert(neighbor);
                    stack.push(neighbor);
                }
            }
        }
    }

    component_count
}

build_main!("day14.txt", "Part 1" => part1, "Part 2" => part2);