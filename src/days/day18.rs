#![allow(clippy::cast_lossless)]
use crate::day_solver::DaySolver;

pub struct Day18;

#[derive(Copy, Clone)]
pub enum Token {
    Num(u8),
    Add,
    Mult,
    OpenParen,
    CloseParen,
}

use Token::*;

type N = u64;

impl DaySolver<'_> for Day18 {
    type Parsed = Vec<Vec<Token>>;
    type Output = N;

    fn parse(input: &str) -> Self::Parsed {
        input
            .as_bytes()
            .split(|&x| x == b'\n')
            .map(|l| {
                l.iter()
                    .filter_map(|c| match c {
                        b'0'..=b'9' => Some(Num(c - b'0')),
                        b'+' => Some(Add),
                        b'*' => Some(Mult),
                        b'(' => Some(OpenParen),
                        b')' => Some(CloseParen),
                        b' ' => None,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.into_iter().map(|e| eval(&e).0).sum()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        data.into_iter().map(|e| eval(&transform(e)).0).sum()
    }
}

fn transform(mut e: Vec<Token>) -> Vec<Token> {
    e.insert(0, OpenParen);

    let mut i = 1;
    while i < e.len() {
        match e[i] {
            Mult => {
                e.insert(i, CloseParen);
                e.insert(i + 2, OpenParen);
                i += 2;
            }
            OpenParen => {
                e.insert(i, OpenParen);
                i += 1;
            }
            CloseParen => {
                e.insert(i, CloseParen);
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }

    e.push(CloseParen);

    e
}

fn eval(tokens: &[Token]) -> (N, usize) {
    let mut prev_op = None;

    let (mut result, position) = match tokens[0] {
        Num(x) => (x as N, 1),
        OpenParen => {
            let (r, mut p) = eval(&tokens[1..]);
            p += 2;
            (r, p)
        }
        _ => unreachable!(),
    };

    let mut position = position..tokens.len();
    while let Some(i) = position.next() {
        match tokens[i] {
            Num(x) => {
                result = apply_op(prev_op, result, x as N);
                prev_op = None;
            }
            Add => {
                debug_assert!(prev_op.is_none());
                prev_op = Some(Add);
            }
            Mult => {
                debug_assert!(prev_op.is_none());
                prev_op = Some(Mult);
            }
            OpenParen => {
                let (res, pos) = eval(&tokens[i + 1..]);
                result = apply_op(prev_op, result, res);
                position.nth(pos);
                prev_op = None;
            }
            CloseParen => return (result, i),
        }
    }

    (result, tokens.len())
}

fn apply_op(prev_op: Option<Token>, result: N, x: N) -> N {
    match prev_op {
        Some(Add) => result + x,
        Some(Mult) => result * x,
        _ => unreachable!(),
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
