use crate::day_solver::DaySolver;

pub struct Day6;

#[derive(Clone)]
pub struct Group {
    person_count: usize,
    answers: [usize; 26],
}

impl DaySolver<'_> for Day6 {
    type Parsed = Vec<Group>;
    type Output = usize;

    fn parse(input: &str) -> Self::Parsed {
        let mut results = Vec::new();
        let mut answers = [0; 26];
        let mut person_count = 0;

        for line in input.as_bytes().split(|&c| c == b'\n') {
            if line.is_empty() {
                results.push(Group {
                    person_count,
                    answers,
                });

                answers = [0; 26];
                person_count = 0;
            } else {
                person_count += 1;
                for answer in line {
                    answers[(*answer - b'a') as usize] += 1;
                }
            }
        }

        results.push(Group {
            person_count,
            answers,
        });
        results
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.into_iter()
            .map(|g| g.answers.iter().filter(|&&a| a > 0).count())
            .sum()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        data.into_iter()
            .map(|g| g.answers.iter().filter(|&&a| a == g.person_count).count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d6p1() {
        assert_eq!(
            Day6::part1(Day6::parse(
                "abc

a
b
c

ab
ac

a
a
a
a

b"
            )),
            11
        );
    }

    #[test]
    fn d6p2() {
        assert_eq!(
            Day6::part2(Day6::parse(
                "abc

a
b
c

ab
ac

a
a
a
a

b"
            )),
            6
        );
    }
}
