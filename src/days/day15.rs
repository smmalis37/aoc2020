use std::collections::hash_map::Entry;

use rustc_hash::FxHashMap;

use crate::{day_solver::DaySolver, util::*};

pub struct Day15;

impl DaySolver<'_> for Day15 {
    type Parsed = Vec<usize>;
    type Output = usize;

    fn parse(input: &str) -> Self::Parsed {
        input
            .as_bytes()
            .split(|&x| x == b',')
            .map(|x| x.parse().unwrap())
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        run(&data, 2020)
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        run(&data, 30_000_000)
    }
}

fn run(data: &<Day15 as DaySolver>::Parsed, count: usize) -> usize {
    let mut map = FxHashMap::with_capacity_and_hasher(count, Default::default());
    for (i, &x) in data.iter().enumerate() {
        map.insert(x, i + 1);
    }

    let mut last = *data.last().unwrap();

    for turn in data.len()..count {
        match map.entry(last) {
            Entry::Occupied(mut v) => {
                let val = v.get_mut();
                last = turn - *val;
                *val = turn;
            }
            Entry::Vacant(v) => {
                v.insert(turn);
                last = 0;
            }
        }
    }

    last
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod tests {
    use super::*;

    #[test]
    fn d15p1() {
        assert_eq!(Day15::part1(Day15::parse("0,3,6")), 436);
        assert_eq!(Day15::part1(Day15::parse("1,3,2")), 1);
        assert_eq!(Day15::part1(Day15::parse("2,1,3")), 10);
        assert_eq!(Day15::part1(Day15::parse("1,2,3")), 27);
        assert_eq!(Day15::part1(Day15::parse("2,3,1")), 78);
        assert_eq!(Day15::part1(Day15::parse("3,2,1")), 438);
        assert_eq!(Day15::part1(Day15::parse("3,1,2")), 1836);
    }

    #[test]
    fn d15p2() {
        assert_eq!(Day15::part2(Day15::parse("0,3,6")), 175594);
        assert_eq!(Day15::part2(Day15::parse("1,3,2")), 2578);
        assert_eq!(Day15::part2(Day15::parse("2,1,3")), 3544142);
        assert_eq!(Day15::part2(Day15::parse("1,2,3")), 261214);
        assert_eq!(Day15::part2(Day15::parse("2,3,1")), 6895259);
        assert_eq!(Day15::part2(Day15::parse("3,2,1")), 18);
        assert_eq!(Day15::part2(Day15::parse("3,1,2")), 362);
    }
}
