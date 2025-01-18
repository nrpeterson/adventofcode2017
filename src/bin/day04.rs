use std::collections::HashSet;
use itertools::Itertools;
use adventofcode2017::build_main;

fn no_repeats(passphrase: &[&str]) -> bool {
    let mut seen = HashSet::new();
    for &word in passphrase {
        if !seen.insert(word) {
            return false;
        }
    }

    true
}

fn no_anagrams(passphrase: &[&str]) -> bool {
    let mut seen = HashSet::new();

    for &word in passphrase {
        let mut sig = [0; 26];
        for c in word.chars() {
            let ord = c as usize - 'a' as usize;
            sig[ord] += 1;
        }

        if !seen.insert(sig) {
            return false;
        }
    }

    true
}

fn part1(input: &str) -> usize {
    input.lines()
        .map(|line| line.split(' ').collect_vec())
        .filter(|passphrase| no_repeats(passphrase))
        .count()
}

fn part2(input: &str) -> usize {
    input.lines()
        .map(|line| line.split(' ').collect_vec())
        .filter(|passphrase| no_anagrams(passphrase))
        .count()
}

build_main!("day04.txt", "Part 1" => part1, "Part 2" => part2);