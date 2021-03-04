use crate::{day_solver::DaySolver, util::*};

pub struct Day13;

type N = u64;

impl DaySolver<'_> for Day13 {
    type Parsed = (N, Vec<Option<N>>);
    type Output = N;

    fn parse(input: &str) -> Self::Parsed {
        let input = input.as_bytes();
        let linebreak = memchr::memchr(b'\n', input).unwrap();

        (
            input[0..linebreak].parse().unwrap(),
            input[linebreak + 1..]
                .split(|&x| x == b',')
                .map(|x| {
                    if x == b"x" {
                        None
                    } else {
                        Some(x.parse().unwrap())
                    }
                })
                .collect(),
        )
    }

    fn part1((timestamp, ids): Self::Parsed) -> Self::Output {
        let mut min = N::MAX;
        let mut min_id = N::MAX;
        for id in ids.into_iter().flatten() {
            let rem = id - (timestamp % id);
            if rem < min {
                min = rem;
                min_id = id;
            }
        }

        min_id * min
    }

    fn part2((_, ids): Self::Parsed) -> Self::Output {
        let mut previous = 1;
        let mut products = 1;
        for (i, id) in ids.into_iter().enumerate() {
            let i = i as N;
            if let Some(id) = id {
                for x in (previous..).step_by(products as usize) {
                    if x % id == (id - (i % id)) % id {
                        previous = x;
                        products *= id;
                        break;
                    }
                }
            }
        }

        previous
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d13p1() {
        assert_eq!(
            Day13::part1(Day13::parse(
                "939
7,13,x,x,59,x,31,19"
            )),
            295
        );
    }

    #[test]
    fn d13p2() {
        assert_eq!(
            Day13::part2(Day13::parse(
                "939
7,13,x,x,59,x,31,19"
            )),
            1_068_781
        );
    }
}
