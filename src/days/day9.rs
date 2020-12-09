use bstr_parse::BStrParse;

use crate::day_solver::DaySolver;

pub struct Day9;

impl DaySolver<'_> for Day9 {
    type Parsed = Vec<u64>;
    type Output = u64;

    fn parse(input: &str) -> Self::Parsed {
        input
            .as_bytes()
            .split(|&x| x == b'\n')
            .map(|x| x.parse().unwrap())
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let size = 25;
        for w in data.windows(size + 1) {
            let (&sum, values) = w.split_last().unwrap();

            if values
                .iter()
                .any(|&v1| values.iter().any(|&v2| v1 + v2 == sum))
            {
                continue;
            }

            return sum;
        }

        unreachable!()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let size = 25;
        let (mut found, mut index) = (0, 0);
        for (i, w) in data.windows(size + 1).enumerate() {
            let (&sum, values) = w.split_last().unwrap();

            if values
                .iter()
                .any(|&v1| values.iter().any(|&v2| v1 + v2 == sum))
            {
                continue;
            }

            found = sum;
            index = i + size;
            break;
        }

        for i in 0..index {
            for j in i..index {
                if data[i..=j].iter().sum::<u64>() == found {
                    return data[i..=j].iter().min().unwrap() + data[i..=j].iter().max().unwrap();
                }
            }
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d9p1() {}

    #[test]
    fn d9p2() {}
}
