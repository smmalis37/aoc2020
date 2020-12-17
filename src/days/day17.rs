#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

use std::collections::{HashMap, HashSet};

use crate::day_solver::DaySolver;

pub struct Day17;

type N = i16;

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
                l.iter()
                    .enumerate()
                    .filter(|(_, &c)| c == b'#')
                    .map(move |(x, _)| (x as N, y as N, 0))
            })
            .collect()
    }

    fn part1(mut data: Self::Parsed) -> Self::Output {
        for _ in 0..6 {
            let mut counts: HashMap<_, N> = HashMap::new();

            for (x, y, z) in &data {
                for &(i, j, k) in &ADJUSTS3 {
                    let key = (x + i, y + j, z + k);
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

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut data: HashSet<(i16, i16, i16, i16)> =
            data.into_iter().map(|(x, y, z)| (x, y, z, 0)).collect();

        for _ in 0..6 {
            let mut counts: HashMap<_, N> = HashMap::new();

            for (x, y, z, w) in &data {
                for &(i, j, k, l) in &ADJUSTS4 {
                    let key = (x + i, y + j, z + k, w + l);
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
