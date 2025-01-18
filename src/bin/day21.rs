/*
    Rotations / flips: if r=rotate right, f = horizontal flip:
    1   r   r^2  r^3  f   rf  r^2f  r^3f
    ab  ca  dc   bd   ba  cb  dc    ad
    cd  db  ba   ac   cd  da  ab    bc

    For 3x3, r and f are:

    1    r    f
    abc  gda  cba
    def  heb  fed
    ghi  ifc  ihg
 */
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, newline};
use nom::combinator::{all_consuming, map, value};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use adventofcode2017::build_main;

fn flip(data: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    (0..data.len()).map(|i| {
        (0..data.len()).map(|j| data[i][data.len() - 1 - j]).collect_vec()
    }).collect_vec()
}

fn rotate(data: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    (0..data.len()).map(|i| {
        (0..data.len()).map(|j| data[j][data.len() - 1- i]).collect_vec()
    }).collect_vec()
}

fn all_symmetries(data: &Vec<Vec<bool>>) -> Vec<Vec<Vec<bool>>> {
    let mut result = Vec::new();

    let mut cur = data.clone();

    for _ in 0..4 {
        result.push(cur.clone());
        result.push(flip(&cur));
        cur = rotate(&cur);
    }

    result
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Grid(Vec<Vec<bool>>);

impl Grid {
    // fn print(&self) {
    //     self.0.iter().for_each(|row| {
    //         let s: String = row.iter().map(|&b| if b { '#' } else { '.' }).collect();
    //         println!("{s}");
    //     });
    // }

    fn copy_from(&mut self, other: &Grid, start_i: usize, start_j: usize) {
        (0..other.0.len()).cartesian_product(0..other.0.len()).for_each(|(i, j)| {
            self[(start_i + i, start_j + j)] = other[(i, j)]
        })
    }

    fn subgrid(&self, start_i: usize, start_j: usize, size: usize) -> Grid {
        let result = (0..size).map(|i| {
            (0..size).map(|j| self[(start_i + i, start_j + j)]).collect_vec()
        }).collect_vec();

        Grid(result)
    }

    fn apply_rules(&self, rules: &HashMap<Grid, Grid>) -> Grid {
        let chunk_size = if self.0.len() % 2 == 0 { 2 } else { 3 };
        let new_chunk_size = if chunk_size == 2 { 3 } else { 4 };
        let new_size =
            if chunk_size == 2 { (self.0.len() / 2) * 3 }
            else { (self.0.len() / 3) * 4 };

        let mut result = Grid(vec![vec![false; new_size]; new_size]);

        for i in 0..self.0.len() / chunk_size {
            for j in 0..self.0.len() / chunk_size {
                let subgrid = self.subgrid(i * chunk_size, j * chunk_size, chunk_size);
                let new_subgrid = rules.get(&subgrid).unwrap();
                result.copy_from(&new_subgrid, i * new_chunk_size, j * new_chunk_size);
            }
        }

        result
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = bool;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.0[i][j]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.0[i][j]
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Grid, Grid)>> {
    fn pixel(input: &str) -> IResult<&str, bool> {
        alt((
            value(true, char('#')),
            value(false, char('.'))
        ))(input)
    }

    fn grid(input: &str) -> IResult<&str, Grid> {
        map(
            separated_list1(char('/'), many1(pixel)),
            |data| Grid(data)
        )(input)
    }

    fn line(input: &str) -> IResult<&str, (Grid, Grid)> {
        separated_pair(grid, tag(" => "), grid)(input)
    }

    all_consuming(separated_list1(newline, line))(input)
}

fn solve(input: &str, iterations: usize) -> usize {
    let rules: HashMap<Grid, Grid> = parse_input(input).unwrap().1.into_iter()
        .flat_map(|(input, ref output)| {
            all_symmetries(&input.0).into_iter()
                .map(move |sym| (Grid(sym), output.clone()))
                .collect_vec()
        })
        .collect();

    let mut grid = Grid(vec![vec![false, true, false], vec![false, false, true], vec![true, true, true]]);

    for _ in 0..iterations {
        grid = grid.apply_rules(&rules);
    }

    grid.0.into_iter().flatten().filter(|&b| b).count()
}

fn part1(input: &str) -> usize {
    solve(input, 5)
}

fn part2(input: &str) -> usize {
    solve(input, 18)
}

build_main!("day21.txt", "Part 1" => part1, "Part 2" => part2);