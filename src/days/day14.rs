use rustc_hash::FxHashMap;

use crate::{day_solver::DaySolver, util::*};

pub struct Day14;

#[derive(Copy, Clone)]
pub enum Op {
    Mask(u64, u64, u64),
    Assign(u64, u64),
}

use Op::*;

impl DaySolver<'_> for Day14 {
    type Parsed = (Vec<Op>, usize);
    type Output = u64;

    fn parse(input: &str) -> Self::Parsed {
        let mut x_count = 0;
        let mut assign_count = 0;
        let mut mask_count = 0;
        let res = input
            .as_bytes()
            .split(|&x| x == b'\n')
            .map(|line| match &line[0..4] {
                b"mask" => {
                    mask_count += 1;
                    let mut or_mask = 0;
                    let mut and_mask = u64::MAX;
                    let mut x_mask = 0;

                    for bit in &line[7..] {
                        match bit {
                            b'0' => {
                                or_mask <<= 1;
                                and_mask <<= 1;
                                x_mask <<= 1;
                            }
                            b'1' => {
                                or_mask = (or_mask << 1) + 1;
                                and_mask = (and_mask << 1) + 1;
                                x_mask <<= 1;
                            }
                            b'X' => {
                                or_mask <<= 1;
                                and_mask = (and_mask << 1) + 1;
                                x_mask = (x_mask << 1) + 1;
                                x_count += 1;
                            }
                            _ => unreachable!(),
                        }
                    }
                    Mask(or_mask, and_mask, x_mask)
                }
                b"mem[" => {
                    assign_count += 1;
                    let close_bracket = line.iter().position(|&x| x == b']').unwrap();
                    let index = line[4..close_bracket].parse().unwrap();
                    let num = line[close_bracket + 4..].parse().unwrap();
                    Assign(index, num)
                }
                _ => unreachable!(),
            })
            .collect();
        let part2_size = assign_count * 2_usize.pow((x_count / mask_count) + 1);
        (res, part2_size)
    }

    fn part1((data, _): Self::Parsed) -> Self::Output {
        let mut current_or_mask = 0;
        let mut current_and_mask = u64::MAX;
        let mut memory = FxHashMap::with_capacity_and_hasher(data.len(), Default::default());

        for x in data {
            match x {
                Mask(or_mask, and_mask, _) => {
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

    fn part2((data, size): Self::Parsed) -> Self::Output {
        let mut current_or_mask = 0;
        let mut current_x_mask = 0;
        let mut memory = FxHashMap::with_capacity_and_hasher(size, Default::default());

        for x in data {
            match x {
                Mask(or_mask, _, x_mask) => {
                    current_or_mask = or_mask;
                    current_x_mask = x_mask;
                }
                Assign(index, num) => {
                    let index = index | current_or_mask;
                    let mut xor = current_x_mask + 1;

                    for _ in 0..2_u16.pow(current_x_mask.count_ones()) {
                        xor = (xor - 1) & current_x_mask;
                        *memory.entry(index ^ xor).or_default() = num;
                    }
                }
            }
        }

        memory.values().sum()
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
    fn d14p2() {
        assert_eq!(
            Day14::part2(Day14::parse(
                "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            )),
            208
        );
    }
}
