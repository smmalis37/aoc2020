use crate::day_solver::DaySolver;

pub struct Day5;

impl<'a> DaySolver<'a> for Day5 {
    type Parsed = Vec<Self::Output>;
    type Output = u16;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut result = Vec::new();

        for line in input.as_bytes().split(|&x| x == b'\n') {
            let mut num = 0;
            for x in line {
                let bit = match x {
                    b'F' => 0,
                    b'B' => 1,
                    b'L' => 0,
                    b'R' => 1,
                    _ => unreachable!(),
                };

                num = (num << 1) + bit;
            }

            result.push(num)
        }

        result
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.into_iter().max().unwrap()
    }

    fn part2(mut data: Self::Parsed) -> Self::Output {
        data.sort_unstable();
        for i in 0..data.len() - 1 {
            if data[i] + 1 != data[i + 1] {
                return data[i] + 1;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d5p1() {
        assert_eq!(
            Day5::parse(
                "FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"
            ),
            &[357, 567, 119, 820]
        );
    }
}
