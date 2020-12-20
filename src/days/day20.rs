use petgraph::{graph::NodeIndex, Graph, Undirected};
use rustc_hash::FxHashMap;
use serde_scan::scan;

use crate::{day_solver::DaySolver, util::*};

pub struct Day20;

#[derive(Clone)]
pub struct Tile {
    id: u16,
    tile: Grid<u8>,
}

impl DaySolver<'_> for Day20 {
    type Parsed = Graph<Tile, (), Undirected>;
    type Output = u64;

    fn parse(input: &str) -> Self::Parsed {
        let mut map = Graph::new_undirected();
        let mut sides = FxHashMap::default();

        let mut input = input.lines();

        while let Some(l) = input.next() {
            let id = scan!("Tile {}:" <- l).unwrap();

            let nid = map.add_node(Tile {
                id,
                tile: input
                    .clone()
                    .take_while(|x| !x.is_empty())
                    .map(|x| x.as_bytes().iter().copied())
                    .collect(),
            });

            let tile = &map[nid].tile;
            input.nth(tile.len());

            let mut up: Vec<_> = tile[0].into();
            let mut down: Vec<_> = tile[tile.len() - 1].into();
            let mut left: Vec<_> = tile.iter().map(|l| l[0]).collect();
            let mut right: Vec<_> = tile.iter().map(|l| *l.last().unwrap()).collect();

            handle_side(&mut map, &mut sides, up.clone(), nid);
            handle_side(&mut map, &mut sides, down.clone(), nid);
            handle_side(&mut map, &mut sides, left.clone(), nid);
            handle_side(&mut map, &mut sides, right.clone(), nid);

            up.reverse();
            down.reverse();
            left.reverse();
            right.reverse();

            handle_side(&mut map, &mut sides, up, nid);
            handle_side(&mut map, &mut sides, down, nid);
            handle_side(&mut map, &mut sides, left, nid);
            handle_side(&mut map, &mut sides, right, nid);
        }

        map
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        data.node_indices()
            .filter_map(|x| {
                if data.edges(x).count() == 2 {
                    Some(data[x].id as u64)
                } else {
                    None
                }
            })
            .product()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        todo!()
    }
}

fn handle_side(
    map: &mut <Day20 as DaySolver>::Parsed,
    sides: &mut FxHashMap<Vec<u8>, NodeIndex>,
    new: Vec<u8>,
    id: NodeIndex,
) {
    if sides.contains_key(&new) {
        map.update_edge(id, sides.remove(&new).unwrap(), ());
    } else {
        sides.insert(new, id);
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
