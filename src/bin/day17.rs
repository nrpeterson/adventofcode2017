use adventofcode2017::build_main;

fn part1(input: &str) -> usize {
    let steps = input.parse::<usize>().unwrap();
    let mut buffer = vec![0];
    let mut cur = 0;
    let mut last= 0;

    for i in 1..=2017 {
        cur = (cur + steps) % buffer.len() + 1;
        buffer.insert(cur , i);
        last = cur;
    }

    buffer[last + 1]
}

fn part2(input: &str) -> usize {
    let steps = input.parse::<usize>().unwrap();
    let mut buffer_len = 1;
    let mut cur = 0;
    let mut succ = 0;

    for i in 1..=50000000 {
        cur = (cur + steps) % buffer_len + 1;
        if cur == 1 {
            succ = i;
        }
        buffer_len += 1;

    }

    succ
}

build_main!("day17.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        println!("{}", part1("3"));
    }
}