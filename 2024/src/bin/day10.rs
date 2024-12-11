use aoc2024::utils::*;
use std::collections::{HashMap, HashSet, VecDeque};

const DAY: u8 = 10;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn neighbors(&self) -> Vec<Position> {
        vec![
            Position::new(self.x + 1, self.y),
            Position::new(self.x - 1, self.y),
            Position::new(self.x, self.y + 1),
            Position::new(self.x, self.y - 1),
        ]
    }
}

fn parse_input(data: &str) -> (HashMap<Position, i32>, Vec<Position>) {
    let grid = data
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, char)| {
                let position = Position::new(x as i32, y as i32);
                let value = char.to_digit(10).unwrap() as i32;
                (position, value)
            })
        })
        .collect::<HashMap<_, _>>();

    let trailheads = grid
        .iter()
        .filter_map(|(&position, &value)| if value == 0 { Some(position) } else { None })
        .collect();

    (grid, trailheads)
}

fn calculate_score(start: &Position, grid: &HashMap<Position, i32>, avoid_visited: bool) -> i32 {
    let mut result = 0;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(*start);

    while let Some(position) = queue.pop_front() {
        if !avoid_visited {
            if visited.contains(&position) {
                continue;
            }

            visited.insert(position);
        }

        let elevation = grid[&position];
        if elevation == 9 {
            result += 1;
            continue;
        }

        position
            .neighbors()
            .into_iter()
            .filter_map(|neighbor| {
                grid.get(&neighbor)
                    .map(|&neighbor_elevation| (neighbor, neighbor_elevation))
            })
            .filter(|&(_, neighbor_elevation)| neighbor_elevation == elevation + 1)
            .for_each(|(neighbor, _)| queue.push_back(neighbor));
    }

    result
}

#[timed::timed]
fn part1(data: &str) -> u64 {
    let (grid, trailheads) = parse_input(data);
    trailheads
        .iter()
        .map(|start| calculate_score(&start, &grid, false) as u64)
        .sum()
}

#[timed::timed]
fn part2(data: &str) -> u64 {
    let (grid, trailheads) = parse_input(data);
    trailheads
        .iter()
        .map(|start| calculate_score(&start, &grid, true) as u64)
        .sum()
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
        assert_eq!(part1(&get_test_input().part1), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&get_test_input().part2), 81);
    }

    fn get_test_input() -> Input {
        read_input(DAY, InputType::Test)
    }
}
