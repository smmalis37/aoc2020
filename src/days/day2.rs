use crate::day_solver::DaySolver;

pub struct Day2;

#[derive(Clone)]
pub struct Password<'a> {
    num1: usize,
    num2: usize,
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
                let (num1, num2, character, password) = scan!("{}-{} {}: {}" <- x).unwrap();
                Password {
                    num1,
                    num2,
                    character,
                    password,
                }
            })
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.into_iter()
            .filter(|p| (p.num1..=p.num2).contains(&p.password.matches(p.character).count()))
            .count()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        data.into_iter()
            .filter(|p| {
                (p.password.as_bytes()[p.num1 - 1] as char == p.character)
                    != (p.password.as_bytes()[p.num2 - 1] as char == p.character)
            })
            .count()
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
    fn d2p2() {
        assert_eq!(
            Day2::part2(Day2::parse(
                "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
            )),
            1
        );
    }
}
