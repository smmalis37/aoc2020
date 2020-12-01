use crate::day_solver::DaySolver;

pub struct Day1;
type Num = u32;

impl DaySolver<'_> for Day1 {
    type Parsed = Vec<Num>;
    type Output = Num;

    fn parse(input: &str) -> Self::Parsed {
        let mut res: Self::Parsed = input.lines().map(|x| x.parse().unwrap()).collect();
        res.sort_unstable();
        res
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        for x in &data {
            let y = 2020 - x;
            if data.binary_search(&y).is_ok() {
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
                if data.binary_search(&y).is_ok() && data.binary_search(&z).is_ok() {
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
