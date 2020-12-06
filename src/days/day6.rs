use crate::day_solver::DaySolver;

pub struct Day6;

impl DaySolver<'_> for Day6 {
    type Parsed = Vec<[bool; 26]>;
    type Output = usize;

    fn parse(input: &str) -> Self::Parsed {
        let mut results = Vec::new();
        let mut current = [false; 26];

        for line in input.as_bytes().split(|&c| c == b'\n') {
            if line.is_empty() {
                results.push(current);
                current = [false; 26];
            } else {
                for answer in line {
                    current[(*answer - b'a') as usize] = true;
                }
            }
        }

        results.push(current);
        results
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.into_iter()
            .map(|a| a.iter().filter(|&&x| x).count())
            .sum()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        todo!()
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
