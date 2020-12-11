#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

use crate::{day_solver::DaySolver, util::*};

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
    type Parsed = Grid<Position>;
    type Output = usize;

    fn parse(input: &str) -> Self::Parsed {
        input
            .as_bytes()
            .split(|&x| x == b'\n')
            .map(|l| {
                l.iter().map(|c| match c {
                    b'.' => Floor,
                    b'L' => Empty,
                    b'#' => Occupied,
                    _ => unreachable!(),
                })
            })
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        run(
            data,
            4,
            |counts, _, i, j, x, y, _| {
                if let Some(c) = counts
                    .get_mut((i as isize + x) as usize)
                    .and_then(|row| row.get_mut((j as isize + y) as usize))
                {
                    *c += 1;
                }
            },
            |c: &u8| *c as usize,
        )
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        run(
            data,
            5,
            |counts: &mut Grid<[bool; 8]>, data, i, j, x, y, a| {
                let mut ix = (i as isize + x) as usize;
                let mut jy = (j as isize + y) as usize;

                while let Some(d) = data.get(ix).and_then(|row| row.get(jy)) {
                    if !matches!(d, Floor) {
                        counts[ix][jy][a] = true;
                        break;
                    }

                    ix = (ix as isize + x) as usize;
                    jy = (jy as isize + y) as usize;
                }
            },
            |c| c.iter().filter(|&&x| x).count(),
        )
    }
}

fn run<'a, Count: Default + Clone>(
    mut data: <Day11 as DaySolver>::Parsed,
    occupied_count: usize,
    adjuster: impl Fn(
        &mut Grid<Count>,
        &<Day11 as DaySolver<'a>>::Parsed,
        usize,
        usize,
        isize,
        isize,
        usize,
    ),
    get_count: impl Fn(&Count) -> usize,
) -> usize {
    let mut counts = std::iter::repeat(std::iter::repeat(Count::default()).take(data[0].len()))
        .take(data.len())
        .collect();

    loop {
        for i in 0..data.len() {
            for j in 0..data[i].len() {
                if matches!(data[i][j], Occupied) {
                    for (a, &(x, y)) in ADJUSTS.iter().enumerate() {
                        adjuster(&mut counts, &data, i, j, x, y, a);
                    }
                }
            }
        }

        let mut change = false;
        data.iter_mut()
            .flatten()
            .zip(counts.iter().flatten())
            .for_each(|(d, c)| {
                let c = get_count(c);
                *d = match d {
                    Empty if c == 0 => {
                        change = true;
                        Occupied
                    }
                    Occupied if c >= occupied_count => {
                        change = true;
                        Empty
                    }
                    _ => *d,
                }
            });

        if !change {
            break data
                .iter()
                .flatten()
                .filter(|x| matches!(x, Occupied))
                .count();
        }

        counts
            .iter_mut()
            .flatten()
            .for_each(|c| *c = Count::default());
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
