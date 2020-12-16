#![allow(clippy::cast_lossless)]

use std::{collections::HashMap, ops::RangeInclusive};

use serde_scan::scan;

use crate::day_solver::DaySolver;

pub struct Day16;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Rule<'a> {
    name: &'a [u8],
    ranges: [RangeInclusive<u16>; 2],
}

impl<'a> DaySolver<'a> for Day16 {
    type Parsed = (Vec<Rule<'a>>, Vec<u16>, Vec<Vec<u16>>);
    type Output = u64;

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

        input.next();
        let mine = input
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        input.nth(1);

        let tickets = input
            .map(|l| l.split(',').map(|x| x.parse().unwrap()).collect())
            .collect();

        (rules, mine, tickets)
    }

    fn part1((rules, _, tickets): Self::Parsed) -> Self::Output {
        let mut sum = 0;

        for t in &tickets {
            for &v in t {
                if !rules
                    .iter()
                    .any(|r| r.ranges.iter().any(|r2| r2.contains(&v)))
                {
                    sum += v;
                }
            }
        }

        sum as u64
    }

    #[allow(clippy::cast_possible_truncation)]
    fn part2((rules, mine, mut tickets): Self::Parsed) -> Self::Output {
        let ticket_length = tickets[0].len();

        tickets.retain(|t| {
            t.iter().all(|v| {
                rules
                    .iter()
                    .any(|r| r.ranges.iter().any(|r2| r2.contains(v)))
            })
        });

        let mut answers: HashMap<Rule, Vec<u16>> = rules
            .iter()
            .map(|r| (r.clone(), (0..ticket_length as u16).collect()))
            .collect();

        for t in &tickets {
            for (i, v) in t.iter().enumerate() {
                for r in &rules {
                    if !r.ranges.iter().any(|r2| r2.contains(v)) {
                        let m = answers.get_mut(r).unwrap();
                        if let Some(i2) = m.iter().position(|&x| x == i as u16) {
                            m.swap_remove(i2);
                        }
                    }
                }
            }
        }

        let mut answers = answers.into_iter().collect::<Vec<_>>();
        answers.sort_unstable_by_key(|v| v.1.len());

        for i in 0..answers.len() {
            let source = &answers[i].1;
            assert!(source.len() == 1);
            let r = source[0];
            for j in i + 1..answers.len() {
                let dest = &mut answers[j].1;
                if let Some(i2) = dest.iter().position(|&x| x == r) {
                    dest.swap_remove(i2);
                }
            }
        }

        assert!(answers.iter().all(|v| v.1.len() == 1));

        answers
            .into_iter()
            .filter_map(|r| {
                if r.0.name.starts_with(b"departure") {
                    Some(mine[r.1[0] as usize] as u64)
                } else {
                    None
                }
            })
            .product()
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
}
