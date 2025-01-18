use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, digit1, newline, space1};
use nom::combinator::{all_consuming, map, map_res, opt, recognize};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded, separated_pair};
use adventofcode2017::build_main;
use crate::Instruction::{Jnz, Mul, Set, Sub};
use crate::Operand::{Literal, Register};

#[derive(Copy, Clone)]
enum Operand {
    Literal(isize),
    Register(usize)
}

#[derive(Copy, Clone)]
enum Instruction {
    Set { tgt: usize, src: Operand },
    Sub { tgt: usize, src: Operand },
    Mul { tgt: usize, src: Operand },
    Jnz { test: Operand, offset: Operand }
}

struct Machine {
    registers: [isize; 8],
    instructions: Vec<Instruction>,
    cur_ptr: isize
}

impl Machine {
    fn eval_operand(&self, operand: Operand) -> isize {
        match operand {
            Operand::Literal(v) => v,
            Operand::Register(r) => self.registers[r]
        }
    }
}

impl Iterator for Machine {
    type Item = (Instruction, [isize; 8]);
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_ptr < 0 || self.cur_ptr >= self.instructions.len() as isize {
            return None;
        }

        let instr = self.instructions[self.cur_ptr as usize];
        let mut advance = 1;

        match instr {
            Set { tgt, src } => {
                self.registers[tgt] = self.eval_operand(src);
            }
            Sub { tgt, src } => {
                self.registers[tgt] -= self.eval_operand(src);
            }
            Mul { tgt, src } => {
                self.registers[tgt] *= self.eval_operand(src);
            }
            Jnz { test, offset } => {
                let test = self.eval_operand(test);
                if test != 0 {
                    advance = self.eval_operand(offset);
                }
            }
        }
        self.cur_ptr += advance;
        Some((instr, self.registers))
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    fn register(input: &str) -> IResult<&str, usize> {
        map(anychar, |c| c as usize - 'a' as usize)(input)
    }

    fn number(input: &str) -> IResult<&str, isize> {
        map_res(
            recognize(pair(opt(char('-')), digit1)),
            |s: &str| s.parse::<isize>()
        )(input)
    }

    fn operand(input: &str) -> IResult<&str, Operand> {
        alt((
            map(number, Literal),
            map(register, Register)
        ))(input)
    }

    fn instruction(input: &str) -> IResult<&str, Instruction> {
        alt((
            map(
                preceded(tag("set "), separated_pair(register, space1, operand)),
                |(tgt, src)| Set { tgt, src },
            ),
            map(
                preceded(tag("sub "), separated_pair(register, space1, operand)),
                |(tgt, src)| Sub { tgt, src },
            ),
            map(
                preceded(tag("mul "), separated_pair(register, space1, operand)),
                |(tgt, src)| Mul { tgt, src },
            ),
            map(
                preceded(tag("jnz "), separated_pair(operand, space1, operand)),
                |(test, offset)| Jnz { test, offset }
            )
        ))(input)
    }

    all_consuming(separated_list1(newline, instruction))(input)
}

fn part1(input: &str) -> usize {
    let instructions = parse_input(input).unwrap().1;
    let machine = Machine { registers: [0; 8], instructions, cur_ptr: 0};
    let mut mults = 0;

    for (instr, _) in machine {
        match instr {
            Mul { .. } => mults += 1,
            _ => ()
        }
    }

    mults
}

/*
0.  set b 84
1.  set c b
4.  mul b 100      // b = 100*b + 100000 => b = 108400
5.  sub b -100000
6.  set c b
7.  sub c -17000   // c = b + 17000 => c = 125400

    label2:
8.  set f 1        // f = 1, d = 2
9.  set d 2

    label4:
10. set e 2         // e = 2

    label5:
11. set g d
12. mul g e
13. sub g b    // g = d * e - b
14. jnz g 2 -> label3
15. set f 0

    label3:
16. sub e -1   // e += 1;
17. set g e
18. sub g b    // g = e - b;
19. jnz g -8 -> label5
20. sub d -1
21. set g d
22. sub g b
23. jnz g -13 -> label4
24. jnz f 2 -> label6
25. sub h -1

    label6:
26. set g b
27. sub g c
28. jnz g 2 -> label7
29. jnz 1 3 -> done

    label7:
30. sub b -17
31. jnz 1 -23 -> label2
 */

fn is_composite(n: usize) -> bool {
    if n % 2 == 0 {
        return true;
    }

    let max = (n as f64).sqrt().ceil() as usize;

    (3..=max).step_by(2).any(|i| n % i == 0)
}

fn part2(_: &str) -> usize {
    let mut b = 84*100 + 100000;
    let c = b + 17000;
    (b..=c).step_by(17)
        .filter(|&n| is_composite(n))
        .count()
}

build_main!("day23.txt", "Part 1" => part1, "Part 2" => part2);