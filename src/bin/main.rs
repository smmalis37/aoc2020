use aoc2020::days::*;
use aoc2020::solver::Solver;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::time::Instant;

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
    println!("AOC 2019");
}

fn run<'a, S: Solver<'a>>(
    day_number: u8,
    input: &'a str,
    part1_output: Option<S::Output>,
    part2_output: Option<S::Output>,
) {
    let trimmed_input = input.trim();

    let start_time = Instant::now();
    let generated = S::generator(trimmed_input);
    let final_time = Instant::now();

    println!(
        "\nDay {}:\n\tgenerator : {:?}",
        day_number,
        (final_time - start_time)
    );

    run_half(generated.clone(), 1, S::part1, part1_output);
    run_half(generated, 2, S::part2, part2_output);
}

fn run_half<T, O: Debug + PartialEq>(
    input: T,
    part_number: u8,
    part: impl Fn(T) -> O,
    expected_output: Option<O>,
) {
    print!("Part {}: ", part_number);

    let start_time = Instant::now();
    let result = part(input);
    let final_time = Instant::now();

    println!("{:?}\n\trunner: {:?}", result, (final_time - start_time));

    if let Some(expected) = expected_output {
        assert_eq!(expected, result);
    }
}
