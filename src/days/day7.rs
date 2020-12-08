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

const MY_BAG: Bag<'static> = Bag {
    adjective: b"shiny",
    color: b"gold",
};

impl<'a> DaySolver<'a> for Day7 {
    type Parsed = DiGraphMap<Bag<'a>, Self::Output>;
    type Output = u16;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut graph = Self::Parsed::new();

        let mut words = input.as_bytes().split(|&x| x == b'\n' || x == b' ');
        while let Some(adjective) = words.next() {
            let color = words.next().unwrap();
            words.next(); // bags
            words.next(); // contain

            let outside = Bag { adjective, color };

            loop {
                let count_word = words.next().unwrap();

                let count = if count_word == b"no" {
                    words.next(); // other
                    words.next(); // bags
                    break;
                } else {
                    count_word.parse().unwrap()
                };

                let adjective = words.next().unwrap();
                let color = words.next().unwrap();
                let inside = Bag { adjective, color };
                graph.add_edge(outside, inside, count);

                // bags[,.]
                if *words.next().unwrap().last().unwrap() == b'.' {
                    break;
                }
            }
        }

        graph
    }

    fn part1(graph: Self::Parsed) -> Self::Output {
        let mut queue = VecDeque::new();
        queue.push_back(MY_BAG);

        let mut seen = HashSet::new();

        while let Some(nx) = queue.pop_front() {
            if seen.insert(nx) {
                for n in graph.neighbors_directed(nx, Incoming) {
                    queue.push_back(n);
                }
            }
        }

        seen.len() as Self::Output - 1
    }

    fn part2(graph: Self::Parsed) -> Self::Output {
        let mut queue = VecDeque::new();
        queue.push_back((MY_BAG, 1));

        let mut count = 0;

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
    fn d7p1() {
        assert_eq!(
            Day7::part1(Day7::parse(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            )),
            4
        );
    }

    #[test]
    fn d7p2() {
        assert_eq!(
            Day7::part2(Day7::parse(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            )),
            32
        );

        assert_eq!(
            Day7::part2(Day7::parse(
                "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
            )),
            126
        );
    }
}
