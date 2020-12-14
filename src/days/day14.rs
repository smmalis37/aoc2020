use std::collections::HashMap;

use crate::{day_solver::DaySolver, util::*};

pub struct Day14;

#[derive(Copy, Clone)]
pub enum Op {
    Mask(u64, u64),
    Assign(usize, u64),
}

use Op::*;

impl DaySolver<'_> for Day14 {
    type Parsed = Vec<Op>;
    type Output = u64;

    fn parse(input: &str) -> Self::Parsed {
        input
            .as_bytes()
            .split(|&x| x == b'\n')
            .map(|line| match &line[0..4] {
                b"mask" => {
                    let mut or_mask = 0;
                    let mut and_mask = u64::MAX;

                    for bit in &line[7..] {
                        match bit {
                            b'0' => {
                                or_mask = or_mask << 1;
                                and_mask = and_mask << 1;
                            }
                            b'1' => {
                                or_mask = (or_mask << 1) + 1;
                                and_mask = (and_mask << 1) + 1;
                            }
                            b'X' => {
                                or_mask = or_mask << 1;
                                and_mask = (and_mask << 1) + 1;
                            }
                            _ => unreachable!(),
                        }
                    }
                    Mask(or_mask, and_mask)
                }
                b"mem[" => {
                    let close_bracket = line.iter().position(|&x| x == b']').unwrap();
                    let index = line[4..close_bracket].parse().unwrap();
                    let num = line[close_bracket + 4..].parse().unwrap();
                    Assign(index, num)
                }
                _ => unreachable!(),
            })
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut current_or_mask = 0;
        let mut current_and_mask = u64::MAX;
        let mut memory = HashMap::new();

        for x in data {
            match x {
                Mask(or_mask, and_mask) => {
                    current_or_mask = or_mask;
                    current_and_mask = and_mask;
                }
                Assign(index, num) => {
                    *memory.entry(index).or_default() = (num & current_and_mask) | current_or_mask;
                }
            }
        }

        memory.values().sum()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d14p1() {
        assert_eq!(
            Day14::part1(Day14::parse(
                "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            )),
            165
        );
    }

    #[test]
    fn d14p2() {}
}
