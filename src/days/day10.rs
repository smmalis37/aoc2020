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
        // vec![0] is special cased and faster than non-zero. Since we only need the first spot to be 1, do this instead of vec![1].
        let mut solutions = vec![0; data.len()];
        solutions[0] = 1;

        for i in 1..data.len() {
            let mut sum = 0;
            for j in i.saturating_sub(3)..i {
                if data[i] - data[j] <= 3 {
                    sum += solutions[j];
                }
            }
            solutions[i] = sum;
        }

        *solutions.last().unwrap()
    }
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
