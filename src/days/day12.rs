use std::mem::swap;

use crate::{day_solver::DaySolver, util::*};

pub struct Day12;

#[derive(Copy, Clone)]
pub enum Order {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

type N = i32;

#[derive(Copy, Clone)]
pub struct Move {
    order: Order,
    count: N,
}

impl DaySolver<'_> for Day12 {
    type Parsed = Vec<Move>;
    type Output = N;

    fn parse(input: &str) -> Self::Parsed {
        input
            .as_bytes()
            .split(|&x| x == b'\n')
            .map(|l| Move {
                order: match l[0] {
                    b'N' => Order::North,
                    b'S' => Order::South,
                    b'E' => Order::East,
                    b'W' => Order::West,
                    b'F' => Order::Forward,
                    b'L' => Order::Left,
                    b'R' => Order::Right,
                    _ => unreachable!(),
                },
                count: l[1..].parse().unwrap(),
            })
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output {
        let mut current_direction = Direction::East;
        let mut x = 0;
        let mut y = 0;

        for m in data {
            match (current_direction, m.order) {
                (Direction::North, Order::Forward) | (_, Order::North) => y += m.count,
                (Direction::South, Order::Forward) | (_, Order::South) => y -= m.count,
                (Direction::East, Order::Forward) | (_, Order::East) => x += m.count,
                (Direction::West, Order::Forward) | (_, Order::West) => x -= m.count,
                (_, Order::Left) => {
                    for _ in 0..m.count / 90 {
                        current_direction = match current_direction {
                            Direction::North => Direction::West,
                            Direction::South => Direction::East,
                            Direction::East => Direction::North,
                            Direction::West => Direction::South,
                        }
                    }
                }
                (_, Order::Right) => {
                    for _ in 0..m.count / 90 {
                        current_direction = match current_direction {
                            Direction::North => Direction::East,
                            Direction::South => Direction::West,
                            Direction::East => Direction::South,
                            Direction::West => Direction::North,
                        }
                    }
                }
            }
        }

        x.abs() + y.abs()
    }

    fn part2(data: Self::Parsed) -> Self::Output {
        let mut x = 0;
        let mut y = 0;
        let mut waypoint_x = 10;
        let mut waypoint_y = 1;

        for m in data {
            match m.order {
                Order::North => waypoint_y += m.count,
                Order::South => waypoint_y -= m.count,
                Order::East => waypoint_x += m.count,
                Order::West => waypoint_x -= m.count,
                Order::Left => {
                    for _ in 0..m.count / 90 {
                        swap(&mut waypoint_x, &mut waypoint_y);
                        waypoint_x *= -1;
                    }
                }
                Order::Right => {
                    for _ in 0..m.count / 90 {
                        swap(&mut waypoint_x, &mut waypoint_y);
                        waypoint_y *= -1;
                    }
                }
                Order::Forward => {
                    x += waypoint_x * m.count;
                    y += waypoint_y * m.count;
                }
            }
        }

        x.abs() + y.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d12p1() {
        assert_eq!(
            Day12::part1(Day12::parse(
                "F10
N3
F7
R90
F11"
            )),
            25
        );
    }

    #[test]
    fn d12p2() {
        assert_eq!(
            Day12::part2(Day12::parse(
                "F10
N3
F7
R90
F11"
            )),
            286
        );
    }
}
