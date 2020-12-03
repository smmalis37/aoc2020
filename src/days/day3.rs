use crate::day_solver::DaySolver;

pub struct Day3;

impl DaySolver<'_> for Day3 {
    type Parsed = Vec<Vec<bool>>;
    type Output = u32;

    fn parse(input: &str) -> Self::Parsed {
        input
            .as_bytes()
            .split(|&x| x == b'\n')
            .map(|x| x.iter().map(|&c| c == b'#').collect())
            .collect()
    }

    fn part1(map: Self::Parsed) -> Self::Output {
        let mut x = 0;
        let mut count = 0;
        for y in map {
            if y[x] {
                count += 1;
            }

            x += 3;
            let line_len = y.len();
            if x >= line_len {
                x -= line_len;
            }
        }

        count
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d3p1() {}

    #[test]
    fn d3p2() {}
}
