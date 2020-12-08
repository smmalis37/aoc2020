use crate::day_solver::DaySolver;

pub struct Day5;

impl<'a> DaySolver<'a> for Day5 {
    type Parsed = Vec<Self::Output>;
    type Output = u16;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut result = Vec::new();

        for line in input.as_bytes().split(|&x| x == b'\n') {
            let mut num = 0;
            for x in line {
                let bit = match x {
                    b'F' | b'L' => 0,
                    b'B' | b'R' => 1,
                    _ => unreachable!(),
                };

                num = (num << 1) + bit;
            }

            result.push(num)
        }

        result
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.into_iter().max().unwrap()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut min = Self::Output::MAX;
        let mut max = Self::Output::MIN;
        let mut total = 0;

        for x in data {
            min = std::cmp::min(min, x);
            max = std::cmp::max(max, x);
            total ^= x;
        }

        (min..=max).fold(0, |a, x| a ^ x) ^ total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d5p1() {
        assert_eq!(
            Day5::parse(
                "FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"
            ),
            &[357, 567, 119, 820]
        );
    }
}
