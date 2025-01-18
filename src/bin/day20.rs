use adventofcode2017::build_main;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, newline, space0};
use nom::combinator::{map, map_res, opt, recognize};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign, Mul, Sub};

fn isqrt(n: isize) -> Option<isize> {
    if n < 0 {
        None
    }
    else {
        let mut low = 0;
        let mut high = n;

        while low < high {
            let mid = low + (high - low) / 2;

            let sq = mid * mid;

            if sq <= n && sq + 2*mid + 1 > n {
                low = mid;
                high = mid;
            }
            else if mid * mid < n {
                low = mid + 1;
            }
            else {
                high = mid - 1;
            }
        }

        if low * low == n { Some(low) } else { None }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Solutions {
    Finite(Vec<isize>),
    Any
}

impl Solutions {
    fn intersection(&self, other: &Solutions) -> Solutions {
        match (&self, other) {
            (&Solutions::Any, x) => x.clone(),
            (&x, &Solutions::Any) => x.clone(),
            (Solutions::Finite(v), Solutions::Finite(w)) => {
                let results = v.iter().cloned()
                    .filter(|&i| w.contains(&i))
                    .collect_vec();

                Solutions::Finite(results)
            }
        }
    }
}

fn nonneg_int_quadratic_sols(a: isize, b: isize, c: isize) -> Solutions {
    if a == 0 && b == 0 {
        match c {
            0 => Solutions::Any,
            _ => Solutions::Finite(vec![])
        }
    }
    else if a == 0 {
        // Solution should be bt+c = 0 => t = -c/b.
        if c % b == 0 && -c / b >= 0 {
            Solutions::Finite(vec![-c / b])
        }
        else {
            Solutions::Finite(vec![])
        }
    }
    else {
        let disc = b*b - 4*a*c;
        match isqrt(disc) {
            None => Solutions::Finite(vec![]),
            Some(disc_sqrt) => {
                let opts = [-b + disc_sqrt, -b - disc_sqrt].into_iter()
                    .filter(|&num| num % (2 * a) == 0)
                    .map(|num| num / (2 * a))
                    .filter(|&num| num >= 0)
                    .collect_vec();

                Solutions::Finite(opts)
            }
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Triple(isize, isize, isize);

impl Triple {
    fn norm(&self) -> isize {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

impl Add for Triple {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Triple {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl Sub for Triple {
    type Output = Triple;

    fn sub(self, rhs: Self) -> Self::Output {
        Triple(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<isize> for Triple {
    type Output = Triple;
    fn mul(self, rhs: isize) -> Self::Output {
        Triple(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

#[derive(Copy, Clone)]
struct Particle { p: Triple, v: Triple, a: Triple }

impl Sub for Particle {
    type Output = Particle;

    fn sub(self, rhs: Self) -> Self::Output {
        Particle { p: self.p - rhs.p, v: self.v - rhs.v, a: self.a - rhs.a }
    }
}

impl Particle {
    fn origin_hits(&self) -> Solutions {
        let Triple(p_x, p_y, p_z) = self.p;
        let Triple(v_x, v_y, v_z) = self.v;
        let Triple(a_x, a_y, a_z) = self.a;

        let x_sols = nonneg_int_quadratic_sols(a_x, 2*v_x + a_x, 2*p_x);
        let y_sols = nonneg_int_quadratic_sols(a_y, 2*v_y + a_y, 2*p_y);
        let z_sols = nonneg_int_quadratic_sols(a_z, 2*v_z + a_z, 2*p_z);

        x_sols.intersection(&y_sols).intersection(&z_sols)
    }

    fn first_origin_hit(&self) -> Option<isize> {
        match self.origin_hits() {
            Solutions::Any => Some(0),
            Solutions::Finite(w) => w.into_iter().min()
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Particle>> {
    fn number(input: &str) -> IResult<&str, isize> {
        map_res(
            recognize(pair(opt(char('-')), digit1)),
            |s: &str| s.parse::<isize>()
        )(input)
    }

    fn triple(input: &str) -> IResult<&str, Triple> {
        map(
            delimited(
                terminated(char('<'), space0),
                separated_list1(char(','), number),
                char('>')
            ),
            |v| Triple(v[0], v[1], v[2])
        )(input)
    }

    fn particle(input: &str) -> IResult<&str, Particle> {
        map(
            tuple((
                preceded(tag("p="), triple),
                preceded(tag(", v="), triple),
                preceded(tag(", a="), triple)
            )),
            |(p, v, a)| Particle { p, v, a }
        )(input)
    }

    separated_list1(newline, particle)(input)
}

fn part1(input: &str) -> usize {
    let particles = parse_input(input).unwrap().1;

    particles.into_iter()
        .position_min_by_key(|p| {
            (p.a.norm(), p.v.norm(), p.p.norm())
        })
        .unwrap()
}

fn part2(input: &str) -> usize {
    let particles = parse_input(input).unwrap().1;
    let mut collisions: HashMap<isize, Vec<(usize, usize)>> = HashMap::new();
    let num_particles = particles.len();
    let mut removed = HashSet::new();

    for ((i0, p0), (i1, p1)) in particles.into_iter().enumerate().tuple_combinations() {
        let delta = p0 - p1;
        if let Some(t) = delta.first_origin_hit() {
            collisions.entry(t).or_default().push((i0, i1));
        }
    }

    collisions.into_iter().sorted()
        .for_each(|(_, v)| {
            let to_remove = v.into_iter()
                .filter(|&(i, j)| !removed.contains(&i) && !removed.contains(&j))
                .flat_map(|(i, j)| [i, j])
                .collect_vec();

            to_remove.into_iter()
                .for_each(|i| { removed.insert(i); });
        });

    num_particles - removed.len()
}

build_main!("day20.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use crate::Solutions::{Any, Finite};
    use super::*;

    #[test]
    fn test_roots() {
        assert_eq!(nonneg_int_quadratic_sols(0, 0, 0), Any);

        // (x-5)(x+7) = x^2+2x-35
        assert_eq!(nonneg_int_quadratic_sols(1, 2, -35), Finite(vec![5]));

        // (2x-1)(x-5) = 2x^2-11x+5
        assert_eq!(nonneg_int_quadratic_sols(2, -11, 5), Finite(vec![5]));

        // (x-1)(x-2) = x^2-3x+2
        assert!(match nonneg_int_quadratic_sols(1, -3, 2) {
            Any => false,
            Finite(v) => v.len() == 2 && v.contains(&1) && v.contains(&2)
        } );

        // (2x-1)(2x-5) = 4x^2-12x+5
        assert_eq!(nonneg_int_quadratic_sols(4, -12, 5), Finite(vec![]));
    }
}