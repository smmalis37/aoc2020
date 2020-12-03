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
    let mut xs = vec![0; slopes.len()];
    let mut counts = vec![0; slopes.len()];
    for (y_i, y) in map.into_iter().enumerate() {
        for (i, x) in xs.iter_mut().enumerate() {
            if y_i % slopes[i].0 != 0 {
                continue;
            }

            if y[*x] {
                counts[i] += 1;
            }

            *x += slopes[i].1;
            let line_len = y.len();
            if *x >= line_len {
                *x -= line_len;
            }
        }
    }

    counts.into_iter().product()
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
