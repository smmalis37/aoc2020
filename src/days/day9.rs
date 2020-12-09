use std::{cmp::Ordering::*, collections::VecDeque};

use bstr_parse::BStrParse;

use crate::day_solver::DaySolver;

pub struct Day9;

type N = u64;

impl DaySolver<'_> for Day9 {
    type Parsed = Vec<N>;
    type Output = N;

    fn parse(input: &str) -> Self::Parsed {
        input
            .as_bytes()
            .split(|&x| x == b'\n')
            .map(|x| x.parse().unwrap())
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        do_part_1(&data, 25)
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        do_part_2(&data, 25)
    }
}

fn do_part_1(data: &<Day9 as DaySolver>::Parsed, size: usize) -> N {
    let mut set: VecDeque<_> = data[0..size].iter().copied().collect();

    for &x in &data[size..] {
        if !set.iter().any(|&v| x > v && set.contains(&(x - v))) {
            return x;
        }

        set.pop_front();
        set.push_back(x);
    }

    unreachable!()
}

fn do_part_2(data: &<Day9 as DaySolver>::Parsed, size: usize) -> N {
    let value = do_part_1(data, size);

    let (mut i, mut j) = (0, 0);
    let mut sum = data[0];
    loop {
        match sum.cmp(&value) {
            Less => {
                j += 1;
                sum += data[j];
            }
            Equal => {
                let range = &data[i..=j];
                break range.iter().min().unwrap() + range.iter().max().unwrap();
            }
            Greater => {
                sum -= data[i];
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d9p1() {
        assert_eq!(
            do_part_1(
                &Day9::parse(
                    "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
                ),
                5
            ),
            127
        )
    }

    #[test]
    fn d9p2() {
        assert_eq!(
            do_part_2(
                &Day9::parse(
                    "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
                ),
                5
            ),
            62
        )
    }
}
