#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

use std::collections::{HashMap, HashSet};

use crate::day_solver::DaySolver;

pub struct Day17;

type N = i8;

const ADJUSTS3: [(N, N, N); 26] = [
    (-1, -1, -1),
    (-1, -1, 0),
    (-1, -1, 1),
    (-1, 0, -1),
    (-1, 0, 0),
    (-1, 0, 1),
    (-1, 1, -1),
    (-1, 1, 0),
    (-1, 1, 1),
    (0, -1, -1),
    (0, -1, 0),
    (0, -1, 1),
    (0, 0, -1),
    (0, 0, 1),
    (0, 1, -1),
    (0, 1, 0),
    (0, 1, 1),
    (1, -1, -1),
    (1, -1, 0),
    (1, -1, 1),
    (1, 0, -1),
    (1, 0, 0),
    (1, 0, 1),
    (1, 1, -1),
    (1, 1, 0),
    (1, 1, 1),
];

const ADJUSTS4: [(N, N, N, N); 80] = [
    (-1, -1, -1, -1),
    (-1, -1, -1, 0),
    (-1, -1, -1, 1),
    (-1, -1, 0, -1),
    (-1, -1, 0, 0),
    (-1, -1, 0, 1),
    (-1, -1, 1, -1),
    (-1, -1, 1, 0),
    (-1, -1, 1, 1),
    (-1, 0, -1, -1),
    (-1, 0, -1, 0),
    (-1, 0, -1, 1),
    (-1, 0, 0, -1),
    (-1, 0, 0, 0),
    (-1, 0, 0, 1),
    (-1, 0, 1, -1),
    (-1, 0, 1, 0),
    (-1, 0, 1, 1),
    (-1, 1, -1, -1),
    (-1, 1, -1, 0),
    (-1, 1, -1, 1),
    (-1, 1, 0, -1),
    (-1, 1, 0, 0),
    (-1, 1, 0, 1),
    (-1, 1, 1, -1),
    (-1, 1, 1, 0),
    (-1, 1, 1, 1),
    (0, -1, -1, -1),
    (0, -1, -1, 0),
    (0, -1, -1, 1),
    (0, -1, 0, -1),
    (0, -1, 0, 0),
    (0, -1, 0, 1),
    (0, -1, 1, -1),
    (0, -1, 1, 0),
    (0, -1, 1, 1),
    (0, 0, -1, -1),
    (0, 0, -1, 0),
    (0, 0, -1, 1),
    (0, 0, 0, -1),
    (0, 0, 0, 1),
    (0, 0, 1, -1),
    (0, 0, 1, 0),
    (0, 0, 1, 1),
    (0, 1, -1, -1),
    (0, 1, -1, 0),
    (0, 1, -1, 1),
    (0, 1, 0, -1),
    (0, 1, 0, 0),
    (0, 1, 0, 1),
    (0, 1, 1, -1),
    (0, 1, 1, 0),
    (0, 1, 1, 1),
    (1, -1, -1, -1),
    (1, -1, -1, 0),
    (1, -1, -1, 1),
    (1, -1, 0, -1),
    (1, -1, 0, 0),
    (1, -1, 0, 1),
    (1, -1, 1, -1),
    (1, -1, 1, 0),
    (1, -1, 1, 1),
    (1, 0, -1, -1),
    (1, 0, -1, 0),
    (1, 0, -1, 1),
    (1, 0, 0, -1),
    (1, 0, 0, 0),
    (1, 0, 0, 1),
    (1, 0, 1, -1),
    (1, 0, 1, 0),
    (1, 0, 1, 1),
    (1, 1, -1, -1),
    (1, 1, -1, 0),
    (1, 1, -1, 1),
    (1, 1, 0, -1),
    (1, 1, 0, 0),
    (1, 1, 0, 1),
    (1, 1, 1, -1),
    (1, 1, 1, 0),
    (1, 1, 1, 1),
];

impl DaySolver<'_> for Day17 {
    type Parsed = HashSet<(N, N, N)>;
    type Output = usize;

    fn parse(input: &str) -> Self::Parsed {
        input
            .as_bytes()
            .split(|&x| x == b'\n')
            .enumerate()
            .flat_map(|(y, l)| {
                l.iter().enumerate().filter_map(move |(x, &c)| {
                    if c == b'#' {
                        Some((x as N, y as N, 0))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        run(data, &ADJUSTS3)
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let data = data.into_iter().map(|(x, y, z)| (x, y, z, 0)).collect();

        run(data, &ADJUSTS4)
    }
}

fn run<T: std::hash::Hash + Eq + Copy + TupleAdd>(mut data: HashSet<T>, adjusts: &[T]) -> usize {
    let mut counts: HashMap<_, N> = HashMap::new();

    for _ in 0..6 {
        counts.clear();

        for &pos in &data {
            for &adj in adjusts {
                let key = pos.tuple_add(&adj);
                *counts.entry(key).or_default() += 1;
            }
        }

        let mut next = HashSet::new();

        for &pos in &data {
            let c = *counts.get(&pos).unwrap_or(&0);
            if c == 2 || c == 3 {
                next.insert(pos);
            }
        }

        for (&pos, &count) in counts.iter().filter(|(p, _)| !data.contains(p)) {
            if count == 3 {
                next.insert(pos);
            }
        }

        data = next;
    }

    data.len()
}

trait TupleAdd {
    fn tuple_add(&self, other: &Self) -> Self;
}

impl TupleAdd for (N, N, N) {
    fn tuple_add(&self, other: &Self) -> Self {
        (self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl TupleAdd for (N, N, N, N) {
    fn tuple_add(&self, other: &Self) -> Self {
        (
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d17p1() {
        assert_eq!(
            Day17::part1(Day17::parse(
                ".#.
..#
###"
            )),
            112
        );
    }

    #[test]
    fn d17p2() {
        assert_eq!(
            Day17::part2(Day17::parse(
                ".#.
..#
###"
            )),
            848
        );
    }
}
