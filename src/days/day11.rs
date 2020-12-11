#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

use std::collections::HashSet;

use crate::day_solver::DaySolver;

pub struct Day11;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Position {
    Floor,
    Empty,
    Occupied,
}

use Position::*;

static ADJUSTS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl DaySolver<'_> for Day11 {
    type Parsed = Vec<Vec<Position>>;
    type Output = usize;

    fn parse(input: &str) -> Self::Parsed {
        input
            .as_bytes()
            .split(|&x| x == b'\n')
            .map(|l| {
                l.iter()
                    .map(|c| match c {
                        b'.' => Floor,
                        b'L' => Empty,
                        b'#' => Occupied,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect()
    }

    fn part1(mut data: Self::Parsed) -> Self::Output {
        let mut next = data.clone();
        let mut counts = vec![vec![0_u8; data[0].len()]; data.len()];

        loop {
            for i in 0..data.len() {
                for j in 0..data[i].len() {
                    if matches!(data[i][j], Occupied) {
                        for &(x, y) in &ADJUSTS {
                            if let Some(c) = counts
                                .get_mut((i as isize + x) as usize)
                                .and_then(|row| row.get_mut((j as isize + y) as usize))
                            {
                                *c += 1;
                            }
                        }
                    }
                }
            }

            let mut change = false;
            for i in 0..data.len() {
                for j in 0..data[i].len() {
                    next[i][j] = match data[i][j] {
                        Empty if counts[i][j] == 0 => {
                            change = true;
                            Occupied
                        }
                        Occupied if counts[i][j] >= 4 => {
                            change = true;
                            Empty
                        }
                        _ => data[i][j],
                    }
                }
            }

            if !change {
                break data
                    .into_iter()
                    .flatten()
                    .filter(|x| matches!(x, Occupied))
                    .count();
            }

            counts.iter_mut().flatten().for_each(|x| *x = 0);
            data.iter_mut()
                .flatten()
                .zip(next.iter_mut().flatten())
                .for_each(|(d, n)| *d = *n);
        }
    }

    fn part2(mut data: Self::Parsed) -> Self::Output {
        let mut next = data.clone();
        let mut counts = vec![vec![HashSet::with_capacity(8); data[0].len()]; data.len()];

        loop {
            for i in 0..data.len() {
                for j in 0..data[i].len() {
                    if matches!(data[i][j], Occupied) {
                        for &(x, y) in &ADJUSTS {
                            let mut mult = 1;
                            let mut ix = (i as isize + (x * mult)) as usize;
                            let mut jy = (j as isize + (y * mult)) as usize;
                            while let Some(d) = data.get_mut(ix).and_then(|row| row.get_mut(jy)) {
                                if !matches!(d, Floor) {
                                    counts[ix][jy].insert((x, y));
                                    break;
                                }

                                mult += 1;
                                ix = (i as isize + (x * mult)) as usize;
                                jy = (j as isize + (y * mult)) as usize;
                            }
                        }
                    }
                }
            }

            let mut change = false;
            for i in 0..data.len() {
                for j in 0..data[i].len() {
                    next[i][j] = match data[i][j] {
                        Empty if counts[i][j].is_empty() => {
                            change = true;
                            Occupied
                        }
                        Occupied if counts[i][j].len() >= 5 => {
                            change = true;
                            Empty
                        }
                        _ => data[i][j],
                    }
                }
            }

            if !change {
                break data
                    .into_iter()
                    .flatten()
                    .filter(|x| matches!(x, Occupied))
                    .count();
            }

            counts.iter_mut().flatten().for_each(HashSet::clear);
            data.iter_mut()
                .flatten()
                .zip(next.iter_mut().flatten())
                .for_each(|(d, n)| *d = *n);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d11p1() {
        assert_eq!(
            Day11::part1(Day11::parse(
                "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            )),
            37
        );
    }

    #[test]
    fn d11p2() {
        assert_eq!(
            Day11::part2(Day11::parse(
                "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            )),
            26
        );
    }
}
