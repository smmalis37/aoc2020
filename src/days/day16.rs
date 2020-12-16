use std::ops::RangeInclusive;

use serde_scan::scan;

use crate::{day_solver::DaySolver, util::*};

pub struct Day16;

#[derive(Clone)]
pub struct Rule<'a> {
    name: &'a [u8],
    ranges: [RangeInclusive<u16>; 2],
}

impl<'a> DaySolver<'a> for Day16 {
    type Parsed = (Vec<Rule<'a>>, Grid<u16>);
    type Output = u16;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut input = input.lines();
        let mut rules = Vec::new();

        while let Some(l) = input.next() {
            if l.is_empty() {
                break;
            }

            let (name, min1, max1, min2, max2) = scan!("{}: {}-{} or {}-{}" <- l).unwrap();
            rules.push(Rule {
                name,
                ranges: [min1..=max1, min2..=max2],
            });
        }

        input.nth(3); // skip my ticket for now

        let tickets = input
            .map(|l| l.split(',').map(|x| x.parse().unwrap()))
            .collect();

        (rules, tickets)
    }

    fn part1((rules, tickets): Self::Parsed) -> Self::Output {
        let mut sum = 0;

        for t in tickets.iter() {
            for &v in t {
                if !rules
                    .iter()
                    .any(|r| r.ranges.iter().any(|r2| r2.contains(&v)))
                {
                    sum += v;
                }
            }
        }

        sum
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d16p1() {
        assert_eq!(
            Day16::part1(Day16::parse(
                "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            )),
            71
        );
    }

    #[test]
    fn d16p2() {
        assert_eq!(Day16::part2(Day16::parse("")), 0);
    }
}
