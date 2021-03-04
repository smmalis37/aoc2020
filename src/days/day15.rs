use std::num::NonZeroU32;

use crate::{day_solver::DaySolver, util::*};

pub struct Day15;

type N = u32;
type NZ = NonZeroU32;

impl DaySolver<'_> for Day15 {
    type Parsed = Vec<N>;
    type Output = N;

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

fn run(data: &<Day15 as DaySolver>::Parsed, count: N) -> N {
    let mut map: Vec<Option<NZ>> = vec![None; count as usize];
    for (&x, i) in data.iter().zip(1..) {
        map[x as usize] = NZ::new(i);
    }

    let mut last = *data.last().unwrap();

    for turn in data.len() as N..count {
        let i = last as usize;
        let nz_turn = NZ::new(turn);
        if let Some(v) = map[i] {
            map[i] = nz_turn;
            last = nz_turn.unwrap().get() - v.get();
        } else {
            map[i] = nz_turn;
            last = 0;
        }
    }

    last
}

#[cfg(test)]
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
        assert_eq!(Day15::part2(Day15::parse("0,3,6")), 175_594);
        assert_eq!(Day15::part2(Day15::parse("1,3,2")), 2578);
        assert_eq!(Day15::part2(Day15::parse("2,1,3")), 3_544_142);
        assert_eq!(Day15::part2(Day15::parse("1,2,3")), 261_214);
        assert_eq!(Day15::part2(Day15::parse("2,3,1")), 6_895_259);
        assert_eq!(Day15::part2(Day15::parse("3,2,1")), 18);
        assert_eq!(Day15::part2(Day15::parse("3,1,2")), 362);
    }
}
