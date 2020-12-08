use bstr_parse::BStrParse;

use crate::day_solver::DaySolver;

pub struct Day8;

#[derive(Clone)]
pub enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

use Instruction::*;

impl DaySolver<'_> for Day8 {
    type Parsed = Vec<Instruction>;
    type Output = isize;

    fn parse(input: &str) -> Self::Parsed {
        let mut results = Vec::new();
        for l in input.as_bytes().split(|&x| x == b'\n') {
            let num = l[4..].parse().unwrap();
            results.push(match &l[0..3] {
                b"acc" => Acc(num),
                b"jmp" => Jmp(num),
                b"nop" => Nop(num),
                _ => unreachable!(),
            });
        }
        results
    }

    fn part1(program: Self::Parsed) -> Self::Output {
        let mut seen = vec![false; program.len()];
        let mut acc = 0;
        let mut pc = 0;

        loop {
            if seen[pc as usize] {
                break acc;
            }

            seen[pc as usize] = true;
            match program[pc as usize] {
                Acc(x) => {
                    acc += x;
                }
                Jmp(x) => {
                    pc += x - 1;
                }
                Nop(_) => {}
            }

            pc += 1;
        }
    }

    fn part2(source: Self::Parsed) -> Self::Output {
        for change in (0..source.len()).rev() {
            let mut test = source.clone();
            test[change] = match test[change] {
                Acc(_) => continue,
                Jmp(x) => Nop(x),
                Nop(x) => Jmp(x),
            };

            let mut acc = 0;
            let mut pc = 0;

            let mut seen = vec![false; test.len()];
            loop {
                if pc as usize == test.len() {
                    return acc;
                }
                if seen[pc as usize] {
                    break;
                }

                seen[pc as usize] = true;
                match test[pc as usize] {
                    Acc(x) => {
                        acc += x;
                    }
                    Jmp(x) => {
                        pc += x - 1;
                    }
                    Nop(_) => {}
                }

                pc += 1;
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d8p1() {
        assert_eq!(
            Day8::part1(Day8::parse(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            )),
            5
        );
    }

    #[test]
    fn d8p2() {
        assert_eq!(
            Day8::part2(Day8::parse(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            )),
            8
        );
    }
}
