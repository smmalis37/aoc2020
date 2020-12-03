use std::convert::TryInto;

use crate::day_solver::DaySolver;

pub struct Day3;

impl DaySolver<'_> for Day3 {
    type Parsed = Vec<Vec<bool>>;
    type Output = u64;

    fn parse(input: &str) -> Self::Parsed {
        input
            .as_bytes()
            .split(|&x| x == b'\n')
            .map(|x| x.iter().map(|&c| c == b'#').collect())
            .collect()
    }

    fn part1(map: Self::Parsed) -> Self::Output {
        run_slope(&map, 1, 3)
    }

    fn part2(map: Self::Parsed) -> Self::Output {
        run_slope(&map, 1, 1)
            * run_slope(&map, 1, 3)
            * run_slope(&map, 1, 5)
            * run_slope(&map, 1, 7)
            * run_slope(&map, 2, 1)
    }
}

fn run_slope<'a>(
    map: &<Day3 as DaySolver>::Parsed,
    y_count: usize,
    x_count: usize,
) -> <Day3 as DaySolver<'a>>::Output {
    map.iter()
        .step_by(y_count)
        .enumerate()
        .filter(|(i, y)| y[i * x_count % y.len()])
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d3p1() {
        assert_eq!(
            Day3::part1(Day3::parse(
                "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            )),
            7
        );
    }

    #[test]
    fn d3p2() {
        assert_eq!(
            Day3::part2(Day3::parse(
                "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            )),
            336
        );
    }
}
