use crate::day_solver::DaySolver;

pub struct Day18;

type N = u64;

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mult,
}

use Op::*;

impl<'a> DaySolver<'a> for Day18 {
    type Parsed = impl Iterator<Item = impl Iterator<Item = u8> + 'a> + Clone;
    type Output = N;

    fn parse(input: &'a str) -> Self::Parsed {
        input
            .as_bytes()
            .split(|&x| x == b'\n')
            .map(|l| l.iter().filter(|&&x| x != b' ').copied())
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.map(|mut e| eval(&mut e)).sum()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        data.map(|e| eval(&mut transform(e))).sum()
    }
}

fn transform(source: impl Iterator<Item = u8>) -> impl Iterator<Item = u8> {
    AdditionWrapper {
        source,
        previous: Some(b'('),
        previous2: None,
    }
}

struct AdditionWrapper<T: Iterator<Item = u8>> {
    source: T,
    previous: Option<u8>,
    previous2: Option<u8>,
}

impl<T: Iterator<Item = u8>> Iterator for AdditionWrapper<T> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.previous {
            self.previous = self.previous2.take();
            Some(x)
        } else if let Some(x) = self.source.next() {
            match x {
                b'*' => {
                    self.previous = Some(x);
                    self.previous2 = Some(b'(');
                    Some(b')')
                }
                b'(' => {
                    self.previous = Some(b'(');
                    Some(b'(')
                }
                b')' => {
                    self.previous = Some(b')');
                    Some(b')')
                }
                b' ' => unreachable!(),
                x => Some(x),
            }
        } else {
            Some(b')')
        }
    }
}

fn eval(tokens: &mut impl Iterator<Item = u8>) -> N {
    let mut prev_op = Some(Add);
    let mut result = 0;

    while let Some(i) = tokens.next() {
        match i {
            x @ b'0'..=b'9' => {
                result = apply_op(prev_op.unwrap(), result, convert(x));
                prev_op = None;
            }
            b'+' => {
                debug_assert!(prev_op.is_none());
                prev_op = Some(Add);
            }
            b'*' => {
                debug_assert!(prev_op.is_none());
                prev_op = Some(Mult);
            }
            b'(' => {
                let res = eval(tokens);
                result = apply_op(prev_op.unwrap(), result, res);
                prev_op = None;
            }
            b')' => break,
            _ => unreachable!(),
        }
    }

    result
}

#[allow(clippy::cast_lossless)]
fn convert(x: u8) -> N {
    (x - b'0') as N
}

fn apply_op(prev_op: Op, result: N, x: N) -> N {
    match prev_op {
        Add => result + x,
        Mult => result * x,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d18p1() {
        assert_eq!(Day18::part1(Day18::parse("1 + 2 * 3 + 4 * 5 + 6")), 71);
        assert_eq!(
            Day18::part1(Day18::parse("1 + (2 * 3) + (4 * (5 + 6))")),
            51
        );
        assert_eq!(Day18::part1(Day18::parse("2 * 3 + (4 * 5)")), 26);
        assert_eq!(
            Day18::part1(Day18::parse("5 + (8 * 3 + 9 + 3 * 4 * 3)")),
            437
        );
        assert_eq!(
            Day18::part1(Day18::parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")),
            12240
        );
        assert_eq!(
            Day18::part1(Day18::parse(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            )),
            13632
        );
    }

    #[test]
    fn d18p2() {
        assert_eq!(Day18::part2(Day18::parse("1 + 2 * 3 + 4 * 5 + 6")), 231);
        assert_eq!(
            Day18::part1(Day18::parse("1 + (2 * 3) + (4 * (5 + 6))")),
            51
        );
        assert_eq!(Day18::part2(Day18::parse("2 * 3 + (4 * 5)")), 46);
        assert_eq!(
            Day18::part2(Day18::parse("5 + (8 * 3 + 9 + 3 * 4 * 3)")),
            1445
        );
        assert_eq!(
            Day18::part2(Day18::parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")),
            669_060
        );
        assert_eq!(
            Day18::part2(Day18::parse(
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
            )),
            23340
        );
    }
}
