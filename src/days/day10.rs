use std::collections::HashMap;

use crate::{day_solver::DaySolver, util::*};

pub struct Day10;

impl DaySolver<'_> for Day10 {
    type Parsed = Vec<usize>;
    type Output = u64;

    fn parse(input: &str) -> Self::Parsed {
        let mut jolts: Self::Parsed = input
            .as_bytes()
            .split(|&x| x == b'\n')
            .map(|x| x.parse().unwrap())
            .collect();
        jolts.sort_unstable();

        jolts
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut diffs = [0; 4];
        diffs[data[0]] += 1;

        for w in data.windows(2) {
            diffs[w[1] - w[0]] += 1;
        }

        diffs[3] += 1;

        diffs[1] * diffs[3]
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        recurse(0, &data, &mut HashMap::new())
    }
}

fn recurse(x: usize, data: &[usize], solutions: &mut HashMap<usize, u64>) -> u64 {
    if let Some(&v) = solutions.get(&x) {
        return v;
    }

    if data.is_empty() {
        return 1;
    }

    let mut sum = 0;

    for i in 0..3 {
        if i >= data.len() {
            sum += 0;
        } else if data[i] - x <= 3 {
            sum += recurse(data[i], &data[i + 1..], solutions);
        }
    }

    solutions.insert(x, sum);

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d10p1() {}

    #[test]
    fn d10p2() {}
}
