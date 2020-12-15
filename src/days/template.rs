use crate::day_solver::DaySolver;

pub struct DayNUM;

impl DaySolver<'_> for DayNUM {
    type Parsed = u32;
    type Output = u32;

    fn parse(input: &str) -> Self::Parsed {
        todo!()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        todo!()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d15p1() {
        assert_eq!(DayNUM::part1(DayNUM::parse("")), 0);
    }

    #[test]
    fn d14p2() {
        assert_eq!(DayNUM::part2(DayNUM::parse("")), 0);
    }
}
