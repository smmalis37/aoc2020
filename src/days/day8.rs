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
        run(&program, &mut vec![false; program.len()]).1
    }

    fn part2(mut program: Self::Parsed) -> Self::Output {
        let mut seen_buf = vec![false; program.len()];

        for change in (0..program.len()).rev() {
            program[change] = match program[change] {
                Acc(_) => continue,
                Jmp(x) => Nop(x),
                Nop(x) => Jmp(x),
            };

            let result = run(&program, &mut seen_buf);
            if result.0 {
                return result.1;
            }

            program[change] = match program[change] {
                Acc(_) => unreachable!(),
                Jmp(x) => Nop(x),
                Nop(x) => Jmp(x),
            };
        }

        unreachable!()
    }
}

fn run<'a>(
    program: &<Day8 as DaySolver>::Parsed,
    seen: &mut [bool],
) -> (bool, <Day8 as DaySolver<'a>>::Output) {
    assert_eq!(program.len(), seen.len());
    seen.iter_mut().for_each(|x| *x = false);
    let mut acc = 0;
    let mut pc = 0;

    loop {
        if pc == program.len() {
            break (true, acc);
        }

        if seen[pc] {
            break (false, acc);
        }
        seen[pc] = true;

        match program[pc] {
            Acc(x) => {
                acc += x;
            }
            Jmp(x) => {
                pc = ((pc as isize) + x - 1) as usize;
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
