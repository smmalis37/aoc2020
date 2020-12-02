use std::ops::RangeInclusive;

use crate::day_solver::DaySolver;

pub struct Day2;

#[derive(Clone)]
pub struct Password<'a> {
    count: RangeInclusive<usize>,
    character: char,
    password: &'a str,
}

impl<'a> DaySolver<'a> for Day2 {
    type Parsed = Vec<Password<'a>>;
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        input
            .lines()
            .map(|x| {
                let (min, max, character, password) = scan!("{}-{} {}: {}" <- x).unwrap();
                Password {
                    count: min..=max,
                    character,
                    password,
                }
            })
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.into_iter()
            .filter(|p| p.count.contains(&p.password.matches(p.character).count()))
            .count()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d2p1() {
        assert_eq!(
            Day2::part1(Day2::parse(
                "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
            )),
            2
        );
    }

    #[test]
    fn d2p2() {}
}
