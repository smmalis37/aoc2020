use crate::day_solver::DaySolver;

pub struct Day1;

impl DaySolver<'_> for Day1 {
    type Parsed = Vec<Self::Output>;
    type Output = u32;

    fn parse(input: &str) -> Self::Parsed {
        let mut res: Self::Parsed = input.lines().map(|x| x.parse().unwrap()).collect();
        res.sort_unstable();
        res
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        for (i, x) in data.iter().enumerate() {
            let y = 2020 - x;
            if data[i + 1..].binary_search(&y).is_ok() {
                return x * y;
            }
        }
        unreachable!()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        for (i, x) in data.iter().enumerate() {
            for (j, y) in data[i + 1..].iter().enumerate() {
                let z = 2020 - x - y;
                if data[i + j + 1..].binary_search(&z).is_ok() {
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
