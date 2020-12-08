#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

use bstr_parse::BStrParse;

use crate::day_solver::DaySolver;

pub struct Day8;

type N = i16;

#[derive(Clone)]
pub enum Instruction {
    Acc(N),
    Jmp(N),
    Nop(N),
}

use Instruction::*;

impl DaySolver<'_> for Day8 {
    type Parsed = Vec<Instruction>;
    type Output = N;

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
        run(&program).1
    }

    fn part2(mut program: Self::Parsed) -> Self::Output {
        let trace = run(&program).2;

        let mut potential_landing_spots = vec![false; program.len() + 1];
        let mut i = program.len();
        loop {
            potential_landing_spots[i] = true;
            i -= 1;

            if let Jmp(x) = program[i] {
                if x < 0 {
                    break;
                }
            }
        }

        let swap = if trace[i] {
            //println!("Found last negative jmp.");
            i
        } else {
            loop {
                i -= 1;

                if let Nop(x) = program[i] {
                    if trace[i] && potential_landing_spots[((i as N) + x) as usize] {
                        //println!("Found nop to jmp.");
                        break i;
                    }
                } else if let Jmp(x) = program[i] {
                    if !trace[i]
                        && potential_landing_spots[((i as N) + x) as usize]
                        && !potential_landing_spots[i]
                    {
                        let mut j = i - 1;
                        loop {
                            if matches!(program[j], Jmp(_)) {
                                break;
                            }
                            j -= 1;
                        }

                        if trace[j] {
                            //println!("Found jmp preceded by hit jmp.");
                            break j;
                        } else {
                            //println!("Found jmp preceded by unhit jmp.");
                            potential_landing_spots[j + 1..=i].iter_mut().for_each(|a| {
                                *a = true;
                            });
                            i = program.len();
                        }
                    }
                }
            }
        };

        program[swap] = match program[swap] {
            Acc(_) => unreachable!(),
            Jmp(x) => Nop(x),
            Nop(x) => Jmp(x),
        };
        let res = run(&program);
        assert!(res.0);
        res.1
    }
}

fn run<'a>(
    program: &<Day8 as DaySolver>::Parsed,
) -> (bool, <Day8 as DaySolver<'a>>::Output, Vec<bool>) {
    let mut seen = vec![false; program.len()];
    let mut acc = 0;
    let mut pc = 0;

    loop {
        if pc == program.len() {
            break (true, acc, seen);
        }

        if seen[pc] {
            break (false, acc, seen);
        }
        seen[pc] = true;

        match program[pc] {
            Acc(x) => {
                acc += x;
            }
            Jmp(x) => {
                pc = ((pc as N) + x - 1) as usize;
            }
            Nop(_) => {}
        }

        pc += 1;
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
