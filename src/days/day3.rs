use crate::day_solver::DaySolver;

pub struct Day3;

impl DaySolver<'_> for Day3 {
    type Parsed = Vec<Vec<bool>>;
    type Output = u64;

    fn parse(input: &str) -> Self::Parsed {
        input
            .as_bytes()
            .split(|&x| x == b'\n')
            .map(|x| x.iter().map(|&c| (c == b'#')).collect())
            .collect()
    }

    fn part1(map: Self::Parsed) -> Self::Output {
        run_slopes(map, &[(1, 3)])
    }

    fn part2(map: Self::Parsed) -> Self::Output {
        run_slopes(map, &[(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)])
    }
}

fn run_slopes<'a>(
    map: <Day3 as DaySolver>::Parsed,
    slopes: &[(usize, usize)],
) -> <Day3 as DaySolver<'a>>::Output {
    let line_length = map[0].len();
    slopes
        .iter()
        .map(|&(y_count, x_count)| {
            map.iter()
                .step_by(y_count)
                .enumerate()
                .filter(|(i, y)| y[i * x_count % line_length])
                .count() as <Day3 as DaySolver<'_>>::Output
        })
        .product()
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
