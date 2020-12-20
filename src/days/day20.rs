use rustc_hash::{FxHashMap, FxHashSet};
use serde_scan::scan;

use crate::{day_solver::DaySolver, util::*};

pub struct Day20;

#[derive(Clone)]
pub struct Tile {
    id: u64,
    tile: Grid<bool>,
}

impl DaySolver<'_> for Day20 {
    type Parsed = Vec<Tile>;
    type Output = u64;

    fn parse(input: &str) -> Self::Parsed {
        let mut input = input.lines();
        let mut results = Vec::new();

        while let Some(l) = input.next() {
            let id = scan!("Tile {}:" <- l).unwrap();
            let t = Tile {
                id,
                tile: input
                    .clone()
                    .take_while(|x| !x.is_empty())
                    .map(|x| x.as_bytes().iter().map(|&c| c == b'#'))
                    .collect(),
            };
            input.nth(t.tile.len());
            results.push(t);
        }

        results
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut edges = FxHashMap::default();

        for t in data {
            let mut up = t.tile[0].iter().copied().collect::<Vec<_>>();
            let mut down = t.tile[t.tile.len() - 1].iter().copied().collect::<Vec<_>>();
            let mut left = t.tile.iter().map(|l| l[0]).collect::<Vec<_>>();
            let mut right = t
                .tile
                .iter()
                .map(|l| *l.last().unwrap())
                .collect::<Vec<_>>();

            handle_edge(&mut edges, up.clone(), t.id);
            handle_edge(&mut edges, down.clone(), t.id);
            handle_edge(&mut edges, left.clone(), t.id);
            handle_edge(&mut edges, right.clone(), t.id);

            up.reverse();
            down.reverse();
            left.reverse();
            right.reverse();

            handle_edge(&mut edges, up, t.id);
            handle_edge(&mut edges, down, t.id);
            handle_edge(&mut edges, left, t.id);
            handle_edge(&mut edges, right, t.id);
        }

        let mut countmap: FxHashMap<u64, u64> = FxHashMap::default();
        for &e in edges.values() {
            *countmap.entry(e).or_default() += 1;
        }

        countmap
            .into_iter()
            .filter(|&(_, v)| v == 4)
            .map(|(k, _)| k)
            .product()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        todo!()
    }
}

fn handle_edge(map: &mut FxHashMap<Vec<bool>, u64>, new: Vec<bool>, id: u64) {
    if map.contains_key(&new) {
        map.remove(&new).unwrap();
    } else {
        map.insert(new, id);
    }
}

#[cfg(test)]
#[allow(clippy::too_many_lines)]
mod tests {
    use super::*;

    #[test]
    fn d20p1() {
        assert_eq!(
            Day20::part1(Day20::parse(
                "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."
            )),
            20899048083289
        );
    }

    #[test]
    fn d20p2() {
        assert_eq!(
            Day20::part2(Day20::parse(
                "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."
            )),
            273
        );
    }
}
