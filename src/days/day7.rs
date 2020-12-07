use std::collections::{HashSet, VecDeque};

use bstr_parse::BStrParse;
use petgraph::{graphmap::DiGraphMap, EdgeDirection::*};

use crate::day_solver::DaySolver;

pub struct Day7;

#[derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bag<'a> {
    adjective: &'a [u8],
    color: &'a [u8],
}

impl<'a> DaySolver<'a> for Day7 {
    type Parsed = DiGraphMap<Bag<'a>, usize>;
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut graph = DiGraphMap::new();

        for l in input.as_bytes().split(|&x| x == b'\n') {
            let mut words = l.split(|&x| x == b' ');
            let adjective = words.next().unwrap();
            let color = words.next().unwrap();
            words.next(); // bags
            words.next(); // contain

            let outside = Bag { adjective, color };

            loop {
                let count_word = words.next().unwrap();

                let count = if count_word == b"no" {
                    break;
                } else {
                    count_word.parse().unwrap()
                };

                let adjective = words.next().unwrap();
                let color = words.next().unwrap();
                let inside = Bag { adjective, color };
                graph.add_edge(outside, inside, count);

                let bag_word = words.next().unwrap();
                if bag_word[bag_word.len() - 1] == b'.' {
                    break;
                }
            }
        }

        graph
    }

    fn part1(graph: Self::Parsed) -> Self::Output {
        let start = Bag {
            adjective: b"shiny",
            color: b"gold",
        };
        let mut seen = HashSet::new();

        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(nx) = queue.pop_front() {
            seen.insert(nx);
            for n in graph.neighbors_directed(nx, Incoming) {
                queue.push_back(n);
            }
        }

        seen.len() - 1
    }

    fn part2(graph: Self::Parsed) -> Self::Output {
        let start = Bag {
            adjective: b"shiny",
            color: b"gold",
        };
        let mut count = 0;

        let mut queue = VecDeque::new();
        queue.push_back((start, 1));

        while let Some((nx, factor)) = queue.pop_front() {
            count += factor;
            for (_, n, e) in graph.edges(nx) {
                queue.push_back((n, factor * e));
            }
        }

        count - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d7p1() {}

    #[test]
    fn d7p2() {}
}
