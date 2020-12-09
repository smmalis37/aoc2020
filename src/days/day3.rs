use crate::day_solver::DaySolver;

pub struct Day3;

impl<'a> DaySolver<'a> for Day3 {
    type Parsed = (&'a [u8], usize);
    type Output = u64;

    fn parse(input: &'a str) -> Self::Parsed {
        (
            input.as_bytes(),
            input
                .as_bytes()
                .split(|&x| x == b'\n')
                .next()
                .unwrap()
                .len(),
        )
    }

    fn part1(map: Self::Parsed) -> Self::Output {
        run_slope(&map, (1, 3))
    }

    fn part2(map: Self::Parsed) -> Self::Output {
        [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
            .iter()
            .map(|&s| run_slope(&map, s))
            .product()
    }
}

fn run_slope<'a>(
    map: &<Day3 as DaySolver>::Parsed,
    (y_count, x_count): (usize, usize),
) -> <Day3 as DaySolver<'a>>::Output {
    let real_line_length = map.1;
    let math_line_length = real_line_length + 1; // newlines
    let grid = &map.0;

    let mut y_pos = 0;
    let mut x_pos = 0;
    let mut count = 0;

    while y_pos * math_line_length < grid.len() {
        if grid[y_pos * math_line_length + x_pos] == b'#' {
            count += 1;
        }
        y_pos += y_count;
        x_pos = (x_pos + x_count) % real_line_length;
    }

    count
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
