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

#[derive(Copy, Clone)]
pub struct Move {
    direction: Order,
    count: i32,
}

impl DaySolver<'_> for Day12 {
    type Parsed = Vec<Move>;
    type Output = i32;

    fn parse(input: &str) -> Self::Parsed {
        input
            .as_bytes()
            .split(|&x| x == b'\n')
            .map(|l| Move {
                direction: match l[0] {
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
            match (current_direction, m.direction) {
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
            match m.direction {
                Order::North => waypoint_y += m.count,
                Order::South => waypoint_y -= m.count,
                Order::East => waypoint_x += m.count,
                Order::West => waypoint_x -= m.count,
                Order::Left => {
                    for _ in 0..m.count / 90 {
                        let temp = waypoint_x;
                        waypoint_x = -waypoint_y;
                        waypoint_y = temp;
                    }
                }
                Order::Right => {
                    for _ in 0..m.count / 90 {
                        let temp = waypoint_x;
                        waypoint_x = waypoint_y;
                        waypoint_y = -temp;
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
    fn d12p1() {}

    #[test]
    fn d12p2() {}
}
