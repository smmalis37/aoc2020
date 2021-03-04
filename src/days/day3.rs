use crate::{day_solver::DaySolver, util::*};

pub struct Day3;

impl DaySolver<'_> for Day3 {
    type Parsed = Grid<bool>;
    type Output = u64;

    fn parse(input: &str) -> Self::Parsed {
        input
            .as_bytes()
            .split(|&x| x == b'\n')
            .map(|x| x.iter().map(|&c| (c == b'#')))
            .collect()
    }

    fn part1(map: Self::Parsed) -> Self::Output {
        run_slope(&map, (3, 1))
    }

    fn part2(map: Self::Parsed) -> Self::Output {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|&s| run_slope(&map, s))
            .product()
    }
}

fn run_slope<'a>(
    map: &<Day3 as DaySolver>::Parsed,
    (x_count, y_count): (usize, usize),
) -> <Day3 as DaySolver<'a>>::Output {
    let line_length = map.line_length();
    map.iter()
        .step_by(y_count)
        .enumerate()
        .filter(|(i, y)| y[i * x_count % line_length])
        .count() as <Day3 as DaySolver<'_>>::Output
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
