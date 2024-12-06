use std::{collections::HashSet, ops::Index};

use aoc2024::utils::*;

const DAY: u8 = 6;

struct Grid {
    data: Vec<Vec<char>>,
}

impl Index<&Point> for Grid {
    type Output = char;

    fn index(&self, point: &Point) -> &Self::Output {
        &self.data[point.y as usize][point.x as usize]
    }
}

impl Grid {
    fn get_start_point(&self) -> Point {
        for (i, row) in self.data.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == '^' {
                    return Point::new(j as isize, i as isize);
                }
            }
        }
        panic!();
    }

    fn is_point_inside(&self, point: &Point) -> bool {
        point.x >= 0
            && point.x < self.data[0].len() as isize
            && point.y >= 0
            && point.y < self.data.len() as isize
    }

    fn from_input(input: &str) -> Self {
        Self {
            data: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn next(&self, direction: &Direction) -> Self {
        Self {
            x: self.x + direction.x,
            y: self.y + direction.y,
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Direction {
    x: isize,
    y: isize,
}

fn find_route(
    grid: &Grid,
    is_looped: &mut bool,
    additional_obstacle: Option<Point>,
) -> HashSet<(Point, usize)> {
    let start = grid.get_start_point();
    const DIRECTIONS: [Direction; 4] = [
        Direction { x: 0, y: -1 },
        Direction { x: 1, y: 0 },
        Direction { x: 0, y: 1 },
        Direction { x: -1, y: 0 },
    ];

    let mut position = start;
    let mut direction = 0;
    let mut visited = HashSet::with_capacity(grid.data.len() * grid.data[0].len());

    loop {
        if visited.contains(&(position, direction)) {
            *is_looped = true;
            break;
        }

        let next = position.next(&DIRECTIONS[direction]);
        visited.insert((position, direction));

        if !grid.is_point_inside(&next) {
            break;
        }

        if grid[&next] == '#' || additional_obstacle == Some(next) {
            direction += 1;
            direction %= 4;
        } else {
            position = next;
        }
    }

    visited
}

#[timed::timed]
fn part1(data: &str) -> usize {
    let grid = Grid::from_input(data);
    let mut is_looped = false;
    let visited = find_route(&grid, &mut is_looped, None);
    visited
        .iter()
        .map(|(point, _)| point)
        .collect::<HashSet<_>>()
        .len()
}

#[timed::timed]
fn part2(data: &str) -> usize {
    let grid = Grid::from_input(data);
    let mut is_looped = false;
    let visited = find_route(&grid, &mut is_looped, None);
    let visited_points = visited
        .iter()
        .map(|(point, _)| *point)
        .collect::<HashSet<_>>();

    visited_points
        .iter()
        .filter(|&point| grid[point] == '.')
        .filter(|&&point| {
            let mut is_looped = false;
            find_route(&grid, &mut is_looped, Some(point));
            is_looped
        })
        .count()
}

fn main() {
    let input = read_input(DAY, InputType::Production);
    println!("Part 1: {}", part1(&input.part1));
    println!("Part 2: {}", part2(&input.part2));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Input {
        read_input(DAY, InputType::Test)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&get_test_input().part1), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input().part2), 6);
    }
}
