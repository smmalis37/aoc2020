use aoc2020::{day_solver::DaySolver, days::*};
use std::{cmp::PartialEq, fmt::Debug, time::Instant};

macro_rules! day {
    ( $d:expr ) => {
        day!($d => None, None);
    };

    ( $d:expr, $o1:expr ) => {
        day!($d => Some($o1), None);
    };

    ( $d:expr, $o1:expr, $o2:expr ) => {
        day!($d => Some($o1), Some($o2));
    };

    ( $d:expr => $o1:expr, $o2:expr ) => {
        paste::expr! {
            run::<[<day $d>]::[<Day $d>]>($d, include_str!(concat!("../../input/2020/day", $d, ".txt")), $o1, $o2);
        }
    };
}

fn main() {
    println!("AOC 2020");
    day!(1, 964875, 158661360);
    day!(2, 515, 711);
    day!(3, 292, 9354744432);
}

fn run<'a, S: DaySolver<'a>>(
    day_number: u8,
    input: &'a str,
    part1_output: Option<S::Output>,
    part2_output: Option<S::Output>,
) {
    let trimmed_input = input.trim();

    let start_time = Instant::now();
    let parsed = S::parse(trimmed_input);
    let end_time = Instant::now();

    println!("\nDay {}:", day_number);
    println!("\tparser: {:?}", (end_time - start_time));

    run_part(parsed.clone(), 1, S::part1, part1_output);
    run_part(parsed, 2, S::part2, part2_output);
}

fn run_part<P, O: Debug + PartialEq>(
    parsed: P,
    part_number: u8,
    part: impl Fn(P) -> O,
    expected_output: Option<O>,
) {
    print!("Part {}: ", part_number);

    let start_time = Instant::now();
    let result = part(parsed);
    let end_time = Instant::now();

    println!("{:?}", result);
    println!("\tsolver: {:?}", (end_time - start_time));

    if let Some(expected) = expected_output {
        assert_eq!(expected, result);
    }
}
