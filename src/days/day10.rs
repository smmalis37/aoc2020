use std::{convert::TryInto, num::NonZeroU64};

use crate::{day_solver::DaySolver, util::*};

pub struct Day10;

type N = u16;

impl DaySolver<'_> for Day10 {
    type Parsed = Vec<N>;
    type Output = u64;

    fn parse(input: &str) -> Self::Parsed {
        let mut jolts: Self::Parsed = input
            .as_bytes()
            .split(|&x| x == b'\n')
            .map(|x| x.parse().unwrap())
            .chain(std::iter::once(0))
            .collect();
        jolts.sort_unstable();
        jolts.push(jolts.last().unwrap() + 3);

        jolts
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut diffs = [0; 4];

        for w in data.windows(2) {
            diffs[(w[1] - w[0]) as usize] += 1;
        }

        diffs[1] * diffs[3]
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        i_dislike_dynamic_programming(&data, &mut vec![None; (*data.last().unwrap() + 1) as usize])
    }
}

fn i_dislike_dynamic_programming<'a>(
    data: &[N],
    solutions: &mut Vec<Option<NonZeroU64>>,
) -> <Day10 as DaySolver<'a>>::Output {
    let (&x, data) = data.split_first().unwrap();

    if let Some(sol) = solutions[x as usize] {
        return sol.get();
    }

    if data.is_empty() {
        return 1;
    }

    let mut sum = 0;

    for i in 0..std::cmp::min(3, data.len()) {
        if data[i] - x <= 3 {
            sum += i_dislike_dynamic_programming(&data[i..], solutions);
        }
    }

    solutions[x as usize] = Some(sum.try_into().unwrap());

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d10p1() {
        assert_eq!(
            Day10::part1(Day10::parse(
                "16
10
15
5
1
11
7
19
6
12
4"
            )),
            7 * 5
        );

        assert_eq!(
            Day10::part1(Day10::parse(
                "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
            )),
            22 * 10
        );
    }

    #[test]
    fn d10p2() {
        assert_eq!(
            Day10::part2(Day10::parse(
                "16
10
15
5
1
11
7
19
6
12
4"
            )),
            8
        );

        assert_eq!(
            Day10::part2(Day10::parse(
                "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
            )),
            19208
        );
    }
}
