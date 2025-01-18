use std::collections::{HashMap, VecDeque};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char as ch, digit1, newline, space1};
use nom::combinator::{map, map_res, opt, recognize};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded, separated_pair};
use adventofcode2017::build_main;
use crate::Instruction::*;
use crate::Operand::*;
use crate::Step::*;

#[derive(Copy, Clone)]
enum Operand {
    Literal(isize),
    Register(char)
}

#[derive(Copy, Clone)]
enum Instruction {
    Snd { src: Operand },
    Set { tgt: char, src: Operand },
    Add { tgt: char, src: Operand },
    Mul { tgt: char, src: Operand },
    Mod { tgt: char, src: Operand },
    Rcv { tgt: char },
    Jgz { test: Operand, offset: Operand }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    fn number(input: &str) -> IResult<&str, isize> {
        map_res(
            recognize(pair(opt(ch('-')), digit1)),
            |s: &str| s.parse::<isize>()
        )(input)
    }

    fn operand(input: &str) -> IResult<&str, Operand> {
        alt((
            map(number, Literal),
            map(anychar, Register)
        ))(input)
    }

    fn instruction(input: &str) -> IResult<&str, Instruction> {
        alt((
            map(preceded(tag("snd "), operand), |src| Snd { src }),
            map(
                preceded(tag("set "), separated_pair(anychar, space1, operand)),
                |(tgt, src)| Set { tgt, src }
            ),
            map(
                preceded(tag("add "), separated_pair(anychar, space1, operand)),
                |(tgt, src)| Add { tgt, src }
            ),
            map(
                preceded(tag("mul "), separated_pair(anychar, space1, operand)),
                |(tgt, src)| Mul { tgt, src }
            ),
            map(
                preceded(tag("mod "), separated_pair(anychar, space1, operand)),
                |(tgt, src)| Mod { tgt, src }
            ),
            map(preceded(tag("rcv "), anychar), |tgt| Rcv { tgt }),
            map(
                preceded(tag("jgz "), separated_pair(operand, space1, operand)),
                |(test, offset)| Jgz { test, offset }
            )
        ))(input)
    }

    separated_list1(newline, instruction)(input)
}

struct Machine {
    registers: HashMap<char, isize>,
    instructions: Vec<Instruction>,
    cur_ptr: isize,
    input_buffer: VecDeque<isize>
}

#[derive(Eq, PartialEq)]
enum Step {
    Sent(isize),
    Waiting(char),
    Received(isize),
    Done,
    Continue
}

impl Machine {
    fn new(instructions: Vec<Instruction>, program_id: isize) -> Machine {
        let mut registers = HashMap::new();
        registers.insert('p', program_id);
        Machine { registers, instructions, cur_ptr: 0, input_buffer: VecDeque::new() }
    }

    fn eval(&mut self, operand: Operand) -> isize {
        match operand {
            Literal(x) => x,
            Register(c) => *self.registers.entry(c).or_insert(0)
        }
    }

    fn step(&mut self) -> Step {
        if self.cur_ptr < 0 || self.cur_ptr >= self.instructions.len() as isize {
            return Done
        }

        let mut advance = 1;
        let instr = self.instructions[self.cur_ptr as usize];
        let result = match instr {
            Snd { src } => {
                Sent(self.eval(src))
            },
            Set { tgt, src } => {
                let val = self.eval(src);
                self.registers.insert(tgt, val);
                Continue
            },
            Add { tgt, src } => {
                let val = self.eval(src);
                *self.registers.entry(tgt).or_insert(0) += val;
                Continue
            },
            Mul { tgt, src } => {
                let val = self.eval(src);
                *self.registers.entry(tgt).or_insert(0) *= val;
                Continue
            },
            Mod { tgt, src } => {
                let val = self.eval(src);
                *self.registers.entry(tgt).or_insert(0) %= val;
                Continue
            },
            Rcv { tgt } => {
                if let Some(v) = self.input_buffer.pop_front() {
                    self.registers.insert(tgt, v);
                    Received(v)
                }
                else {
                    advance = 0;
                    Waiting(tgt)
                }
            },
            Jgz { test, offset } => {
                let test_val = self.eval(test);
                if test_val > 0 {
                    advance = self.eval(offset);
                }
                Continue
            }
        };

        self.cur_ptr += advance;
        result
    }
}

fn part1(input: &str) -> isize {
    let instructions = parse_input(input).unwrap().1;
    let mut machine = Machine::new(instructions, 0);
    let mut last_value = None;

    loop {
        match machine.step() {
            Done => panic!("Didn't ever enter waiting"),
            Sent(value) => { last_value = Some(value); },
            Waiting(reg) => {
                if *machine.registers.entry(reg).or_insert(0) != 0 {
                    return last_value.unwrap()
                }
            }
            _ => continue
        }
    }
}

fn part2(input: &str) -> usize {
    let instructions = parse_input(input).unwrap().1;
    let mut p1_sent_count = 0;

    let mut m0 = Machine::new(instructions.clone(), 0);
    let mut m1 = Machine::new(instructions, 1);

    loop {
        let m0_step = m0.step();
        let m1_step = m1.step();

        if m0_step == Done && m1_step == Done {
            break;
        }

        if let Waiting(_) = m0_step {
            if let Waiting(_) = m1_step {
                break;
            }
        }

        if let Sent(x) = m0_step {
            m1.input_buffer.push_back(x);
        }

        if let Sent(x) = m1_step {
            m0.input_buffer.push_back(x);
            p1_sent_count += 1;
        }
    }

    p1_sent_count
}

build_main!("day18.txt", "Part 1" => part1, "Part 2" => part2);