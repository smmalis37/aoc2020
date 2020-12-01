use std::collections::BTreeSet;

use crate::day_solver::DaySolver;

pub struct Day1;

impl DaySolver<'_> for Day1 {
    type Parsed = BTreeSet<u32>;
    type Output = u32;

    fn parse(input: &str) -> Self::Parsed {
        input.lines().map(|x| x.parse().unwrap()).collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        for x in &data {
            let y = 2020 - x;
            if data.contains(&y) {
                return x * y;
            }
        }
        unreachable!()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        for x in &data {
            let rem = 2020 - x;
            for y in 0..rem {
                let z = rem - y;
                if data.contains(&y) && data.contains(&z) {
                    return x * y * z;
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1p1() {
        assert_eq!(
            Day1::part1(Day1::parse(
                "1721
979
366
299
675
1456"
            )),
            1721 * 299
        );
    }

    #[test]
    fn d1p2() {
        assert_eq!(
            Day1::part2(Day1::parse(
                "1721
979
366
299
675
1456"
            )),
            979 * 366 * 675
        );
    }
}
