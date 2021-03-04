use std::collections::VecDeque;

use rustc_hash::{FxHashMap, FxHashSet};

use petgraph::{graph::NodeIndex, visit::EdgeRef, EdgeDirection::*, Graph};

use crate::{day_solver::DaySolver, util::*};

pub struct Day7;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Bag<'a> {
    adjective: &'a [u8],
    color: &'a [u8],
}

const MY_BAG: Bag<'static> = Bag {
    adjective: b"shiny",
    color: b"gold",
};

impl<'a> DaySolver<'a> for Day7 {
    type Parsed = (Graph<Bag<'a>, u16>, NodeIndex);
    type Output = usize;

    fn parse(input: &'a str) -> Self::Parsed {
        let mut graph = Graph::new();
        let mut indexes = FxHashMap::default();
        let mut my_bag_index = NodeIndex::end();

        let mut words = input.as_bytes().split(|&x| x == b'\n' || x == b' ');

        while let Some(adjective) = words.next() {
            let color = words.next().unwrap();

            let outside_bag = Bag { adjective, color };
            let outside_index = *indexes
                .entry(outside_bag)
                .or_insert_with(|| graph.add_node(outside_bag));

            if outside_bag == MY_BAG {
                my_bag_index = outside_index;
            }

            // skip "bags contain"
            words.nth(1);

            loop {
                let count_word = words.next().unwrap();

                let count = if count_word == b"no" {
                    words.nth(1); // skip "other bags."
                    break;
                } else {
                    count_word.parse().unwrap()
                };

                let inner_adjective = words.next().unwrap();
                let inner_color = words.next().unwrap();
                let inner_bag = Bag {
                    adjective: inner_adjective,
                    color: inner_color,
                };
                let inner_index = *indexes
                    .entry(inner_bag)
                    .or_insert_with(|| graph.add_node(inner_bag));

                graph.add_edge(outside_index, inner_index, count);

                // bags[,.]
                if *words.next().unwrap().last().unwrap() == b'.' {
                    break;
                }
            }
        }

        (graph, my_bag_index)
    }

    fn part1((graph, my_bag): Self::Parsed) -> Self::Output {
        let mut queue = VecDeque::with_capacity(graph.node_count());
        queue.push_back(my_bag);

        let mut seen = FxHashSet::with_capacity_and_hasher(graph.node_count(), Default::default());

        while let Some(nx) = queue.pop_front() {
            if seen.insert(nx) {
                for n in graph.neighbors_directed(nx, Incoming) {
                    queue.push_back(n);
                }
            }
        }

        seen.len() - 1
    }

    fn part2((graph, my_bag): Self::Parsed) -> Self::Output {
        let mut queue = VecDeque::with_capacity(graph.edge_count());
        queue.push_back((my_bag, 1));

        let mut count = 0;

        while let Some((nx, factor)) = queue.pop_front() {
            count += factor;
            for er in graph.edges(nx) {
                queue.push_back((er.target(), factor * er.weight()));
            }
        }

        count as Self::Output - 1
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
