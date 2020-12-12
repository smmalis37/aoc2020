#![allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]

use crate::{day_solver::DaySolver, util::*};

use arrayvec::ArrayVec;

pub struct Day11;

#[derive(Copy, Clone)]
pub enum Position {
    Floor,
    Empty,
    Occupied,
}

use Position::*;

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
        let neighbors = neighbors_loop(&data, |i, j, x, y, n| {
            let ix = (i as isize + x) as usize;
            let jy = (j as isize + y) as usize;
            if data.get(ix).and_then(|r| r.get(jy)).is_some() {
                n.push((ix, jy));
            }
        });

        run(data, &neighbors, 4)
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let neighbors = neighbors_loop(&data, |i, j, x, y, n| {
            let mut ix = (i as isize + x) as usize;
            let mut jy = (j as isize + y) as usize;

            while let Some(d) = data.get(ix).and_then(|r| r.get(jy)) {
                if !matches!(d, Floor) {
                    n.push((ix, jy));
                    break;
                }

                ix = (ix as isize + x) as usize;
                jy = (jy as isize + y) as usize;
            }
        });

        run(data, &neighbors, 5)
    }
}

type Neighbors = ArrayVec<[(usize, usize); 8]>;
type NeighborsGrid = Grid<Neighbors>;

fn neighbors_loop(
    data: &<Day11 as DaySolver>::Parsed,
    neighbor_func: impl Fn(usize, usize, isize, isize, &mut Neighbors),
) -> NeighborsGrid {
    let mut neighbors = Grid::from_value(Neighbors::new(), data.line_length(), data.len());

    for i in 0..data.len() {
        for j in 0..data.line_length() {
            if !matches!(data[i][j], Floor) {
                for &(x, y) in &[
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    neighbor_func(i, j, x, y, &mut neighbors[i][j]);
                }
            }
        }
    }

    neighbors
}

fn run(
    mut data: <Day11 as DaySolver>::Parsed,
    neighbors: &NeighborsGrid,
    occupied_count: u8,
) -> usize {
    let mut counts = Grid::from_value(0_u8, data.line_length(), data.len());

    loop {
        data.flat_iter()
            .zip(neighbors.flat_iter())
            .filter(|(d, _)| matches!(d, Occupied))
            .for_each(|(_, n)| {
                for &(x, y) in n {
                    counts[x][y] += 1;
                }
            });

        let mut change = false;
        data.flat_iter_mut()
            .zip(counts.flat_iter())
            .for_each(|(d, &c)| {
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
            break data.flat_iter().filter(|x| matches!(x, Occupied)).count();
        }

        counts.flat_iter_mut().for_each(|c| *c = 0);
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
