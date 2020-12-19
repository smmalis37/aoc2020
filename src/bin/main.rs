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
    day!(1, 964_875, 158_661_360);
    day!(2, 515, 711);
    day!(3, 292, 9_354_744_432);
    day!(4, 247, 145);
    day!(5, 864, 739);
    day!(6, 6_590, 3_288);
    day!(7, 287, 48_160);
    day!(8, 1_528, 640);
    day!(9, 29_221_323, 4_389_369);
    day!(10, 2_240, 99_214_346_656_768);
    day!(11, 2_319, 2_117);
    day!(12, 820, 66_614);
    day!(13, 3_464, 760_171_380_521_445);
    day!(14, 18_630_548_206_046, 4_254_673_508_445);
    //day!(15, 203, 9_007_186);
    day!(16, 19_093, 5_311_123_569_883);
    day!(17, 346, 1_632);
    day!(18, 1_451_467_526_514, 224_973_686_321_527);
    day!(19, 265);
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
