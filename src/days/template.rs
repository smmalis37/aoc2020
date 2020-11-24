use crate::solver::DaySolver;

pub struct DayNUM;

impl DaySolver<'_> for DayNUM {
    type Parsed = u32;
    type Output = u32;

    fn parse(input: &str) -> Self::Parsed {
        0
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        0
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dNUMp1() {}

    #[test]
    fn dNUMp2() {}
}
