#![allow(clippy::cast_lossless)]

use std::ops::RangeInclusive;

use serde_scan::scan;

use crate::day_solver::DaySolver;

pub struct Day16;

type N = u16;

#[derive(Clone, Debug)]
pub struct Rule<'a> {
    name: &'a [u8],
    ranges: [RangeInclusive<N>; 2],
}

impl<'a> DaySolver<'a> for Day16 {
    type Parsed = (Vec<Rule<'a>>, Vec<N>, Vec<Vec<N>>);
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

        input.next(); // your ticket:
        let mine = input
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        input.nth(1); // nearby tickets:
        let tickets = input
            .map(|l| l.split(',').map(|x| x.parse().unwrap()).collect())
            .collect();

        (rules, mine, tickets)
    }

    fn part1((rules, _, tickets): Self::Parsed) -> Self::Output {
        let mut sum = 0;

        for t in &tickets {
            if let Some(v) = is_valid_ticket(t, &rules) {
                sum += v;
            }
        }

        sum as u64
    }

    #[allow(clippy::cast_possible_truncation)]
    fn part2((rules, mine, mut tickets): Self::Parsed) -> Self::Output {
        let ticket_length = tickets[0].len() as N;

        tickets.retain(|t| is_valid_ticket(t, &rules).is_none());

        let mut valid_positions = vec![(0..ticket_length).collect::<Vec<_>>(); rules.len()];

        for t in tickets {
            for (vi, v) in t.iter().enumerate() {
                for (ri, r) in rules.iter().enumerate() {
                    if !r.ranges.iter().any(|r2| r2.contains(v)) {
                        remove_item(&mut valid_positions[ri], vi as N);
                    }
                }
            }
        }

        let mut results: Vec<_> = rules.into_iter().zip(valid_positions.into_iter()).collect();
        results.sort_unstable_by_key(|v| v.1.len());

        for i in 0..results.len() {
            let source = &results[i].1;
            debug_assert!(source.len() == 1);
            let v = source[0];

            for j in &mut results[i + 1..] {
                remove_item(&mut j.1, v);
            }
        }

        debug_assert!(results.iter().all(|v| v.1.len() == 1));

        results
            .into_iter()
            .filter_map(|(r, v)| {
                if r.name.starts_with(b"departure") {
                    Some(mine[v[0] as usize] as u64)
                } else {
                    None
                }
            })
            .product()
    }
}

fn is_valid_ticket(t: &[N], rules: &[Rule]) -> Option<N> {
    for &v in t {
        if !rules
            .iter()
            .any(|r| r.ranges.iter().any(|r2| r2.contains(&v)))
        {
            return Some(v);
        }
    }
    None
}

fn remove_item(r: &mut Vec<N>, v: N) {
    if let Some(i) = r.iter().position(|&x| x == v) {
        r.swap_remove(i);
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
        assert_eq!(
            Day16::part2(Day16::parse(
                "class: 0-1 or 4-19
departure row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
            )),
            11
        );
    }
}
