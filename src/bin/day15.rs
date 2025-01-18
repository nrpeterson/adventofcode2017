use itertools::Itertools;
use adventofcode2017::build_main;

struct Generator {
    cur: u128,
    factor: u128
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur *= self.factor;
        self.cur %= 2147483647;

        Some((self.cur & 0xffff) as usize)
    }
}

fn part1(input: &str) -> usize {
    let starts = input.lines()
        .map(|line| {
            line.split(' ')
                .last().unwrap()
                .parse::<u128>()
                .unwrap()
        })
        .collect_vec();

    let gen_a = Generator { cur: starts[0], factor: 16807 };
    let gen_b = Generator { cur: starts[1], factor: 48271 };

    gen_a.zip(gen_b)
        .take(40000000)
        .filter(|&(a, b)| a == b)
        .count()
}

fn part2(input: &str) -> usize {
    let starts = input.lines()
        .map(|line| {
            line.split(' ')
                .last().unwrap()
                .parse::<u128>()
                .unwrap()
        })
        .collect_vec();

    let gen_a = Generator { cur: starts[0], factor: 16807 }
        .filter(|&x| x % 4 == 0);
    let gen_b = Generator { cur: starts[1], factor: 48271 }
        .filter(|&x| x % 8 == 0);

    gen_a.zip(gen_b)
        .take(5000000)
        .filter(|&(a, b)| a == b)
        .count()
}

build_main!("day15.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator() {
        let gen_a = Generator { cur: 65, factor: 16807 };
        let gen_b = Generator { cur: 8921, factor: 48271 };

        gen_a.zip(gen_b).take(5).for_each(|(x, y)| println!("{x:10} {y:10}"));
    }
}