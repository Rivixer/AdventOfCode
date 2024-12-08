use std::collections::{HashMap, HashSet};

use aoc2024::utils::*;

const DAY: u8 = 8;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn is_within_bounds(&self, len: usize) -> bool {
        self.x >= 0 && self.x < len as i32 && self.y >= 0 && self.y < len as i32
    }
}

fn parse_input(data: &str) -> HashMap<u8, Vec<Point>> {
    let mut result: HashMap<u8, Vec<Point>> = HashMap::new();

    data.lines().enumerate().for_each(|(y, line)| {
        line.chars()
            .enumerate()
            .filter_map(|(x, c)| {
                if c == '.' {
                    None
                } else {
                    Some((
                        c as u8,
                        Point {
                            x: x as i32,
                            y: y as i32,
                        },
                    ))
                }
            })
            .for_each(|(key, point)| {
                result.entry(key).or_insert_with(Vec::new).push(point);
            });
    });

    result
}

#[timed::timed]
fn part1(data: &str) -> usize {
    let antennas = parse_input(data);
    let len = data.lines().collect::<Vec<_>>().len();
    let mut antinodes: HashSet<Point> = HashSet::new();

    antennas.values().for_each(|points| {
        points.iter().for_each(|p1| {
            points.iter().filter(|&p2| p1 != p2).for_each(|p2| {
                let dx = p1.x - p2.x;
                let dy = p1.y - p2.y;
                let point = Point::new(p1.x + dx, p1.y + dy);
                if point.is_within_bounds(len) {
                    antinodes.insert(point.clone());
                }
            });
        });
    });

    antinodes.len()
}

#[timed::timed]
fn part2(data: &str) -> usize {
    let antennas = parse_input(data);
    let len = data.lines().collect::<Vec<_>>().len();
    let mut antinodes: HashSet<Point> = HashSet::new();

    antennas.values().for_each(|points| {
        points.iter().for_each(|p1| {
            points.iter().filter(|&p2| p1 != p2).for_each(|p2| {
                if p1.is_within_bounds(len) {
                    antinodes.insert(p1.clone());
                }

                let dx = p1.x - p2.x;
                let dy = p1.y - p2.y;
                let mut point = Point::new(p1.x + dx, p1.y + dy);

                while point.is_within_bounds(len) {
                    antinodes.insert(point.clone());
                    point.x += dx;
                    point.y += dy;
                }
            });
        });
    });

    antinodes.len()
}

fn main() {
    let input = read_input(DAY, InputType::Production);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input().part1), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input().part2), 34);
    }

    fn get_test_input() -> Input {
        read_input(DAY, InputType::Test)
    }
}
