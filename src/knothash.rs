use std::ops::{Index, IndexMut};
use itertools::Itertools;

pub struct Circle {
    pub values: Vec<u8>
}

impl Index<usize> for Circle {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        let i = index % self.values.len();
        &self.values[i]
    }
}

impl IndexMut<usize> for Circle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let i = index % self.values.len();
        &mut self.values[i]
    }
}

pub fn reverse(circle: &mut Circle, from: usize, number: usize) {
    let values = (from..from+number)
        .rev()
        .map(|i| circle[i])
        .collect_vec();

    (from..from+number).zip(values.into_iter()).for_each(|(i, v)| circle[i] = v);
}



pub fn knot_hash(input: &str) -> [u8; 16] {
    let mut lengths = input.chars().map(|c| c as usize).collect_vec();
    lengths.extend([17, 31, 73, 47, 23]);

    let mut circle = Circle { values: (0..=255).collect_vec() };
    let mut cur_position = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        for &length in lengths.iter() {
            if length > circle.values.len() {
                continue;
            }

            reverse(&mut circle, cur_position, length);
            cur_position += length + skip_size;
            skip_size += 1;
        }
    }

    (0..256).step_by(16)
        .map(|i| (i..i+16).map(|j| circle[j]).reduce(|a, b| a ^ b).unwrap())
        .collect_vec()
        .try_into()
        .unwrap()
}