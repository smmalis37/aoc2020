use aoc2020::{day_solver::DaySolver, days::*};
use std::{cmp::PartialEq, fmt::Debug, time::Instant};

#[cfg(debug_assertions)]
#[global_allocator]
static ALLOCATOR: dhat::DhatAlloc = dhat::DhatAlloc;

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
            solve::<_, [<day $d>]::[<Day $d>]>($d, $o1, $o2);
        }
    };
}

fn main() {
    #[cfg(debug_assertions)]
    let _dhat = dhat::Dhat::start_heap_profiling();

    println!("AOC 2020");
    day!(1, 964875, 158661360);
    day!(2, 515, 711);
    day!(3, 292, 9354744432);
    day!(4, 247, 145);
    day!(5, 864, 739);
    day!(6, 6590, 3288);
    day!(7, 287, 48160);
    day!(8, 1528, 640);
    day!(9, 29221323, 4389369);
}

fn solve<O, S: for<'a> DaySolver<'a, Output = O>>(
    day_number: u8,
    part1_output: Option<O>,
    part2_output: Option<O>,
) {
    let input = std::fs::read_to_string(format!("input/2020/day{}.txt", day_number)).unwrap();
    let trimmed = input.trim();

    let mut args = std::env::args();
    if args.len() > 1 {
        if args.any(|x| x == day_number.to_string() || x == "a") {
            bench::<S>(day_number, &trimmed);
        }
    } else {
        run::<S>(day_number, &trimmed, part1_output, part2_output);
    }
}

fn run<'a, S: DaySolver<'a>>(
    day_number: u8,
    input: &'a str,
    part1_output: Option<S::Output>,
    part2_output: Option<S::Output>,
) {
    let start_time = Instant::now();
    let parsed = S::parse(input);
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
    } else {
        println!("Not checking input!");
    }
}

fn bench<'a, S: DaySolver<'a>>(day_number: u8, input: &'a str) {
    let mut criterion = criterion::Criterion::default().without_plots();
    let mut group = criterion.benchmark_group(format!("Day {}", day_number));

    group.bench_with_input("parser", &input, |b, i| {
        b.iter_with_large_drop(|| S::parse(i));
    });

    let parsed = S::parse(input);

    group.bench_with_input("part 1", &parsed, |b, i| {
        b.iter_batched(|| i.clone(), S::part1, criterion::BatchSize::SmallInput)
    });

    group.bench_with_input("part 2", &parsed, |b, i| {
        b.iter_batched(|| i.clone(), S::part2, criterion::BatchSize::SmallInput)
    });
}
