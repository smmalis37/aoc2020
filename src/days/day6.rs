use crate::day_solver::DaySolver;

pub struct Day6;

type Group = Vec<Person>;
type Person = [bool; 26];

impl DaySolver<'_> for Day6 {
    type Parsed = Vec<Group>;
    type Output = usize;

    fn parse(input: &str) -> Self::Parsed {
        let mut results = Vec::new();
        let mut current = Group::new();

        for line in input.as_bytes().split(|&c| c == b'\n') {
            if line.is_empty() {
                results.push(current);
                current = Group::new();
            } else {
                let mut person = [false; 26];
                for answer in line {
                    person[(*answer - b'a') as usize] = true;
                }
                current.push(person);
            }
        }

        results.push(current);
        results
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.into_iter()
            .map(|g| (0..26usize).filter(|i| g.iter().any(|p| p[*i])).count())
            .sum()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        data.into_iter()
            .map(|g| (0..26usize).filter(|i| g.iter().all(|p| p[*i])).count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d6p1() {}

    #[test]
    fn d6p2() {}
}
